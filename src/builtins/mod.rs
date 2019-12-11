use std::{
    collections::HashMap,
    process::{Child, Command},
};

use super::error::Error;
use super::parsing::CommandRaw;

pub mod alias;
mod cd;
pub mod export;

type BuiltinFn = dyn Fn(&CommandRaw) -> Result<Child, Error>;
type Builtins = HashMap<String, Box<BuiltinFn>>;

pub fn ok_true() -> Result<Child, Error> {
    Command::new("true").spawn().map_err(|_| Error::Run)
}
pub fn ok_false() -> Result<Child, Error> {
    Command::new("false").spawn().map_err(|_| Error::Run)
}

pub fn get_builtins() -> Builtins {
    let mut builtins = HashMap::new();

    builtins.insert("cd".to_string(), cd::builtin_cd());
    builtins.insert("alias".to_string(), alias::builtin_alias());
    builtins.insert("export".to_string(), export::builtin_export());

    builtins
}
