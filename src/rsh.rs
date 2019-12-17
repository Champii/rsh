use super::config::Config;
use super::error::Error;
use super::exec::Executor;
use super::input::Input;
use super::parsing::Parser;

pub struct RSH {
    input: Input,
    executor: Executor,
    parser: Parser,
    pub config: Config,
}

impl RSH {
    pub fn new(config: Config) -> Self {
        let mut rsh = Self {
            input: Input::new(config.clone()),
            executor: Executor::new(),
            parser: Parser::new(),
            config,
        };

        if rsh.config.script_path.is_none() && rsh.config.input.is_none() {
            if rsh.load_conf().is_err() {
                println!("Warning: Cannot load .rshrc");
            }
        }

        rsh
    }

    fn load_conf(&mut self) -> Result<(), Error> {
        let p = format!("{}/.rshrc", dirs::home_dir().unwrap().to_str().unwrap());

        let config = Config {
            script_path: Some(p),
            input: None,
        };

        Self::new(config).run()
    }

    fn strip_comment(line: &str) -> &str {
        if let Some(idx) = line.find("#") {
            if idx == 0 {
                ""
            } else {
                &line[idx - 1..]
            }
        } else {
            line
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.input.init()?;

        loop {
            match self.input.aquire() {
                Ok(line) => {
                    let line = Self::strip_comment(&line);

                    if line.is_empty() {
                        continue;
                    }

                    let ast = self.parser.run(&line.clone())?;

                    match self.executor.run(ast) {
                        Ok(_) => (),
                        Err(Error::Run(s)) => {
                            println!("Error Run: {}", s);
                        }
                        Err(err) => {
                            println!("Error: {}", err);

                            return Err(err);
                        }
                    }
                }
                Err(err) => match err {
                    Error::Interrupt => {}
                    Error::Run(..)
                    | Error::String(..)
                    | Error::Builtin
                    | Error::Env(..)
                    | Error::Mutex
                    | Error::None(..)
                    | Error::Parser(..)
                    | Error::Lexer
                    | Error::Io(..)
                    | Error::Readline(..) => break,
                },
            };
        }

        Ok(())
    }
}
