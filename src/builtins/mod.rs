use std::collections::HashMap;

use super::error::Error;
use super::exec::Program;
use super::parsing::CommandRaw;

pub mod alias;
mod cd;
pub mod export;

use super::exec::{ok_false, ok_true};

type BuiltinFn = dyn Fn(&CommandRaw) -> Result<Program, Error>;
type Builtins = HashMap<String, Box<BuiltinFn>>;

pub fn get_builtins() -> Builtins {
    let mut builtins = HashMap::new();

    builtins.insert("cd".to_string(), cd::builtin_cd());
    builtins.insert("alias".to_string(), alias::builtin_alias());
    builtins.insert("export".to_string(), export::builtin_export());

    builtins
}
