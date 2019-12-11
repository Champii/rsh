mod ast;

use super::error::Error;

pub use ast::*;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self, input: &str) -> Result<Option<Ast>, Error> {
        if input.is_empty() {
            return Ok(None);
        }

        let line: Vec<&str> = input.split(' ').collect::<_>();

        let ast = Ast {
            0: vec![Command::Raw(CommandRaw {
                exe: line[0].to_string(),
                args: line[1..]
                    .iter()
                    .map(|x| (*x).to_string())
                    .collect::<Vec<String>>(),
            })],
        };

        Ok(Some(ast))
    }
}
