use crate::assembly::amd64::AMD64Operand::*;
use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, ModRM, Rex, Sib};
use crate::assembly::Instruction;

impl AMD64Instruction {
    pub(crate) fn encode_vmovdqu(
        self: &AMD64Instruction,
        dst: &AMD64Operand,
        src: &AMD64Operand,
    ) -> Vec<u8> {
        todo!()
    }
}
