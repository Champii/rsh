use std::process::Command as OSCommand;

use super::error::Error;
use super::parsing::*;

mod runnable;

pub use runnable::{Program, Runnable};

pub fn ok_true() -> Result<Box<dyn Program>, Error> {
    OSCommand::new("true")
        .spawn()
        .map_err(|e| Error::Run(e.to_string()))
        .map(|x| Box::new(x) as Box<dyn Program>)
}
pub fn ok_false() -> Result<Box<dyn Program>, Error> {
    OSCommand::new("false")
        .spawn()
        .map_err(|e| Error::Run(e.to_string()))
        .map(|x| Box::new(x) as Box<dyn Program>)
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
