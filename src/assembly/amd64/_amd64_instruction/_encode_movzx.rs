use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, MemorySize};
use crate::BFResult;
use AMD64Operand::*;
use MemorySize::*;

impl AMD64Instruction {
    pub(crate) fn encode_movzx(
        self: &AMD64Instruction,
        dst: &AMD64Operand,
        src: &AMD64Operand,
    ) -> BFResult<Vec<u8>> {
        match (dst, src) {
            (Register(_), Memory(size, _, _, _, _)) => {
                let Some(size) = size else {
                    return self.encoding_err();
                };
                let rex = self.encode_rex(Some(dst), Some(src))?;

                let opcode: Vec<u8> = match size {
                    Byte => Ok(vec![0x0F, 0xB6]),
                    Word => Ok(vec![0x0F, 0xB7]),
                    _ => self.encoding_err(),
                }?;

                let operand = self.encode_reg_rmi(Some(dst), Some(src), size.size())?;

                Ok(vec![rex]
                    .into_iter()
                    .flatten()
                    .chain(opcode)
                    .chain(operand)
                    .collect())
            }
            (_, _) => self.encoding_err(),
        }
    }
}
