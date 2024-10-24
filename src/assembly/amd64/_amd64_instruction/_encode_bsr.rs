use crate::assembly::amd64::{AMD64Instruction, AMD64Operand};
use crate::BFResult;

impl AMD64Instruction {
    pub(crate) fn encode_bsr(
        self: &AMD64Instruction,
        dst: &AMD64Operand,
        src: &AMD64Operand,
    ) -> BFResult<Vec<u8>> {
        todo!()
    }
}
