use logos::Lexer;

use crate::lexer::Token;

mod node;
use node::*;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add(usize),
    Subtract(usize),
    Multiply(usize),
    Divide(usize),

    MoveRight(usize),
    MoveLeft(usize),
    MoveZero,

    Loop(Vec<Instruction>),

    StackPush,
    StackPop,

    Jump(JumpLocation),
}

impl Instruction {
    fn from_node(node: Node) -> Self {
        match node {
            Node::Add(i)        => Self::Add(i),
            Node::Subtract(i)   => Self::Subtract(i),
            Node::Multiply(i)   => Self::Multiply(i),
            Node::Divide(i)     => Self::Divide(i),

            Node::MoveRight(i)  => Self::MoveRight(i),
            Node::MoveLeft(i)   => Self::MoveLeft(i),
            Node::MoveZero      => Self::MoveZero,

            Node::StackPush     => Self::StackPush,
            Node::StackPop      => Self::StackPop,

            other => panic!("Node type `{:?}` should be not be converted to an instruction directly!", other),
        }
    }
}

#[derive(Debug)]
pub struct ParseError {

}

#[derive(Debug)]
pub struct InstructionTree {
    pub top_instructions: Vec<Instruction>,
}

impl InstructionTree {
    fn print_list(&self, indent: usize, instructions: &Vec<Instruction>) {
        for inst in instructions {
            println!("{} {:?}", format!("{:\t<1$}", "", indent), inst);
            if let Instruction::Loop(instructions) = inst {
                self.print_list(indent + 1, instructions);
            }
        }
    }

    pub fn debug_print(&self) {
        self.print_list(0, &self.top_instructions)
    }
}

pub struct TreeBuilder {
    node_structure: NodeStructure,
    cursor: usize,
}

impl TreeBuilder {
    fn from_node_structure(node_structure: NodeStructure) -> Self {
        Self {
            node_structure: node_structure,
            cursor: 0,
        }
    }

    fn at_end(&self) -> bool {
        self.cursor >= self.node_structure.nodes.len()
    }

    fn peek(&self) -> Option<Node> {
        self.node_structure.nodes.get(self.cursor).map(|n| n.clone())
    }

    fn at(&self, node: Node) -> bool {
        self.peek() == Some(node)
    }

    fn handle_loop(&mut self) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        'search: loop {
            if self.at_end() {
                panic!("aaaa error handling");
            }

            let node = self.peek();

            if node == Some(Node::LoopStart) {
                self.cursor += 1;
                self.handle_loop();
                continue;
            }

            if node == Some(Node::LoopEnd) {
                self.cursor += 1;
                break 'search;
            }

            if let Some(node) = node {
                self.cursor += 1;
                instructions.push(Instruction::from_node(node));
            }
        }
        instructions
    }

    fn walk(&mut self) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();

        while !self.at_end() {
            let node = self.peek();
            if node == Some(Node::LoopStart) {
                self.cursor += 1;
                instructions.push(Instruction::Loop( self.handle_loop() ));
            } else if let Some(node) = node {
                self.cursor += 1;
                instructions.push(Instruction::from_node(node));
            }
        }

        instructions
    }

    fn get_tree(mut self) -> Result<InstructionTree, ParseError> {
        let instructions = self.walk();

        Ok(InstructionTree {
            top_instructions: instructions,
        })
    }
}

/// Performs RLE and builds an InstructionTree
pub fn parse(mut lexer: Lexer<Token>) -> Result<InstructionTree, ParseError> {
    let node_structure = nodeify(lexer);

    let tree_builder = TreeBuilder::from_node_structure(node_structure);
    tree_builder.get_tree()
}
