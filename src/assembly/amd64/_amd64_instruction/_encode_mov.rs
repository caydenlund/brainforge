use crate::assembly::amd64::AMD64Operand::*;
use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, ModRM, Rex};

impl AMD64Instruction {
    pub(crate) fn encode_mov(
        self: &AMD64Instruction,
        dst: &AMD64Operand,
        src: &AMD64Operand,
    ) -> Vec<u8> {
        match (dst, src) {
            // reg = reg
            (Register(dst_reg), Register(src_reg)) => {
                let mut rex = Rex::new();
                rex.r_reg(src_reg);
                rex.b_reg(dst_reg);

                let mut mod_rm = ModRM::new();
                mod_rm.mode(3);
                mod_rm.reg_reg(src_reg);
                mod_rm.rm_reg(dst_reg);

                let mut result = vec![];
                if rex.is_some() {
                    result.push(rex.as_byte().unwrap());
                }
                result.push(0x89);
                result.push(mod_rm.as_byte());

                result
            }
            _ => todo!(),
        }
    }
}
