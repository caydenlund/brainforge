use crate::assembly::amd64::AMD64Operand::*;
use crate::assembly::amd64::{AMD64Instruction, AMD64Operand};
use crate::assembly::Instruction;

impl AMD64Instruction {
    pub(crate) fn encode_xor(
        self: &AMD64Instruction,
        dst: &AMD64Operand,
        src: &AMD64Operand,
    ) -> Vec<u8> {
        match (dst, src) {
            (Register(dst_reg), Register(src_reg)) => {
                self.check_reg_size(dst_reg, src_reg);

                let prefix_reg_16 = (dst_reg.size() == 16).then_some(0x66);

                let rex = self.unwrap(Self::encode_rex(Some(src), Some(dst)));

                let opcode: u8 = if dst_reg.size() == 8 { 0x30 } else { 0x31 };

                let rmi = self.unwrap(Self::encode_reg_rmi(Some(src), Some(dst), dst_reg.size()));

                vec![prefix_reg_16, rex, Some(opcode)]
                    .into_iter()
                    .flatten()
                    .chain(rmi)
                    .collect()
            }
            (_, _) => panic!("Invalid instruction: `{}`", self.to_string()),
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

    fn run_tests(tests: &Tests) {
        for (instruction, expected) in tests {
            assert_eq!(
                instruction.to_binary(),
                *expected,
                "{}",
                instruction.to_string()
            );
        }
    }

    #[test]
    fn test_encode_xor_reg_reg() {
        let tests: Tests = vec![
            (Xor(Register(CL), Register(CL)), vec![0x30, 0xC9]),
            (Xor(Register(CX), Register(CX)), vec![0x66, 0x31, 0xC9]),
            (Xor(Register(RCX), Register(RCX)), vec![0x48, 0x31, 0xC9]),
            //
            (Xor(Register(ECX), Register(ECX)), vec![0x31, 0xC9]),
            (Xor(Register(ECX), Register(ESP)), vec![0x31, 0xE1]),
            (Xor(Register(ECX), Register(EBP)), vec![0x31, 0xE9]),
            (Xor(Register(ECX), Register(R12D)), vec![0x44, 0x31, 0xE1]),
            //
            (Xor(Register(ESP), Register(ECX)), vec![0x31, 0xCC]),
            (Xor(Register(ESP), Register(ESP)), vec![0x31, 0xE4]),
            (Xor(Register(ESP), Register(EBP)), vec![0x31, 0xEC]),
            (Xor(Register(ESP), Register(R12D)), vec![0x44, 0x31, 0xE4]),
            //
            (Xor(Register(EBP), Register(ECX)), vec![0x31, 0xCD]),
            (Xor(Register(EBP), Register(ESP)), vec![0x31, 0xE5]),
            (Xor(Register(EBP), Register(EBP)), vec![0x31, 0xED]),
            (Xor(Register(EBP), Register(R12D)), vec![0x44, 0x31, 0xE5]),
            //
            (Xor(Register(R12D), Register(ECX)), vec![0x41, 0x31, 0xCC]),
            (Xor(Register(R12D), Register(ESP)), vec![0x41, 0x31, 0xE4]),
            (Xor(Register(R12D), Register(EBP)), vec![0x41, 0x31, 0xEC]),
            (Xor(Register(R12D), Register(R12D)), vec![0x45, 0x31, 0xE4]),
        ];
        run_tests(&tests);
    }
}
