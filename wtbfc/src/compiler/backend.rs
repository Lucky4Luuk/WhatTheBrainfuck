use cranelift::codegen::Context;
use cranelift::codegen::settings;
use cranelift::codegen::binemit::{NullRelocSink, NullTrapSink, NullStackMapSink};

use cranelift::codegen::entity::EntityRef;
use cranelift::codegen::ir::types::*;
use cranelift::codegen::ir::{AbiParam, ExternalName, Function, FuncRef, InstBuilder, Signature};
use cranelift::codegen::isa::CallConv;
use cranelift::codegen::verifier::verify_function;
use cranelift::frontend::{FunctionBuilder, FunctionBuilderContext, Variable};

use cranelift_object::*;
use cranelift_module::{Module, Linkage};

use crate::parser::{Instruction, InstructionTree};
use super::CompileTarget;
use super::block_gen::*;

pub(super) fn gen(obj_name: &str, instruction_tree: InstructionTree, target: CompileTarget) {
    let obj_builder = ObjectBuilder::new(target.isa, obj_name, cranelift_module::default_libcall_names()).expect("Failed to create an object builder!");
    let mut obj_module = ObjectModule::new(obj_builder);

    let mut func = {
        let sig = Signature::new(target.call_conv);
        Function::with_name_signature(ExternalName::user(0,0), sig)
    };
    let mut func_builder_ctx = FunctionBuilderContext::new();
    {
        let mut func_builder = FunctionBuilder::new(&mut func, &mut func_builder_ctx);

        //Blocks should only have 1 entry point, and 1 exit point.
        //Therefore, all instructions in a block, once the first one is executed, are to be
        //executed one by one, in order.
        //
        //Because of this requirement, we want to start blocks (preferably) at a label, and then
        //keep going until the first jump instruction, or until a new label starts.
        //
        //The approach I decided to use:
        //1. Create the initial block.
        //2. Walk through the code until it hits either a `Label` or a branching instruction.
        //   Store every instruction in the block, and remember the jump instruction.
        //3. Create a new block and go back until step 1, until the full code has been walked.
        //   If the block was created after a label, remember the name of the block.
        //4. Walk through every branching instruction, and handle this accordingly.
        //   We need to do this afterwards, as we need to already have created the block we want
        //   to jump to.
        //5. Seal all blocks.

        let smart_blocks = gen_smart_blocks(&instruction_tree, &mut func_builder);

        //Handle branching instructions
        for smart_block in &smart_blocks {
            if let Some(inst) = &smart_block.branch {
                if let Instruction::Jump(name) = inst {
                    func_builder.switch_to_block(smart_block.block);
                    let mut found = false;
                    'search: for search_block in &smart_blocks {
                        if let Some(label) = &search_block.label {
                            if label == name {
                                found = true;
                                func_builder.ins().jump(search_block.block, &[]);
                                break 'search;
                            }
                        }
                    }

                    if !found {
                        panic!("Found jump to non-existant label!")
                    }
                } else {
                    panic!("Branching instruction not yet supported!");
                }
            }
        }

        //Seal all the blocks, we are done!
        func_builder.seal_all_blocks();
    }
}
