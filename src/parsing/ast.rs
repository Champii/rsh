#[derive(Clone, Debug)]
pub struct Ast(pub Vec<Command>);

impl Ast {
    pub fn replace_left(&mut self, cmd: Box<Command>) {
        *self.0.get_mut(0).unwrap() = *cmd;
    }
}

#[derive(Clone, Debug)]
pub struct CommandRaw {
    pub exe: String,
    pub args: Vec<String>,
}

impl CommandRaw {
    pub fn new(exe: String, args: Vec<String>) -> Self {
        Self { exe, args }
    }
}

#[derive(Clone, Debug)]
pub enum Command {
    Raw(CommandRaw),
    Parenthesis(Box<Ast>),
    And(Box<Command>, Box<Command>),
    Or(Box<Command>, Box<Command>),
    Pipe(Box<Command>, Box<Command>),
}

impl Command {
    pub fn replace_left(&mut self, cmd: Box<Command>) {
        match self {
            Self::And(ref mut left, _) => *left = cmd,
            Self::Or(ref mut left, _) => *left = cmd,
            Self::Pipe(ref mut left, _) => *left = cmd,
            Self::Parenthesis(ref mut left) => left.replace_left(cmd),
            _ => (),
        }
    }
}
