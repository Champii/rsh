use os_pipe::pipe;
use std::process::{Child, Command as OSCommand};

use super::super::error::Error;
use super::super::parsing::*;

pub struct BuiltinProgram {}

pub trait Program {
    fn run(&self);
    fn code(&mut self) -> Result<i32, Error>;
}

impl Program for Child {
    fn run(&self) {}
    fn code(&mut self) -> Result<i32, Error> {
        Ok(self.wait()?.code().unwrap())
    }
}

pub trait Runnable {
    fn exec(&self) -> Result<Box<dyn Program>, Error>;
}

impl Runnable for Ast {
    fn exec(&self) -> Result<Box<dyn Program>, Error> {
        let mut last_child = None;

        for cmd in &self.0 {
            let mut child = cmd.exec()?;
            let code = child.code().unwrap();

            if code != 0 {
                return Ok(child);
            }

            last_child = Some(child);
        }

        Ok(last_child.unwrap())
    }
}

fn pre_exec(cmd: &mut CommandRaw) -> Result<(), Error> {
    if cmd.exe.chars().nth(0).unwrap() == '\\' {
        cmd.exe = cmd.exe[1..].to_string();
    } else {
        super::super::builtins::alias::substitute(cmd)?;
    }

    super::super::builtins::export::substitute(&mut cmd.exe)?;

    for arg in &mut cmd.args {
        super::super::builtins::export::substitute(arg)?;
    }

    Ok(())
}

impl Runnable for CommandRaw {
    fn exec(&self) -> Result<Box<dyn Program>, Error> {
        if self.exe.is_empty() {
            return super::ok_true();
        }

        let mut cmd = self.clone();

        pre_exec(&mut cmd)?;

        let builtins = super::super::builtins::get_builtins();

        if let Some(f) = builtins.get(&cmd.exe) {
            f(&cmd)
        } else {
            let child = OSCommand::new(&cmd.exe)
                .args(&cmd.args)
                .spawn()
                .map_err(|e| Error::Run(e.to_string()))?;

            Ok(Box::new(child))
        }
    }
}

fn exec_pipe(cmd1: &CommandRaw, cmd2: &CommandRaw) -> Result<Box<dyn Program>, Error> {
    // if cmd1.exe.is_empty() {
    //     return super::ok_true();
    // }

    // let mut cmd = self.clone();

    // if cmd.exe.chars().nth(0).unwrap() == '\\' {
    //     cmd.exe = cmd.exe[1..].to_string();
    // } else {
    //     super::super::builtins::alias::substitute(&mut cmd)?;
    // }

    // let builtins = super::super::builtins::get_builtins();

    // if let Some(f) = builtins.get(&cmd.exe) {
    //     f(&cmd)
    // } else {
    //     let child = OSCommand::new(&cmd.exe)
    //         .args(&cmd.args)
    //         .spawn()
    //         .map_err(|_| Error::Run)?;

    //     Ok(Box::new(child))
    // }

    // let (mut reader, writer) = pipe().unwrap();
    // let writer_clone = writer.try_clone().unwrap();
    // child.stdout(writer);
    // child.stderr(writer_clone);

    // let mut handle = child.spawn().unwrap();
    Err(Error::Run(String::new()))
}

impl Runnable for Command {
    fn exec(&self) -> Result<Box<dyn Program>, Error> {
        match self {
            Self::Raw(cmd) => cmd.exec(),
            Self::Parenthesis(cmd) => cmd.exec(),
            Self::And(left, right) => {
                let mut child = left.exec()?;
                let code = child.code().unwrap();

                if code == 0 {
                    right.exec()
                } else {
                    Ok(child)
                }
            }
            Self::Or(left, right) => {
                let mut child = left.exec()?;
                let code = child.code().unwrap();

                if code == 0 {
                    Ok(child)
                } else {
                    right.exec()
                }
            }
            Self::Pipe(left, _right) => {
                let child = left.exec()?;

                // Here's the interesting part. Open a pipe, copy its write end, and
                // give both copies to the child.
                // let mut child2 = right.exec()?;

                Ok(child)
            }
        }
    }
}
