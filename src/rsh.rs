use std::path::Path;

use super::config::Config;
use super::error::Error;
use super::exec::Executor;
use super::input::Input;
use super::parsing::Parser;

pub struct RSH {
    input: Input,
    executor: Executor,
    parser: Parser,
    config: Config,
}

impl RSH {
    pub fn new(config: Config) -> Self {
        Self {
            input: Input::new(),
            executor: Executor::new(),
            parser: Parser::new(),
            config,
        }
    }

    fn load_conf(&mut self) -> Result<(), Error> {
        //
        let p = format!("{}/.rsh_history", env!("HOME").to_owned());
        let filepath = Path::new(&p);

        self.run(filepath)?;

        Ok(())
    }

    pub fn run(&mut self, filepath: &Path) -> Result<(), Error> {
        let file = std::fs::read_to_string(filepath)?;
        let lines = file.split('\n').collect::<Vec<&str>>();

        for line in lines {
            if line.is_empty() {
                continue;
            }

            let ast = self.parser.run(&line)?;

            match self.executor.run(ast) {
                Ok(_) => (),
                Err(Error::Run) => {
                    println!("Error: {}", Error::Run);
                }
                Err(err) => {
                    println!("Error: {}", err);

                    return Err(err);
                }
            }
        }

        Ok(())
    }

    pub fn interactive(&mut self) -> Result<(), Error> {
        self.load_conf()?;

        self.input.init()?;

        loop {
            match self.input.aquire() {
                Ok(line) => {
                    let ast = self.parser.run(&line.clone())?;

                    match self.executor.run(ast) {
                        Ok(_) => (),
                        Err(Error::Run) => {
                            println!("Error: {}", Error::Run);
                        }
                        Err(err) => {
                            println!("Error: {}", err);

                            return Err(err);
                        }
                    }
                }
                Err(err) => match err {
                    Error::Interrupt => {}
                    Error::Run
                    | Error::Builtin
                    | Error::Mutex
                    | Error::Parser(..)
                    | Error::Lexer
                    | Error::Io(..)
                    | Error::Readline(..) => break,
                },
            };
        }

        self.input.exit()?;

        Ok(())
    }
}
