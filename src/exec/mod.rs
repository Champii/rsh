use std::process::Command as OSCommand;

use super::error::Error;
use super::parsing::*;

mod runnable;

pub use runnable::{Program, Runnable};

pub fn ok_true() -> Result<Program, Error> {
    let mut prog = Program::new("true", vec![]);

    prog.run()?;
    prog.wait()?;

    Ok(prog)
}
pub fn ok_false() -> Result<Program, Error> {
    let mut prog = Program::new("false", vec![]);

    prog.run()?;
    prog.wait()?;

    Ok(prog)
}

pub struct Executor {
    source: Option<Ast>,
}

impl Executor {
    pub fn new() -> Self {
        Self { source: None }
    }

    pub fn run(&mut self, source: Option<Ast>) -> Result<(), Error> {
        self.source = source;

        if let Some(source) = &self.source {
            source.exec()?;
        }

        Ok(())
    }
}
