use crate::assembly::amd64::AMD64Instruction;
use crate::BFResult;

impl AMD64Instruction {
    pub(crate) fn encode_je(self: &AMD64Instruction, tgt: isize) -> BFResult<Vec<u8>> {
        Ok(vec![0x0F, 0x84]
            .into_iter()
            .chain(self.encode_imm(tgt, 32)?)
            .collect())
    }
}
