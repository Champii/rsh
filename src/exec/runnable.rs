use super::super::error::Error;
use super::super::parsing::*;
use std::process::{Child, Command as OSCommand};

pub trait Runnable {
    fn exec(&self) -> Result<Child, Error>;
}

impl Runnable for Ast {
    fn exec(&self) -> Result<Child, Error> {
        let mut last_child = None;

        for cmd in &self.0 {
            let mut child = cmd.exec()?;
            let code = child.wait()?.code().unwrap();

            if code != 0 {
                return Ok(child);
            }

            last_child = Some(child);
        }

        Ok(last_child.unwrap())
    }
}

impl Runnable for CommandRaw {
    fn exec(&self) -> Result<Child, Error> {
        if self.exe.is_empty() {
            return Err(Error::Run);
        }

        let child = OSCommand::new(&self.exe)
            .args(&self.args)
            .spawn()
            .map_err(|_| Error::Run)?;

        Ok(child)
    }
}

impl Runnable for Command {
    fn exec(&self) -> Result<Child, Error> {
        match self {
            Self::Raw(cmd) => cmd.exec(),
            Self::Parenthesis(cmd) => cmd.exec(),
            Self::And(left, right) => {
                let mut child = left.exec()?;
                let code = child.wait()?.code().unwrap();

                if code == 0 {
                    right.exec()
                } else {
                    Ok(child)
                }
            }
            Self::Or(left, right) => {
                let mut child = left.exec()?;
                let code = child.wait()?.code().unwrap();

                if code == 0 {
                    Ok(child)
                } else {
                    right.exec()
                }
            }
            Self::Pipe(left, _right) => {
                let child1 = left.exec()?;

                // let mut child2 = right.exec()?;

                Ok(child1)
            }
        }
    }
}
