use std::collections::HashMap;
use std::process::{Child, Command};

use super::error::Error;
use super::parsing::CommandRaw;

mod alias;
mod cd;

type BuiltinFn = dyn Fn(&CommandRaw) -> Result<Child, Error>;
type Builtins = HashMap<String, Box<BuiltinFn>>;

fn ok_true() -> Result<Child, Error> {
    Command::new("true").spawn().map_err(|_| Error::Run)
}
fn ok_false() -> Result<Child, Error> {
    Command::new("false").spawn().map_err(|_| Error::Run)
}

pub fn get_builtins() -> Builtins {
    let mut builtins = HashMap::new();

    builtins.insert("cd".to_string(), cd::builtin_cd());
    builtins.insert("alias".to_string(), alias::builtin_alias());

    builtins
}