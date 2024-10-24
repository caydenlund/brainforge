use crate::assembly::amd64::AMD64Instruction;
use crate::BFResult;

impl AMD64Instruction {
    pub(crate) fn encode_jne(self: &AMD64Instruction, tgt: isize) -> BFResult<Vec<u8>> {
        todo!()
    }
}
