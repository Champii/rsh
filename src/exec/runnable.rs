use super::super::error::Error;
use super::super::parser::*;
use std::process::{Child, Command as OSCommand, Stdio};

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

// fn exec_pipe(this: Box<CommandRaw>, source: Child) -> Result<Child, Error> {
//     if this.exe.is_empty() {
//         return Err(Error::Run);
//     }

//     let child = OSCommand::new(&this.exe)
//         .args(&this.args)
//         .stdin(source.stdout.unwrap())
//         .spawn()
//         .map_err(|_| Error::Run)?;

//     Ok(child)
// }

// fn get_cmd(this: Box<CommandRaw>) -> Result<&mut OSCommand, Error> {
//     if this.exe.is_empty() {
//         return Err(Error::Run);
//     }

//     let cmd = OSCommand::new(&this.exe).args(&this.args);

//     Ok(cmd)
// }

impl Runnable for CommandRaw {
    fn exec(&self) -> Result<Child, Error> {
        if self.exe.is_empty() {
            return Err(Error::Run);
        }

        let mut child = OSCommand::new(&self.exe)
            .args(&self.args)
            // .stdin(Stdio::null())
            // .stdout(Stdio::null())
            // .stderr(Stdio::null())
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
            Self::Pipe(left, right) => {
                // let cmd1 = OSCommand::new((**left).exe).args(&left.args);
                // let cmd2 = OSCommand::new(&right.exe).args(&right.args);
                let child1 = left.exec()?;
                let mut child2 = right.exec()?;

                // child2.stdin(child2.stdout());
                // std::process::Stdio::from_inner(child1.to_inner()).

                // exec_pipe(right, child);
                // child.stdout
                Ok(child1)
            }
        }
    }
}
