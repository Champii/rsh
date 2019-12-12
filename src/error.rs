quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Parser(err: String)
        Lexer
        Builtin
        Run
        Mutex
        Interrupt {}
        Io(err: std::io::Error) { from() }
        Readline(err: rustyline::error::ReadlineError) { from() }
        String(err: std::string::FromUtf8Error) { from() }
    }
}
