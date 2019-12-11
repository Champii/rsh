use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::config::OutputStreamType;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::{
    At, Cmd, ColorMode, CompletionType, Config, Context, EditMode, Editor, KeyPress, Movement, Word,
};
use rustyline_derive::Helper;
use std::borrow::Cow::{self, Borrowed, Owned};

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

pub struct Input {
    editor: Editor<EditorHelper>,
}

impl Input {
    pub fn new() -> Self {
        let config = Config::builder()
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

        let mut editor = Editor::with_config(config);

        editor.bind_sequence(
            KeyPress::ControlLeft,
            Cmd::Move(Movement::BackwardWord(1, Word::Big)),
        );

        editor.bind_sequence(
            KeyPress::ControlRight,
            Cmd::Move(Movement::ForwardWord(1, At::Start, Word::Big)),
        );

        editor.set_helper(Some(h));

        Self { editor }
    }

    pub fn init(&mut self) -> Result<(), Error> {
        match self
            .editor
            .load_history(&format!("{}/.rsh_history", env!("HOME").to_owned()))
        {
            Ok(_) => Ok(()),
            Err(_) => Ok(()),
        }
    }

    pub fn exit(&mut self) -> Result<(), Error> {
        self.editor
            .save_history(&format!("{}/.rsh_history", env!("HOME").to_owned()))
            .map_err(Error::from)
    }

    pub fn aquire(&mut self) -> Result<String, Error> {
        let p = "rsh #> ";

        if let Some(helper) = self.editor.helper_mut() {
            helper.colored_prompt = format!("\x1b[1;33m{}\x1b[1;32m{}\x1b[0m", "rsh ", "#> ");
        };

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
}
