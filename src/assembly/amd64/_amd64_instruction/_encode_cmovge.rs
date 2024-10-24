use crate::assembly::amd64::{AMD64Instruction, AMD64Operand};
use crate::BFResult;
use AMD64Operand::*;

impl AMD64Instruction {
    pub(crate) fn encode_cmovge(
        self: &AMD64Instruction,
        dst: &AMD64Operand,
        src: &AMD64Operand,
    ) -> BFResult<Vec<u8>> {
        match (dst, src) {
            // cmovge <reg>, <reg>
            (Register(dst_reg), Register(src_reg)) => {
                if dst_reg.size() != src_reg.size() {
                    return self.encoding_err();
                }

                if dst_reg.size() == 8 {
                    return self.encoding_err();
                }

                let prefix_reg_16 = (dst_reg.size() == 16).then_some(0x66);

                let rex = self.encode_rex(Some(dst), Some(src))?;

                let opcode: Vec<u8> = vec![0x0F, 0x4D];

                let rmi = self.encode_reg_rmi(Some(dst), Some(src), dst_reg.size())?;

                Ok(vec![prefix_reg_16, rex]
                    .into_iter()
                    .flatten()
                    .chain(opcode)
                    .chain(rmi)
                    .collect())
            }

            (_, _) => self.encoding_err(),
        }
    }
}
