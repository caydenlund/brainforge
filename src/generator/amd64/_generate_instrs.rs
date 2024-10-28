//! Assembly generation for the BF instructions for AMD64

use crate::assembly::amd64::AMD64Instruction;
use crate::instruction::IntermediateInstruction;

/// Generates (string) assembly instructions for the given abstract BF instructions
pub(crate) fn generate_instrs(src: &[IntermediateInstruction]) -> Vec<String> {
    let mut label_counter = 0;
    src.iter()
        .map(|instr| AMD64Instruction::bf_to_assembly(instr, &mut label_counter))
        .collect::<Vec<Vec<String>>>()
        .concat()
}
