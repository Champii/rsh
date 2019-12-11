quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Parser(err: String)
        Lexer
        Interrupt {}
        Io(err: std::io::Error) { from() }
        Readline(err: rustyline::error::ReadlineError) { from() }
    }
}
