use os_pipe::pipe;
use std::process::{Child, Command as OSCommand, ExitStatus};

use super::super::error::Error;
use super::super::parsing::*;

// pub struct BuiltinProgram {}
pub struct Program {
    cmd: OSCommand,
    args: Vec<String>,
    child: Option<Child>,
}

impl Program {
    pub fn new(cmd: &str, args: Vec<String>) -> Self {
        let mut c = OSCommand::new(cmd);

        for arg in &args {
            c.arg(arg);
        }

        Self {
            cmd: c,
            args: args.iter().map(|x| x.to_string()).collect(),
            // args: args.iter().map(|x| x.to_string()).collect(),
            child: None,
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.child = Some(self.cmd.spawn()?);

        Ok(())
    }

    pub fn wait(&mut self) -> Result<ExitStatus, Error> {
        match &mut self.child {
            Some(child) => Ok(child.wait()?),
            None => Err(Error::Run("Error waiting for child".to_string())),
        }
        // self.child.map(|x| x.wait().unwrap_or(Error::Run))
    }
}

// pub trait Program {
//     fn run(&self);
//     fn code(&mut self) -> Result<i32, Error>;
// }

// impl Program for Child {
//     fn run(&self) {}
//     fn code(&mut self) -> Result<i32, Error> {
//         Ok(self.wait()?.code().unwrap())
//     }
// }

pub trait Runnable {
    fn exec(&self) -> Result<Program, Error>;
}

impl Runnable for Ast {
    fn exec(&self) -> Result<Program, Error> {
        let mut last_prog = None;

        for cmd in &self.0 {
            let mut prog = cmd.exec()?;
            let code = prog.wait()?.code().unwrap();

            if code != 0 {
                return Ok(prog);
            }

            last_prog = Some(prog);
        }

        Ok(last_prog.unwrap())
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
    fn exec(&self) -> Result<Program, Error> {
        if self.exe.is_empty() {
            return super::ok_true();
        }

        let mut cmd = self.clone();

        pre_exec(&mut cmd)?;

        let builtins = super::super::builtins::get_builtins();

        if let Some(f) = builtins.get(&cmd.exe) {
            f(&cmd)
        } else {
            let mut prog = Program::new(&cmd.exe, cmd.args);

            // prog.run()?;
            // prog.wait().map_err(|e| Error::Run(e.to_string()))?;

            Ok(prog)
        }
    }
}

fn exec_pipe(cmd1: &CommandRaw, cmd2: &CommandRaw) -> Result<Program, Error> {
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
    fn exec(&self) -> Result<Program, Error> {
        match self {
            Self::Raw(cmd) => {
                let mut prog = cmd.exec()?;

                prog.run()?;
                prog.wait().map_err(|e| Error::Run(e.to_string()))?;

                Ok(prog)
            }
            Self::Parenthesis(cmd) => cmd.exec(),
            Self::And(left, right) => {
                let mut left_prog = left.exec()?;

                let left_code = left_prog.wait()?.code().unwrap();

                if left_code != 0 {
                    return Ok(left_prog);
                }

                right.exec()
            }
            Self::Or(left, right) => {
                let mut left_prog = left.exec()?;

                let code = left_prog.wait()?.code().unwrap();

                if code == 0 {
                    return Ok(left_prog);
                }

                right.exec()
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
