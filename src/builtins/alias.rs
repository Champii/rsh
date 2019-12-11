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

fn print_alias() {
    for (name, alias) in ALIAS.read().unwrap().iter() {
        println!("{}=\"{}\"", name, alias);
    }
}

pub fn substitute(cmd: &mut CommandRaw) {
    let alias = ALIAS.read().unwrap();

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
            substitute(cmd);
        }
    };
}

fn alias(cmd: &CommandRaw) -> Result<Child, Error> {
    if cmd.args.is_empty() {
        print_alias();
    } else if cmd.args.len() == 1 {
        ALIAS.write().unwrap().remove(&cmd.args[0]);
    } else if cmd.args.len() == 2 {
        let val = cmd.args[1].clone();
        let val = val[1..val.len() - 1].to_string();

        ALIAS.write().unwrap().insert(cmd.args[0].clone(), val);
    } else {
        println!("Usage: alias [name [\"value\"]]");

        return super::ok_false();
    }

    super::ok_true()
}

pub fn builtin_alias() -> Box<BuiltinFn> {
    Box::new(|cmd| alias(cmd))
}
