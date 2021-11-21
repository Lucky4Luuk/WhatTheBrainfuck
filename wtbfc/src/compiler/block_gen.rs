use cranelift::prelude::FunctionBuilder;
use cranelift::prelude::Block;

use crate::parser::{Instruction, InstructionTree};

fn is_branching(inst: &Instruction) -> bool {
    match inst {
        Instruction::Jump(_) => true,
        Instruction::Loop(_) => true,
        _ => false,
    }
}

pub struct SmartBlock {
    /// The actual block.
    pub block: Block,
    /// The label of the block.
    pub label: Option<String>,
    /// The branching instruction that ends this block.
    /// If None, this block has no branching instruction ending this block.
    /// This does not mean the program exits here.
    pub branch: Option<Instruction>,
}

pub fn gen_smart_blocks(instruction_tree: &InstructionTree, builder: &mut FunctionBuilder) -> Vec<SmartBlock> {
    let mut smart_blocks = Vec::new();
    smart_blocks.push(SmartBlock {
        block: builder.create_block(),
        label: None,
        branch: None,
    });
    builder.switch_to_block(smart_blocks.last_mut().unwrap().block);

    for inst in &instruction_tree.top_instructions {
        if is_branching(inst) {
            //Ends the block
            smart_blocks.last_mut().unwrap().branch = Some(inst.clone());
            smart_blocks.push(SmartBlock {
                block: builder.create_block(),
                label: None,
                branch: None,
            });
        } else {
            if let Instruction::Label(name) = inst {
                //Ends the block
                smart_blocks.push(SmartBlock {
                    block: builder.create_block(),
                    label: Some(name.clone()),
                    branch: None,
                });
                builder.switch_to_block(smart_blocks.last_mut().unwrap().block);
            } else {
                //We have found a "normal" instruction!
                //Finally, we get to add something to our block.
                //What, exactly? That's not for this function to handle :)
                // super::instruction::gen(inst, builder);
            }
        }
    }

    smart_blocks
}
