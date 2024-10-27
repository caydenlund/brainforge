use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, AMD64Register, MemorySize, Vex};
use crate::BFResult;
use AMD64Operand::*;
use AMD64Register::*;
use MemorySize::*;

impl AMD64Instruction {
    pub(crate) fn encode_vmovdqu(
        self: &AMD64Instruction,
        dst: &AMD64Operand,
        src: &AMD64Operand,
    ) -> BFResult<Vec<u8>> {
        match (dst, src) {
            (Register(dst_reg), Memory(size, base_reg, index_reg, index_scale, displacement)) => {
                if let Some(size) = size {
                    if size.size() != YMMWord.size() {
                        return self.encoding_err();
                    }
                }
                if dst_reg.size() != YMMWord.size() {
                    return self.encoding_err();
                }

                let vex = Vex::new();
                // vex.b = 
                // VEX.256.F3.0F.WIG 6F /r
                // c4 c1 7e 6f 1c 24
                // c1: 1 1 0 00001
                // b

                // 7e: 0 1111 1 10
                // l, 0xF3
                todo!()
            }
            (_, _) => self.encoding_err(),
        }
    }
}
