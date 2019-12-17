use regex::Regex;
use std::process::Stdio;

use super::super::error::*;
use super::super::parsing::*;
use super::Runnable;

pub fn substitute_inner_exec(cmd: &mut CommandRaw) -> Result<(), Error> {
    let mut new_args = vec![];

    let exe_res = substitute_inner_exec_one(cmd.exe.clone())?;
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

            let cmd = CommandRaw::new(
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
    let splited = s_cpy
        .to_string()
        .split(|c| c == ' ')
        .map(|x| (*x).to_string())
        .collect::<Vec<String>>();

    Ok(splited)
}

pub fn pre_exec(cmd: &mut CommandRaw) -> Result<(), Error> {
    if cmd.exe.chars().nth(0).unwrap() == '\\' {
        cmd.exe = cmd.exe[1..].to_string();
    } else {
        super::super::builtins::alias::substitute(cmd)?;
    }

    super::super::builtins::export::substitute(cmd)?;

    substitute_inner_exec(cmd)?;

    Ok(())
}
