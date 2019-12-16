quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Parser(err: String)
        Lexer
        Builtin
        Run(err: String)
        Mutex
        Interrupt {}
        Io(err: std::io::Error) { from() }
        None(err: std::option::NoneError) { from() }
        Readline(err: rustyline::error::ReadlineError) { from() }
        String(err: std::string::FromUtf8Error) { from() }
    }
}
