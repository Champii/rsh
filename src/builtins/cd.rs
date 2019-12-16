use std::env;
use std::path::Path;

use super::super::error::Error;
use super::super::exec::Program;
use super::super::parsing::CommandRaw;
use super::BuiltinFn;

fn cd(cmd: &CommandRaw) -> Result<Box<dyn Program>, Error> {
    if cmd.args.len() > 1 {
        println!("Usage: cd [path]");

        return super::ok_false();
    } else if cmd.args.is_empty() {
        let arg = std::env::home_dir().unwrap();

        let root = Path::new(&arg);

        if env::set_current_dir(&root).is_err() {
            println!("Cannot change dir to {}", arg.to_str().unwrap());

            return super::ok_false();
        }
    } else {
        let arg = cmd.args[0].clone();

        let root = Path::new(&arg);

        if env::set_current_dir(&root).is_err() {
            println!("Cannot change dir to {}", arg);

            return super::ok_false();
        }
    }

    super::ok_true()
}

pub fn builtin_cd() -> Box<BuiltinFn> {
    Box::new(|cmd| cd(cmd))
}
