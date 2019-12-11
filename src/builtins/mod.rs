use super::error::Error;
use super::parsing::CommandRaw;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process::{Child, Command};

type BuiltinFn = dyn Fn(&CommandRaw) -> Result<Child, Error>;
type Builtins = HashMap<String, Box<BuiltinFn>>;

fn ok_true() -> Result<Child, Error> {
    Command::new("true").spawn().map_err(|_| Error::Run)
}
fn ok_false() -> Result<Child, Error> {
    Command::new("false").spawn().map_err(|_| Error::Run)
}

fn cd(cmd: &CommandRaw) -> Result<Child, Error> {
    if cmd.args.len() > 1 {
        println!("Usage: cd [path]");

        return ok_false();
    }
    let arg = cmd.args[0].clone();

    let root = Path::new(&arg);

    if env::set_current_dir(&root).is_err() {
        println!("Cannot change dir to {}", arg);

        return ok_false();
    }

    ok_true()
}

fn builtin_cd() -> Box<BuiltinFn> {
    Box::new(|cmd| cd(cmd))
}

pub fn get_builtins() -> Builtins {
    let mut builtins = HashMap::new();

    builtins.insert("cd".to_string(), builtin_cd());

    builtins
}
