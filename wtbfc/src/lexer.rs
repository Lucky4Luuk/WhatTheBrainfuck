use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Instruction {
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

    #[token("[")]
    LoopLeft,
    #[token("]")]
    LoopRight,

    #[token(":")]
    StackPush,
    #[token(";")]
    StackPop,

    #[regex(r"[\^][a-zA-Z][a-zA-Z]*")]
    Label,
    #[regex(r"[@][a-zA-Z][a-zA-Z]*")]
    Jump,

    #[error]
    Error,
}
