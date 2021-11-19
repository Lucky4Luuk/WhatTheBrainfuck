use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token("+")]
    Add,
    #[token("-")]
    Subtract,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,

    #[token(">")]
    MoveRight,
    #[token("<")]
    MoveLeft,
    #[token("#")]
    MoveZero,

    #[token("[")]
    LoopLeft,
    #[token("]")]
    LoopRight,

    #[token(":")]
    StackPush,
    #[token(";")]
    StackPop,

    #[regex(r"[\^][a-zA-Z_]+")]
    Label,
    #[regex(r"[@][a-zA-Z_]+")]
    Jump,

    #[regex(r"[ \t\n\r\f]+", logos::skip)]
    Whitespace,

    #[error]
    Error,
}
