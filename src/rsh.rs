use super::error::Error;
use super::exec::Executor;
use super::input::Input;
use super::parser::Parser;

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

                    println!("{:#?}", ast);

                    self.executor.run(ast)?
                }
                Err(err) => match err {
                    Error::Interrupt => {}
                    Error::Parser(..) | Error::Lexer | Error::Io(..) | Error::Readline(..) => break,
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
