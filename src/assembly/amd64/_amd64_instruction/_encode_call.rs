use crate::assembly::amd64::AMD64Operand::*;
use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, Function, ModRM, Rex, Sib};
use crate::assembly::Instruction;

impl AMD64Instruction {
    pub(crate) fn encode_call(
        self: &AMD64Instruction,
        tgt: &Function,
    ) -> Vec<u8> {
        todo!()
    }
}
