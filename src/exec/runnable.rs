use super::super::error::Error;
use super::super::parser::*;
use std::process::Command as OSCommand;

pub trait Runnable {
    fn exec(&self) -> Result<i32, Error> {
        Ok(0)
    }
}

impl Runnable for Ast {
    fn exec(&self) -> Result<i32, Error> {
        for cmd in &self.0 {
            let code = cmd.exec()?;

            if code != 0 {
                return Ok(code);
            }
        }

        Ok(0)
    }
}

impl Runnable for CommandRaw {
    fn exec(&self) -> Result<i32, Error> {
        if self.exe.is_empty() {
            return Ok(0);
        }

        let mut child = match OSCommand::new(&self.exe).args(&self.args).spawn() {
            Ok(child) => child,
            Err(err) => {
                println!("Error: {}", err);

                return Ok(0);
            }
        };

        let status = child.wait()?;

        Ok(status.code().unwrap())
    }
}

impl Runnable for Command {
    fn exec(&self) -> Result<i32, Error> {
        match self {
            Self::Raw(cmd) => cmd.exec(),
            Self::Parenthesis(cmd) => cmd.exec(),
            Self::And(left, right) => {
                let code = left.exec()?;

                if code == 0 {
                    right.exec()
                } else {
                    Ok(code)
                }
            }
            Self::Or(left, right) => {
                let code = left.exec()?;

                if code == 0 {
                    Ok(code)
                } else {
                    right.exec()
                }
            }
            Self::Pipe(left, _right) => left.exec(),
        }
    }
}
