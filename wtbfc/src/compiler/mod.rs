use cranelift::prelude::settings;
use cranelift::prelude::isa::{self, TargetIsa, CallConv};
use cranelift::prelude::Configurable;
pub use target_lexicon::Triple; //Re-export for use in the frontend

use crate::parser::InstructionTree;

mod backend;
pub(super) mod block_gen;
pub(super) mod instruction;

pub struct CompileTarget {
    pub triple: Triple,
    pub isa: Box<dyn TargetIsa>,
    pub call_conv: CallConv,
}

impl CompileTarget {
    pub fn from_triple(flags: settings::Flags, triple: Triple) -> Self {
        let isa = match cranelift::codegen::isa::lookup(triple.clone()) {
            Err(_) => {
                panic!("The target ISA is not available!")
            }
            Ok(isa_builder) => {
                // isa_builder.set("use_popcnt", "on");
                isa_builder.finish(flags.clone())
            }
        };
        Self {
            call_conv: CallConv::triple_default(&triple),
            isa: isa,
            triple: triple,
        }
    }
}

pub fn compile(obj_name: &str, instruction_tree: InstructionTree, triple: Triple) {
    let flags = settings::Flags::new(settings::builder());

    let target = CompileTarget::from_triple(flags, triple);

    backend::gen(obj_name, instruction_tree, target)
}
