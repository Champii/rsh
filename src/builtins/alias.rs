use std::{
    collections::HashMap,
    process::Child,
    sync::{Arc, RwLock},
};

use super::super::error::Error;
use super::super::parsing::CommandRaw;
use super::BuiltinFn;

lazy_static! {
    pub static ref ALIAS: Arc<RwLock<HashMap<String, String>>> =
        { Arc::new(RwLock::new(HashMap::new())) };
}

fn print_alias() -> Result<(), Error> {
    let alias = match ALIAS.read() {
        Ok(alias) => alias,
        Err(_) => return Err(Error::Mutex),
    };

    let mut v = vec![];

    for (name, alias) in alias.iter() {
        v.push(format!("{}=\"{}\"", name, alias));
    }

    v.sort();

    for alias in v {
        println!("{}", alias);
    }

    Ok(())
}

pub fn substitute(cmd: &mut CommandRaw) -> Result<(), Error> {
    let alias = match ALIAS.read() {
        Ok(alias) => alias,
        Err(_) => return Err(Error::Mutex),
    };

    if let Some(res) = alias.get(&cmd.exe) {
        let res_splited = res
            .split(' ')
            .map(|x| (*x).to_string())
            .collect::<Vec<String>>();

        let exe = res_splited[0].clone();

        let mut res_splited = res_splited[1..].to_vec();

        res_splited.extend_from_slice(&cmd.args);

        cmd.args = res_splited;

        cmd.exe = exe;

        if alias.get(&cmd.exe).is_some() {
            substitute(cmd)?;
        }
    };

    Ok(())
}

fn alias(cmd: &CommandRaw) -> Result<Child, Error> {
    if cmd.args.is_empty() {
        print_alias()?;
    } else if cmd.args.len() == 1 {
        let mut alias = match ALIAS.write() {
            Ok(alias) => alias,
            Err(_) => return Err(Error::Mutex),
        };

        alias.remove(&cmd.args[0]);
    } else if cmd.args.len() == 2 {
        let mut val = cmd.args[1].clone();

        let first_char = match val.chars().nth(0) {
            Some(c) => c,
            None => return Err(Error::Builtin),
        };

        if first_char == '"' {
            val = val[1..val.len() - 1].to_string();
        }

        let mut alias = match ALIAS.write() {
            Ok(alias) => alias,
            Err(_) => return Err(Error::Mutex),
        };

        alias.insert(cmd.args[0].clone(), val);
    } else {
        println!("Usage: alias [name [\"value\"]]");

        return super::ok_false();
    }

    super::ok_true()
}

pub fn builtin_alias() -> Box<BuiltinFn> {
    Box::new(|cmd| alias(cmd))
}
