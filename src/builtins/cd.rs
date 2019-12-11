use std::env;
use std::path::Path;
use std::process::Child;

use super::super::error::Error;
use super::super::parsing::CommandRaw;
use super::BuiltinFn;

fn cd(cmd: &CommandRaw) -> Result<Child, Error> {
    if cmd.args.len() > 1 {
        println!("Usage: cd [path]");

        return super::ok_false();
    }
    let arg = cmd.args[0].clone();

    let root = Path::new(&arg);

    if env::set_current_dir(&root).is_err() {
        println!("Cannot change dir to {}", arg);

        return super::ok_false();
    }

    super::ok_true()
}

pub fn builtin_cd() -> Box<BuiltinFn> {
    Box::new(|cmd| cd(cmd))
}
