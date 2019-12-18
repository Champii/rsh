use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[end]
    End,
    #[error]
    Error,

    #[token = "="]
    Equal,
    #[token = "||"]
    DoublePipe,
    #[token = "|"]
    Pipe,
    #[token = "&&"]
    DoubleAnd,
    #[token = "&"]
    And,
    #[token = "("]
    ParensOpen,
    #[token = ")"]
    ParensClose,
    #[token = "<<"]
    DoubleRedirLeft,
    #[token = "<"]
    RedirLeft,
    #[token = ">>"]
    DoubleRedirRight,
    #[token = ">"]
    RedirRight,
    #[token = ";"]
    SemiColon,

    #[regex = "#.*"]
    Comment,

    #[regex = r#"`([^`\\]|\\[tu]|\\)*`"#]
    SubExec,

    #[regex = r#""([^"\\]|\\[tu]|\\)*""#]
    String,

    // #[regex = "[0-9a-zA-Z\\$_\\-/\\.\\?\\{\\}=\\(\\)\\+\\[\\]:\\\\]+"]
    #[regex = "[0-9a-zA-Z\\$_\\-/\\.\\?\\{\\}=\\+\\[\\]:\\\\]+"]
    Text,
}
