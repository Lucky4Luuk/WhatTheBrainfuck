use std::collections::HashMap;

use logos::Lexer;

use crate::lexer::Token;

pub type NodeCount = usize;
pub type JumpLocation = usize;
pub type JumpIdentifier = String;

#[derive(Debug, PartialEq, Clone)]
pub(super) enum Node {
    Add(NodeCount),
    Subtract(NodeCount),
    Multiply(NodeCount),
    Divide(NodeCount),

    MoveRight(NodeCount),
    MoveLeft(NodeCount),
    MoveZero,

    LoopStart,
    LoopEnd,

    StackPush,
    StackPop,

    Jump(JumpIdentifier),

    Error,
}

impl Node {
    fn from_token(token: Token) -> Self {
        match token {
            Token::Add          => Self::Add(1),
            Token::Subtract     => Self::Subtract(1),
            Token::Multiply     => Self::Multiply(1),
            Token::Divide       => Self::Divide(1),

            Token::MoveRight    => Self::MoveRight(1),
            Token::MoveLeft     => Self::MoveLeft(1),
            Token::MoveZero     => Self::MoveZero,

            Token::LoopLeft     => Self::LoopStart,
            Token::LoopRight    => Self::LoopEnd,

            Token::StackPush    => Self::StackPush,
            Token::StackPop     => Self::StackPop,

            Token::Jump         => panic!("Jump token cannot be directly transformed into a node!"),

            _ => Self::Error,
        }
    }

    fn eq_token(&self, token: Token) -> bool {
        match self {
            Self::Add(_) => token == Token::Add,
            Self::Subtract(_) => token == Token::Subtract,
            Self::Multiply(_) => token == Token::Multiply,
            Self::Divide(_) => token == Token::Divide,

            Self::MoveRight(_) => token == Token::MoveRight,
            Self::MoveLeft(_) => token == Token::MoveLeft,

            Self::LoopStart => token == Token::LoopLeft,
            Self::LoopEnd => token == Token::LoopRight,

            _ => false,
        }
    }

    fn increment(&mut self) {
        match self {
            Self::Add(i) => *i+=1,
            Self::Subtract(i) => *i+=1,
            Self::Multiply(i) => *i+=1,
            Self::Divide(i) => *i+=1,

            Self::MoveRight(i) => *i+=1,
            Self::MoveLeft(i) => *i+=1,

            _ => panic!("Cannot increment this Node's count!"),
        }
    }
}

pub(super) struct NodeStructure {
    pub nodes: Vec<Node>,
    pub label_map: HashMap<String, JumpLocation>,
}

/// Performs basic RLE as well
pub(super) fn nodeify(mut lexer: Lexer<Token>) -> NodeStructure {
    let mut nodes: Vec<Node> = Vec::new();
    let mut label_map: HashMap<String, JumpLocation> = HashMap::new();

    while let Some(token) = lexer.next() {
        match token {
            Token::Add | Token::Subtract | Token::Multiply | Token::Divide | Token::MoveRight | Token::MoveLeft => {
                if let Some(last) = nodes.last_mut() {
                    if last.eq_token(token.clone()) {
                        last.increment();
                    } else {
                        nodes.push(Node::from_token(token));
                    }
                } else {
                    nodes.push(Node::from_token(token));
                }
            },
            Token::MoveZero => {
                if let Some(last) = nodes.last_mut() {
                    // If there is already a MoveZero node, we don't need a 2nd one, as it would do nothing.
                    if *last != Node::MoveZero {
                        nodes.push(Node::MoveZero);
                    }
                } else {
                    nodes.push(Node::MoveZero);
                }
            },
            Token::Label => {
                let loc = nodes.len();
                let mut chars = lexer.slice().chars();
                chars.next();
                let name = chars.as_str();
                label_map.insert(name.to_string(), loc);
            },
            Token::Jump => {
                let mut chars = lexer.slice().chars();
                chars.next();
                let name = chars.as_str();
                nodes.push(Node::Jump(name.to_string()));
            },
            _ => nodes.push(Node::from_token(token)),
        }
    }

    NodeStructure {
        nodes: nodes,
        label_map: label_map,
    }
}
