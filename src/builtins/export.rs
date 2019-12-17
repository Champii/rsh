use std::env;

use super::super::error::Error;
use super::super::exec::Program;
use super::super::parsing::CommandRaw;
use super::BuiltinFn;
use super::{ok_false, ok_true};
use regex::Regex;

fn print_exports() -> Result<(), Error> {
    for (var, val) in env::vars() {
        println!("{}={}", var, val);
    }

    Ok(())
}

fn export(cmd: &CommandRaw) -> Result<Program, Error> {
    if cmd.args.is_empty() {
        print_exports()?;
    } else if cmd.args.len() == 1 {
        env::remove_var(&cmd.args[0])
    } else if cmd.args.len() == 2 {
        let mut val = cmd.args[1].clone();

        let first_char = match val.chars().nth(0) {
            Some(c) => c,
            None => ' ',
        };

        if first_char == '"' {
            val = val[1..val.len() - 1].to_string();
        }

        env::set_var(cmd.args[0].clone(), val);
    } else {
        println!("Usage: export [name [\"value\"]]");

        return ok_false();
    }

    ok_true()
}

pub fn builtin_export() -> Box<BuiltinFn> {
    Box::new(|cmd| export(cmd))
}

pub fn get(var: &str) -> Result<String, Error> {
    if var.is_empty() {
        return Ok(String::new());
    }

    Ok(env::var(var)?)
}

pub fn substitute(cmd: &mut CommandRaw) -> Result<(), Error> {
    substitute_one(&mut cmd.exe)?;

    for arg in &mut cmd.args {
        substitute_one(arg)?;
    }

    Ok(())
}

pub fn substitute_one(s: &mut String) -> Result<(), Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\$\w+)").unwrap();
        static ref REP: Regex = Regex::new(r"(\$\{\w+\})").unwrap();
    }

    let mut s_cpy = s.clone();

    for cap in RE.captures_iter(&s) {
        for c in cap.iter().skip(1) {
            let to_replace = c.unwrap().as_str();

            let res = match get(&to_replace[1..]) {
                Ok(res) => res,
                Err(_) => String::new(),
            };

            s_cpy = s_cpy.replace(to_replace, res.as_str());
        }
    }

    // TODO: fixme
    for cap in REP.captures_iter(&s) {
        for c in cap.iter().skip(1) {
            let to_replace = c.unwrap().as_str();

            let res = match get(&to_replace[2..to_replace.len() - 1]) {
                Ok(res) => res,
                Err(_) => String::new(),
            };

            s_cpy = s_cpy.replace(to_replace, res.as_str());
        }
    }

    *s = s_cpy;

    Ok(())
}
