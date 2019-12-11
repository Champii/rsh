use std::io::{self, Write};
use std::process::{Command, Stdio};

use super::error::Error;
use super::input::Input;

pub struct RSH {
    input: Input,
}

impl RSH {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.input.init()?;

        loop {
            match self.input.aquire() {
                Ok(line) => self.exec(&line)?,
                Err(err) => match err {
                    Error::Interrupt => {}
                    Error::Io(..) | Error::Readline(..) => break,
                },
            };
        }

        self.input.exit()?;

        Ok(())
    }

    fn exec(&mut self, cmd: &str) -> Result<(), Error> {
        if cmd.is_empty() {
            return Ok(());
        }

        let line: Vec<&str> = cmd.split(' ').collect::<_>();

        let mut child = match Command::new(&line[0]).args(&line[1..]).spawn() {
            Ok(child) => child,
            Err(err) => {
                println!("Error: {}", err);

                return Ok(());
            }
        };

        child.wait()?;

        Ok(())
    }
}

pub fn run() -> Result<(), Error> {
    RSH::new().run()
}
