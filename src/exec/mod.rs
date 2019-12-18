use super::error::Error;
use super::parsing::*;

mod program;
mod runnable;
mod substitute;

pub use program::Program;
pub use runnable::Runnable;
pub use substitute::substitute_inner_exec_one;

pub fn ok_true() -> Result<Program, Error> {
    let mut prog = Program::new(CommandRaw::new("true".to_string(), vec![]));

    prog.run()?;
    prog.wait()?;

    Ok(prog)
}

pub fn ok_false() -> Result<Program, Error> {
    let mut prog = Program::new(CommandRaw::new("false".to_string(), vec![]));

    prog.run()?;
    prog.wait()?;

    Ok(prog)
}

pub struct Executor {
    source: Option<Ast>,
}

impl Executor {
    pub fn new() -> Self {
        std::env::set_var("?", "0");

        Self { source: None }
    }

    pub fn run(&mut self, source: Option<Ast>) -> Result<(), Error> {
        self.source = source;

        if let Some(source) = &self.source {
            let mut prog = source.exec()?;

            if let Ok(child) = prog.wait() {
                if let Some(code) = child.code() {
                    std::env::set_var("?", code.to_string());
                }
            }
        }

        Ok(())
    }
}
