use super::super::error::Error;
use super::super::parser::*;
use std::process::Command as OSCommand;

pub trait Runnable {
    fn exec(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Runnable for Ast {
    fn exec(&self) -> Result<(), Error> {
        for cmd in &self.0 {
            cmd.exec()?;
        }

        Ok(())
    }
}

impl Runnable for CommandRaw {
    fn exec(&self) -> Result<(), Error> {
        if self.exe.is_empty() {
            return Ok(());
        }

        let mut child = match OSCommand::new(&self.exe).args(&self.args).spawn() {
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

impl Runnable for Command {
    fn exec(&self) -> Result<(), Error> {
        match self {
            Self::Raw(cmd) => cmd.exec(),
            Self::Parenthesis(cmd) => cmd.exec(),
            Self::And(left, _right) => left.exec(),
            Self::Or(left, _right) => left.exec(),
            Self::Pipe(left, _right) => left.exec(),
        }
    }
}
