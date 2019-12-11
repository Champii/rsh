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
        println!("{}={}", name, alias);
    }
}

fn alias(cmd: &CommandRaw) -> Result<Child, Error> {
    if cmd.args.is_empty() {
        print_alias();
    } else if cmd.args.len() == 1 {
        let splited = cmd.args[0]
            .clone()
            .split('=')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        match splited.len() {
            // Remove
            1 => ALIAS.write().unwrap().remove(&splited[0]),
            // Add
            2 => ALIAS
                .write()
                .unwrap()
                .insert(splited[0].clone(), splited[1].clone()),
            // Error
            _ => {
                println!("Usage: alias [name=[value]]");

                Some(String::new())
            }
        };
        return super::ok_false();
    } else {
        println!("Usage: alias [name=[value]]");

        return super::ok_false();
    }

    super::ok_true()
}

pub fn builtin_alias() -> Box<BuiltinFn> {
    Box::new(|cmd| alias(cmd))
}
