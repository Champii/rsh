use std::env;
use std::path::Path;

use super::super::error::Error;
use super::super::exec::Program;
use super::super::parsing::CommandRaw;
use super::BuiltinFn;

fn cd(cmd: &CommandRaw) -> Result<Program, Error> {
    if cmd.args.len() > 1 {
        println!("Usage: cd [path]");

        return super::ok_false();
    }

    let root = if cmd.args.is_empty() {
        dirs::home_dir().unwrap()
    } else {
        if cmd.args[0] == "-" {
            Path::new(&env::var("OLDPWD")?).to_path_buf()
        } else {
            Path::new(&cmd.args[0].clone()).to_path_buf()
        }
    };

    if env::set_current_dir(&root).is_err() {
        println!("Cannot change dir to {}", root.to_str().unwrap());

        return super::ok_false();
    }

    let old = env::var("PWD")?;
    let current = env::current_dir().unwrap();

    env::set_var("OLDPWD", old);
    env::set_var("PWD", current);

    super::ok_true()
}

pub fn builtin_cd() -> Box<BuiltinFn> {
    Box::new(|cmd| cd(cmd))
}
