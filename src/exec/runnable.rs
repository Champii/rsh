use os_pipe::pipe;
use regex::Regex;
use std::process::{Child, Command as OSCommand, ExitStatus, Stdio};

use super::super::config::Config;
use super::super::error::Error;
use super::super::parsing::*;

#[derive(Debug)]
pub struct Program {
    cmd: CommandRaw,
    prog: OSCommand,
    child: Option<Child>,
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

pub fn substitute_inner_exec(cmd: &mut CommandRaw) -> Result<(), Error> {
    let mut new_args = vec![];

    let mut exe_res = substitute_inner_exec_one(cmd.exe.clone())?;
    cmd.exe = exe_res[0].clone();

    if exe_res.len() > 1 {
        new_args = exe_res[1..].to_vec();
    }

    for arg in &mut cmd.args {
        let res = substitute_inner_exec_one(arg.clone())?;

        for x in res {
            new_args.push(x);
        }
    }

    cmd.args = new_args;

    // println!("CMD {:#?}", cmd);
    Ok(())
}

pub fn substitute_inner_exec_one(s: String) -> Result<Vec<String>, Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(`.+`)").unwrap();
    }

    let mut s_cpy = s.clone();

    for cap in RE.captures_iter(&s) {
        for c in cap.iter().skip(1) {
            let inner = c.unwrap().as_str();

            let mut cmd = CommandRaw::new(
                // TODO: Security vulmerability here
                // https://vulners.com/securityvulns/SECURITYVULNS:DOC:22183
                std::env::current_exe()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                vec!["-e".to_string(), inner[1..inner.len() - 1].to_string()],
            );

            let mut prog = cmd.exec()?;

            prog.prog.stdout(Stdio::piped());
            prog.prog.stderr(Stdio::piped());

            if prog.run().is_err() || prog.child.is_none() {
                s_cpy = s_cpy.replace(inner, "");

                continue;
            }

            let out = if let Some(child) = prog.child {
                if let Ok(out) = child.wait_with_output() {
                    out
                } else {
                    s_cpy = s_cpy.replace(inner, "");

                    continue;
                }
            } else {
                s_cpy = s_cpy.replace(inner, "");

                continue;
            };

            let mut parsed = String::from_utf8(out.stdout)
                .map_err(|_| Error::Run("Cannot read stdout".to_string()))?;

            parsed = parsed.replace('\n', "");

            let escaped = match &unescape::unescape(&parsed) {
                Some(escaped) => escaped.clone(),
                None => parsed,
            };

            // TODO: do one exec per match
            s_cpy = s_cpy.replace(inner, &escaped);
        }
    }

    if s == s_cpy {
        return Ok(vec![s.clone()]);
    }

    // TODO: handle multiline output (create new CommandRaw for that)
    let mut splited = s_cpy
        .to_string()
        .split(|c| c == ' ')
        .map(|x| (*x).to_string())
        .collect::<Vec<String>>();

    Ok(splited)
}

fn pre_exec(cmd: &mut CommandRaw) -> Result<(), Error> {
    if cmd.exe.chars().nth(0).unwrap() == '\\' {
        cmd.exe = cmd.exe[1..].to_string();
    } else {
        super::super::builtins::alias::substitute(cmd)?;
    }

    super::super::builtins::export::substitute(cmd)?;

    substitute_inner_exec(cmd)?;

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
            Ok(Program::new(cmd))
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
