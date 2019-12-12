use super::error::Error;
use super::parsing::*;

mod runnable;

use runnable::Runnable;

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
