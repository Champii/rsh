use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::config::OutputStreamType;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::{
    At, Cmd, ColorMode, CompletionType, Config as RConfig, Context, EditMode, Editor, KeyPress,
    Movement, Word,
};
use rustyline_derive::Helper;
use std::borrow::Cow::{self, Borrowed, Owned};
use std::ops::Try;
use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, RwLock},
};

use super::config::Config;
use super::error::Error;

#[derive(Helper)]
struct EditorHelper {
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl Completer for EditorHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for EditorHelper {
    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for EditorHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(format!("\x1b[2;37m{}\x1b[m", hint))
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

lazy_static! {
    static ref SUBSTITUTIONS: Arc<RwLock<HashMap<String, String>>> = {
        let mut map = HashMap::new();

        map.insert("%u".to_string(), "$USER".to_string());
        map.insert("%h".to_string(), r"`hostname`".to_string());
        map.insert("%d".to_string(), "$PWD".to_string());
        map.insert("%%".to_string(), "%".to_string());

        Arc::new(RwLock::new(map))
    };
}

pub struct Input {
    editor: Editor<EditorHelper>,
    // config: Config,
    script: Option<Vec<String>>,
    script_idx: usize,
}

impl Input {
    pub fn new(config: Config) -> Self {
        let rusty_config = RConfig::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Emacs)
            .color_mode(ColorMode::Enabled)
            .output_stream(OutputStreamType::Stdout)
            .build();

        let h = EditorHelper {
            completer: FilenameCompleter::new(),
            highlighter: MatchingBracketHighlighter::new(),
            hinter: HistoryHinter {},
            colored_prompt: "".to_owned(),
        };

        let mut editor = Editor::with_config(rusty_config);

        editor.bind_sequence(
            KeyPress::ControlLeft,
            Cmd::Move(Movement::BackwardWord(1, Word::Big)),
        );

        editor.bind_sequence(
            KeyPress::ControlRight,
            Cmd::Move(Movement::ForwardWord(1, At::Start, Word::Big)),
        );

        editor.set_helper(Some(h));

        let script = if let Some(input) = &config.input {
            Some(input.split('\n').map(|y| y.to_string()).collect::<Vec<_>>())
        } else {
            config.script_path.clone().map(|x| {
                match std::fs::read_to_string(Path::new(&x)) {
                    Ok(s) => s,
                    Err(_) => String::new(),
                }
                .split('\n')
                .map(|y| y.to_string())
                .collect::<Vec<_>>()
            })
        };

        Self {
            editor,
            // config,
            script,
            script_idx: 0,
        }
    }

    pub fn init(&mut self) -> Result<(), Error> {
        match self.editor.load_history(&format!(
            "{}/.rsh_history",
            dirs::home_dir().unwrap().to_str().unwrap()
        )) {
            Ok(_) => Ok(()),
            Err(_) => Ok(()),
        }
    }

    fn substitute_prompt(prompt: &mut String) -> Result<(), Error> {
        let substitutions = match SUBSTITUTIONS.read() {
            Ok(substitutions) => substitutions,
            Err(_) => return Err(Error::Mutex),
        };

        for (modifier, out) in substitutions.iter() {
            *prompt = prompt.replace(modifier, out);
        }

        super::builtins::export::substitute_one(prompt)?;

        *prompt = super::exec::substitute_inner_exec_one(prompt.clone())?.join(" ");

        Ok(())
    }

    fn get_prompt(&mut self) -> Result<String, Error> {
        let default_prompt = "\x1b[1;33mrsh\x1b[1;32m $>\x1b[0m ".to_string();

        let mut p = super::builtins::export::get("PROMPT")
            .unwrap_or(default_prompt.clone())
            .clone();

        Self::substitute_prompt(&mut p)?;

        let p = match unescape::unescape(&p) {
            Some(p) => p,
            None => "PROMPT_ERROR > ".to_string(),
        };

        if let Some(helper) = self.editor.helper_mut() {
            helper.colored_prompt = p.clone();
        };

        let p = String::from_utf8(strip_ansi_escapes::strip(&p)?)?;

        Ok(p)
    }

    pub fn exit(&mut self) -> Result<(), Error> {
        self.editor
            .save_history(&format!(
                "{}/.rsh_history",
                dirs::home_dir().unwrap().to_str().unwrap()
            ))
            .map_err(Error::from)
    }

    fn aquire_script(&mut self) -> Result<String, Error> {
        if let Some(script) = &self.script {
            self.script_idx += 1;

            let res = script.get(self.script_idx - 1).cloned().into_result();

            res.map_err(|x| Error::from(x))
        } else {
            Err(Error::Lexer)
        }
    }

    fn aquire_readline(&mut self) -> Result<String, Error> {
        let p = self.get_prompt()?;

        self.editor
            .readline(&p)
            .map(|line| {
                self.editor.add_history_entry(line.as_str());

                line
            })
            .map_err(|err| match err {
                ReadlineError::Interrupted => Error::Interrupt,
                ReadlineError::Io(..)
                | ReadlineError::Eof
                | ReadlineError::Utf8Error
                | ReadlineError::Errno(..) => Error::from(err),
            })
    }

    pub fn aquire(&mut self) -> Result<String, Error> {
        if self.script.is_some() {
            self.aquire_script()
        } else {
            self.aquire_readline()
        }
    }
}
