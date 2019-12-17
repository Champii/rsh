use std::process::{Child, Command as OSCommand, ExitStatus};

use super::super::error::*;
use super::super::parsing::*;

#[derive(Debug)]
pub struct Program {
    pub cmd: CommandRaw,
    pub prog: OSCommand,
    pub child: Option<Child>,
}

impl Program {
    pub fn new(cmd: CommandRaw) -> Self {
        let mut prog = OSCommand::new(cmd.exe.clone());

        for arg in &cmd.args {
            prog.arg(arg);
        }

        Self {
            cmd,
            prog,
            child: None,
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        match self.prog.spawn() {
            Ok(child) => self.child = Some(child),
            Err(_) => println!("Command not found: {}", self.cmd.exe),
        };

        Ok(())
    }

    pub fn wait(&mut self) -> Result<ExitStatus, Error> {
        match &mut self.child {
            Some(child) => Ok(child.wait()?),
            None => Ok(super::ok_false()?.wait()?),
        }
    }
}
