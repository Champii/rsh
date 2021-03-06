use logos::Lexer;
use logos::Logos;

use super::super::error::Error;
use super::ast::*;
use super::lexer::Token;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self, input: &str) -> Result<Option<Ast>, Error> {
        if input.is_empty() {
            return Ok(None);
        }

        let mut lexer = Token::lexer(input);
        // let mut tmp = Token::lexer(input);

        // while tmp.token != Token::End {
        //     println!("TOKEN {:?}", tmp.token);

        //     tmp.advance();
        // }

        let ast = self.parse_ast(&mut lexer)?;

        // println!("AST {:#?}", ast);

        Ok(Some(ast))
    }

    pub fn parse_ast(&self, lexer: &mut Lexer<Token, &str>) -> Result<Ast, Error> {
        let mut cmds = vec![];

        while lexer.token != Token::End {
            cmds.push(self.parse_command(lexer)?);

            if lexer.token == Token::SemiColon {
                lexer.advance();
            } else {
                break;
            }
        }

        Ok(Ast { 0: cmds })
    }

    pub fn parse_raw(lexer: &mut Lexer<Token, &str>) -> Result<CommandRaw, Error> {
        let mut items = vec![];

        if lexer.token != Token::Text && lexer.token != Token::SubExec {
            return Err(Error::Parser("Expected command".to_string()));
        }

        while lexer.token == Token::Text
            || lexer.token == Token::String
            || lexer.token == Token::SubExec
        {
            let mut val = lexer.slice();

            if lexer.token == Token::String {
                val = &val[1..val.len() - 1];
            }

            let to_push = match unescape::unescape(val) {
                Some(val) => val,
                None => val.to_string(),
            };

            items.push(to_push);

            lexer.advance();
        }

        Ok(CommandRaw {
            exe: items[0].to_string(),
            args: items[1..]
                .iter()
                .map(|x| (*x).to_string())
                .collect::<Vec<String>>(),
        })
    }

    pub fn parse_command(&self, lexer: &mut Lexer<Token, &str>) -> Result<Command, Error> {
        let left = if lexer.token == Token::ParensOpen {
            lexer.advance();

            let res = Command::Parenthesis(Box::new(self.parse_ast(lexer)?));

            if lexer.token != Token::ParensClose {
                return Err(Error::Parser("Expected close parenthesis".to_string()));
            }

            lexer.advance();

            res
        } else {
            Command::Raw(Self::parse_raw(lexer)?)
        };

        Ok(match lexer.token {
            Token::DoubleAnd => {
                lexer.advance();

                let right = self.parse_command(lexer)?;

                Command::And(Box::new(left), Box::new(right))
            }
            Token::DoublePipe => {
                lexer.advance();

                let right = self.parse_command(lexer)?;

                Command::Or(Box::new(left), Box::new(right))
            }
            Token::Pipe => {
                lexer.advance();

                let right = self.parse_command(lexer)?;

                Command::Pipe(Box::new(left), Box::new(right))
            }
            _ => left,
        })
    }
}
