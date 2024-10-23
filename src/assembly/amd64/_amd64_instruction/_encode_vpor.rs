use crate::assembly::amd64::AMD64Operand::*;
use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, ModRM, Rex, Sib};
use crate::assembly::Instruction;

impl AMD64Instruction {
    pub(crate) fn encode_vpor(
        self: &AMD64Instruction,
        dst: &AMD64Operand,
        op1: &AMD64Operand,
        op2: &AMD64Operand,
    ) -> Vec<u8> {
        todo!()
    }
}
