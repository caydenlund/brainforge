use crate::assembly::amd64::AMD64Instruction;
use crate::BFResult;

impl AMD64Instruction {
    pub(crate) fn encode_jmp(self: &AMD64Instruction, tgt: isize) -> BFResult<Vec<u8>> {
        Ok(vec![0xE9]
            .into_iter()
            .chain(self.encode_imm(tgt, 32)?)
            .collect())
    }
}
