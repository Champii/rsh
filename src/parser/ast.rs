#[derive(Clone, Debug)]
pub struct Ast(pub Vec<Command>);

// #[derive(Clone, Debug)]
// pub enum Command {
//     Inner(CommandNext),
//     Raw(CommandRaw),
// }

#[derive(Clone, Debug)]
pub struct CommandRaw {
    pub exe: String,
    pub args: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum Command {
    Raw(CommandRaw),
    Parenthesis(Box<Command>),
    And(Box<Command>, Box<Command>),
    Or(Box<Command>, Box<Command>),
    Pipe(Box<Command>, Box<Command>),
}
