use crate::assembly::amd64::{AMD64Instruction, AMD64Operand};
use crate::BFResult;
use AMD64Operand::*;

impl AMD64Instruction {
    pub(crate) fn encode_push(self: &AMD64Instruction, src: &AMD64Operand) -> BFResult<Vec<u8>> {
        match src {
            // `push <reg>`
            Register(src_reg) => {
                let prefix_16 = match src_reg.size() {
                    16 => Some(0x66_u8),
                    64 => None,
                    _ => return self.encoding_err(),
                };
                let (rex, opcode): (Option<u8>, u8) = match src_reg.id() {
                    0..8 => (None, 0x50_u8 + src_reg.id() as u8),
                    id => (Some(0x41), 0x50_u8 + (id & 7) as u8),
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
    fn test_encode_push_reg() {
        let tests: Tests = vec![
            // push ax
            (Push(Register(AX)), vec![0x66, 0x50]),
            // push rax
            (Push(Register(RAX)), vec![0x50]),
            // push cx
            (Push(Register(CX)), vec![0x66, 0x51]),
            // push rcx
            (Push(Register(RCX)), vec![0x51]),
            // push sp
            (Push(Register(SP)), vec![0x66, 0x54]),
            // push rsp
            (Push(Register(RSP)), vec![0x54]),
            // push bp
            (Push(Register(BP)), vec![0x66, 0x55]),
            // push rbp
            (Push(Register(RBP)), vec![0x55]),
            // push r8w
            (Push(Register(R8W)), vec![0x66, 0x41, 0x50]),
            // push r8
            (Push(Register(R8)), vec![0x41, 0x50]),
            // push r12w
            (Push(Register(R12W)), vec![0x66, 0x41, 0x54]),
            // push r12
            (Push(Register(R12)), vec![0x41, 0x54]),
        ];
        run_tests(tests);
    }
}
