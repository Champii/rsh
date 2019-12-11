pub struct Ast(pub Vec<Command>);

pub enum Command {
    Inner(Box<CommandNext>),
    Raw(CommandRaw),
}

pub struct CommandRaw {
    pub exe: String,
    pub args: Vec<String>,
}

pub enum CommandNext {
    Parenthesis(Box<Command>),
    And(Box<Command>, Box<Command>),
    Or(Box<Command>, Box<Command>),
    Pipe(Box<Command>, Box<Command>),
}
