use crate::assembly::amd64::AMD64Operand::*;
use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, ModRM, Rex, Sib};
use crate::assembly::Instruction;

impl AMD64Instruction {
    pub(crate) fn encode_jmp(
        self: &AMD64Instruction,
        tgt: isize,
    ) -> Vec<u8> {
        todo!()
    }
}
