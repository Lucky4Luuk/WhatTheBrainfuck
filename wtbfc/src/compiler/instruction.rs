use cranelift::prelude::FunctionBuilder;
use cranelift::prelude::Block;
use cranelift::codegen::ir::types::*;
use cranelift::codegen::ir::{AbiParam, ExternalName, Function, InstBuilder, Signature};

use crate::parser::Instruction;

pub fn gen(inst: &Instruction, builder: &mut FunctionBuilder) {
    match inst {
        Instruction::Add(i) => add(builder, *i),
        Instruction::Subtract(i) => subtract(builder, *i),
        Instruction::Multiply(i) => multiply(builder, *i),
        Instruction::Divide(i) => divide(builder, *i),

        Instruction::MoveLeft(i) => move_left(builder, *i),
        Instruction::MoveRight(i) => move_right(builder, *i),
        Instruction::MoveZero => move_zero(builder),

        Instruction::StackPush => stack_push(builder),
        Instruction::StackPop => stack_pop(builder),

        Instruction::Loop(_) => panic!("Loop instructions are not to be run through this function!"),
        _ => panic!("Instruction not yet implemented!"),
    }
}

fn add(builder: &mut FunctionBuilder, count: usize) {
    // builder.ins().iadd(); //Something to do with this
    todo!();
}

fn subtract(builder: &mut FunctionBuilder, count: usize) {
    todo!();
}

fn multiply(builder: &mut FunctionBuilder, count: usize) {
    todo!();
}

fn divide(builder: &mut FunctionBuilder, count: usize) {
    todo!();
}

fn move_left(builder: &mut FunctionBuilder, count: usize) {
    todo!();
}

fn move_right(builder: &mut FunctionBuilder, count: usize) {
    todo!();
}

fn move_zero(builder: &mut FunctionBuilder) {
    todo!();
}

fn stack_push(builder: &mut FunctionBuilder) {
    todo!();
}

fn stack_pop(builder: &mut FunctionBuilder) {
    todo!();
}
