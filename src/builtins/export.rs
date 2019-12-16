use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use super::super::error::Error;
use super::super::exec::Program;
use super::super::parsing::CommandRaw;
use super::BuiltinFn;
use super::{ok_false, ok_true};

lazy_static! {
    pub static ref EXPORTS: Arc<RwLock<HashMap<String, String>>> =
        { Arc::new(RwLock::new(HashMap::new())) };
}

fn print_exports() -> Result<(), Error> {
    let export = match EXPORTS.read() {
        Ok(export) => export,
        Err(_) => return Err(Error::Mutex),
    };

    let mut v = vec![];

    for (name, export) in export.iter() {
        v.push(format!("{}={}", name, export));
    }

    v.sort();

    for export in v {
        println!("{}", export);
    }

    Ok(())
}

fn export(cmd: &CommandRaw) -> Result<Box<dyn Program>, Error> {
    if cmd.args.is_empty() {
        print_exports()?;
    } else if cmd.args.len() == 1 {
        let mut export = match EXPORTS.write() {
            Ok(export) => export,
            Err(_) => return Err(Error::Mutex),
        };

        export.remove(&cmd.args[0]);
    } else if cmd.args.len() == 2 {
        let mut val = cmd.args[1].clone();

        let first_char = match val.chars().nth(0) {
            Some(c) => c,
            None => return Err(Error::Builtin),
        };

        if first_char == '"' {
            val = val[1..val.len() - 1].to_string();
        }

        let mut export = match EXPORTS.write() {
            Ok(export) => export,
            Err(_) => return Err(Error::Mutex),
        };

        export.insert(cmd.args[0].clone(), val);
    } else {
        println!("Usage: export [name [\"value\"]]");

        return ok_false();
    }

    ok_true()
}

pub fn builtin_export() -> Box<BuiltinFn> {
    Box::new(|cmd| export(cmd))
}
