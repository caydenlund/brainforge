use crate::assembly::amd64::{AMD64Instruction, AMD64Operand};
use crate::BFResult;
use AMD64Operand::*;

impl AMD64Instruction {
    pub(crate) fn encode_imul(
        self: &AMD64Instruction,
        dst: &AMD64Operand,
        src: &AMD64Operand,
    ) -> BFResult<Vec<u8>> {
        match (dst, src) {
            (Register(dst_reg), Immediate(imm)) => {
                if dst_reg.size() == 8 {
                    return self.encoding_err();
                }

                let prefix_reg_16 = (dst_reg.size() == 16).then_some(0x66);

                let rex = self.encode_rex(Some(dst), Some(dst))?;

                let (opcode, imm_size): (u8, usize) = match imm {
                    -0x80..0x80 => (0x6B, 8),
                    _ => (0x69, dst_reg.size().min(32)),
                };
                let imm = self.encode_imm(*imm, imm_size)?;
                let operand = self.encode_reg_rmi(Some(dst), Some(dst), 0)?;

                Ok(vec![prefix_reg_16, rex, Some(opcode)]
                    .into_iter()
                    .flatten()
                    .chain(operand)
                    .chain(imm)
                    .collect())
            }
            (_, _) => self.encoding_err(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, AMD64Register};
    use crate::assembly::Instruction;

    use AMD64Instruction::*;
    use AMD64Operand::*;
    use AMD64Register::*;

    type Tests = Vec<(AMD64Instruction, Vec<u8>)>;

    fn run_tests(tests: Tests) {
        for (instruction, expected) in tests {
            match instruction.to_binary() {
                Ok(actual) => assert_eq!(actual, expected, "{}", instruction.to_string()),
                Err(err) => panic!("{:?}", err),
            }
        }
    }

    #[test]
    fn test_encode_imul_reg_imm_sizes() {
        let tests: Tests = vec![
            // imul cx, 0x11
            (
                Imul(Register(CX), Immediate(0x11)),
                vec![0x66, 0x6B, 0xC9, 0x11],
            ),
            // imul ecx, 0x11
            (Imul(Register(ECX), Immediate(0x11)), vec![0x6B, 0xC9, 0x11]),
            // imul rcx, 0x11
            (
                Imul(Register(RCX), Immediate(0x11)),
                vec![0x48, 0x6B, 0xC9, 0x11],
            ),
            // imul cx, 0x44332211
            (
                Imul(Register(CX), Immediate(0x44332211)),
                vec![0x66, 0x69, 0xC9, 0x11, 0x22],
            ),
            // imul ecx, 0x44332211
            (
                Imul(Register(ECX), Immediate(0x44332211)),
                vec![0x69, 0xC9, 0x11, 0x22, 0x33, 0x44],
            ),
            // imul rcx, 0x44332211
            (
                Imul(Register(RCX), Immediate(0x44332211)),
                vec![0x48, 0x69, 0xC9, 0x11, 0x22, 0x33, 0x44],
            ),
        ];
        run_tests(tests);
    }

    #[test]
    fn test_encode_imul_reg_imm_regs() {
        let tests: Tests = vec![
            // imul eax, 0x11
            (Imul(Register(EAX), Immediate(0x11)), vec![0x6B, 0xC0, 0x11]),
            // imul ecx, 0x11
            (Imul(Register(ECX), Immediate(0x11)), vec![0x6B, 0xC9, 0x11]),
            // imul esp, 0x11
            (Imul(Register(ESP), Immediate(0x11)), vec![0x6B, 0xE4, 0x11]),
            // imul ebp, 0x11
            (Imul(Register(EBP), Immediate(0x11)), vec![0x6B, 0xED, 0x11]),
            // imul r12d, 0x11
            (
                Imul(Register(R12D), Immediate(0x11)),
                vec![0x45, 0x6B, 0xE4, 0x11],
            ),
            // imul r13d, 0x11
            (
                Imul(Register(R13D), Immediate(0x11)),
                vec![0x45, 0x6B, 0xED, 0x11],
            ),
            // imul eax, 0x44332211
            (
                Imul(Register(EAX), Immediate(0x44332211)),
                vec![0x69, 0xC0, 0x11, 0x22, 0x33, 0x44],
            ),
            // imul ecx, 0x44332211
            (
                Imul(Register(ECX), Immediate(0x44332211)),
                vec![0x69, 0xC9, 0x11, 0x22, 0x33, 0x44],
            ),
            // imul esp, 0x44332211
            (
                Imul(Register(ESP), Immediate(0x44332211)),
                vec![0x69, 0xE4, 0x11, 0x22, 0x33, 0x44],
            ),
            // imul ebp, 0x44332211
            (
                Imul(Register(EBP), Immediate(0x44332211)),
                vec![0x69, 0xED, 0x11, 0x22, 0x33, 0x44],
            ),
            // imul r12d, 0x44332211
            (
                Imul(Register(R12D), Immediate(0x44332211)),
                vec![0x45, 0x69, 0xE4, 0x11, 0x22, 0x33, 0x44],
            ),
            // imul r13d, 0x44332211
            (
                Imul(Register(R13D), Immediate(0x44332211)),
                vec![0x45, 0x69, 0xED, 0x11, 0x22, 0x33, 0x44],
            ),
        ];
        run_tests(tests);
    }
}
