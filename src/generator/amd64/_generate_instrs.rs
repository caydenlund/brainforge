//! Assembly generation for the BF instructions for AMD64

use super::AMD64Generator;
use crate::assembly::Instruction;
use crate::instruction::IntermediateInstruction;

impl AMD64Generator {
    pub(crate) fn generate_instrs(src: &[IntermediateInstruction]) -> Vec<String> {
        let mut label_counter = 0;
        crate::assembly::amd64::AMD64Instruction::bf_to_asm_instrs(src, &mut label_counter)
            .into_iter()
            .map(|instr| instr.to_string())
            .collect::<Vec<String>>()
    }
}
