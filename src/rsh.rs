use super::error::Error;
use super::exec::Executor;
use super::input::Input;
use super::parsing::Parser;

pub struct RSH {
    input: Input,
    executor: Executor,
    parser: Parser,
}

impl RSH {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
            executor: Executor::new(),
            parser: Parser::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.input.init()?;

        loop {
            match self.input.aquire() {
                Ok(line) => {
                    let ast = self.parser.run(&line.clone())?;

                    // println!("{:#?}", ast);

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

pub fn run() -> Result<(), Error> {
    RSH::new().run()
}
