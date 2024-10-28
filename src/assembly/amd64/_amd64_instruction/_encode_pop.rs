use crate::assembly::amd64::AMD64Operand::Register;
use crate::assembly::amd64::{AMD64Instruction, AMD64Operand};
use crate::BFResult;

impl AMD64Instruction {
    pub(crate) fn encode_pop(self: &AMD64Instruction, src: &AMD64Operand) -> BFResult<Vec<u8>> {
        match src {
            // `pop <reg>`
            Register(src_reg) => {
                let prefix_16 = match src_reg.size() {
                    16 => Some(0x66_u8),
                    64 => None,
                    _ => return self.encoding_err(),
                };
                let (rex, opcode): (Option<u8>, u8) = match src_reg.id() {
                    0..8 => (None, 0x58_u8 + src_reg.id() as u8),
                    id => (Some(0x41), 0x58_u8 + (id & 7) as u8),
                };

                Ok(vec![prefix_16, rex, Some(opcode)]
                    .into_iter()
                    .flatten()
                    .collect())
            }

            _ => self.encoding_err(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, AMD64Register};

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
    fn test_encode_pop_reg() {
        let tests: Tests = vec![
            // pop ax
            (Pop(Register(AX)), vec![0x66, 0x58]),
            // pop rax
            (Pop(Register(RAX)), vec![0x58]),
            // pop cx
            (Pop(Register(CX)), vec![0x66, 0x59]),
            // pop rcx
            (Pop(Register(RCX)), vec![0x59]),
            // pop sp
            (Pop(Register(SP)), vec![0x66, 0x5C]),
            // pop rsp
            (Pop(Register(RSP)), vec![0x5C]),
            // pop bp
            (Pop(Register(BP)), vec![0x66, 0x5D]),
            // pop rbp
            (Pop(Register(RBP)), vec![0x5D]),
            // pop r8w
            (Pop(Register(R8W)), vec![0x66, 0x41, 0x58]),
            // pop r8
            (Pop(Register(R8)), vec![0x41, 0x58]),
            // pop r12w
            (Pop(Register(R12W)), vec![0x66, 0x41, 0x5C]),
            // pop r12
            (Pop(Register(R12)), vec![0x41, 0x5C]),
        ];
        run_tests(tests);
    }
}
