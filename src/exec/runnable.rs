use std::process::Stdio;

use super::super::error::Error;
use super::super::parsing::*;
use super::program::Program;

pub trait Runnable {
    fn exec(&self) -> Result<Program, Error>;
}

impl Runnable for Ast {
    fn exec(&self) -> Result<Program, Error> {
        let mut last_prog = None;

        for cmd in &self.0 {
            let prog = cmd.exec()?;
            // let code = prog.wait()?.code().unwrap();

            // if code != 0 {
            //     return Ok(prog);
            // }

            last_prog = Some(prog);
        }

        Ok(last_prog.unwrap())
    }
}

impl Runnable for CommandRaw {
    fn exec(&self) -> Result<Program, Error> {
        if self.exe.is_empty() {
            return super::ok_true();
        }

        let mut cmd = self.clone();

        super::substitute::pre_exec(&mut cmd)?;

        let builtins = super::super::builtins::get_builtins();

        if let Some(f) = builtins.get(&cmd.exe) {
            f(&cmd)
        } else {
            Ok(Program::new(cmd))
        }
    }
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
                    // let mut right = right.clone();

                    // right.replace_left(Box::new(Self::Raw(CommandRaw::new(
                    //     "false".to_string(),
                    //     vec![],
                    // ))));

                    // if let Self::Raw(_) = &*right {
                    //     return Ok(left_prog);
                    // } else {
                    //     return right.exec();
                    // }
                }

                right.exec()
            }
            Self::Or(left, right) => {
                let mut left_prog = left.exec()?;

                let code = left_prog.wait()?.code().unwrap();

                if code == 0 {
                    let mut right = right.clone();

                    right.replace_left(Box::new(Self::Raw(CommandRaw::new(
                        "true".to_string(),
                        vec![],
                    ))));

                    if let Self::Raw(_) = &*right {
                        return Ok(left_prog);
                    } else {
                        return right.exec();
                    }
                }

                right.exec()
            }
            Self::Pipe(left, right) => {
                if let Self::Raw(left_cmd) = &**left {
                    if let Self::Raw(right_cmd) = &**right {
                        let mut left_prog = left_cmd.exec()?;

                        left_prog.prog.stdout(Stdio::piped());

                        left_prog.run()?;

                        let mut right_prog = right_cmd.exec()?;

                        let to_run = right_prog
                            .prog
                            .stdin(left_prog.child.unwrap().stdout.unwrap());

                        let right_child = to_run.spawn()?;

                        // TODO: Fixme, very ugly
                        let definitive = Program {
                            child: Some(right_child),
                            prog: right_prog.prog,
                            cmd: right_cmd.clone(),
                        };

                        return Ok(definitive);
                    } else {
                        return right.exec();
                    }
                }

                Err(Error::Run("Cannot pipe".to_string()))
            }
        }
    }
}
