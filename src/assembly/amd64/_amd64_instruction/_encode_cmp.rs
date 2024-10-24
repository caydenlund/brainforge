use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, AMD64Register};
use crate::BFResult;

use AMD64Operand::*;

impl AMD64Instruction {
    pub(crate) fn encode_cmp(
        self: &AMD64Instruction,
        dst: &AMD64Operand,
        src: &AMD64Operand,
    ) -> BFResult<Vec<u8>> {
        match (dst, src) {
            // cmp <mem>, <imm>
            (Memory(size, base_reg, index_reg, _, _), Immediate(imm)) => {
                let Some(size) = size else {
                    return self.encoding_err();
                };

                let prefix_reg_16 = (size.size() == 16).then_some(0x66);

                let prefix_addr_32 = self.encode_prefix_addr_32(base_reg, index_reg)?;

                let rex = self.encode_rex(None, Some(dst))?;

                let rmi = self.encode_reg_rmi(
                    Some(&Register(AMD64Register::RDI)),
                    Some(dst),
                    size.size(),
                )?;

                let (opcode, imm): (u8, Vec<u8>) = match (size.size(), *imm) {
                    (8, _) => (0x80, self.encode_imm(*imm, 8)?),
                    (_, -0x80..0x80) => (0x83, self.encode_imm(*imm, 8)?),
                    (_, _) => (0x81, self.encode_imm(*imm, size.size().min(32))?),
                };

                Ok(vec![prefix_reg_16, prefix_addr_32, rex, Some(opcode)]
                    .into_iter()
                    .flatten()
                    .chain(rmi)
                    .chain(imm)
                    .collect())
            }

            // cmp <reg>, <imm>
            (Register(dst_reg), Immediate(imm)) => {
                let prefix_reg_16 = (dst_reg.size() == 16).then_some(0x66);

                let rex = self.encode_rex(None, Some(dst))?;

                let rmi = self.encode_reg_rmi(
                    Some(&Register(AMD64Register::RDI)),
                    Some(dst),
                    dst_reg.size(),
                )?;

                let (opcode, imm): (u8, Vec<u8>) = match (dst_reg.size(), *imm) {
                    (8, _) => (0x80, self.encode_imm(*imm, 8)?),
                    (_, -0x80..0x80) => (0x83, self.encode_imm(*imm, 8)?),
                    (_, _) => (0x81, self.encode_imm(*imm, dst_reg.size().min(32))?),
                };

                Ok(vec![prefix_reg_16, rex, Some(opcode)]
                    .into_iter()
                    .flatten()
                    .chain(rmi)
                    .chain(imm)
                    .collect())
            }

            (_, _) => todo!(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, AMD64Register, MemorySize};
    use crate::assembly::Instruction;

    use AMD64Instruction::*;
    use AMD64Operand::*;
    use AMD64Register::*;
    use MemorySize::*;

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
    fn test_encode_cmp_mem_imm() {
        let tests: Tests = vec![
            (
                Cmp(
                    Memory(Some(Byte), Some(RCX), None, None, None),
                    Immediate(0x00),
                ),
                vec![0x80, 0x39, 0x00],
            ),
            (
                Cmp(
                    Memory(Some(Byte), Some(RCX), None, None, None),
                    Immediate(0x1100),
                ),
                vec![0x80, 0x39, 0x00],
            ),
            (
                Cmp(
                    Memory(Some(Word), Some(RCX), None, None, None),
                    Immediate(0x00),
                ),
                vec![0x66, 0x83, 0x39, 0x00],
            ),
            (
                Cmp(
                    Memory(Some(Word), Some(RCX), None, None, None),
                    Immediate(0x1100),
                ),
                vec![0x66, 0x81, 0x39, 0x00, 0x11],
            ),
            (
                Cmp(
                    Memory(Some(Word), Some(RCX), None, None, None),
                    Immediate(0x221100),
                ),
                vec![0x66, 0x81, 0x39, 0x00, 0x11],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(RCX), None, None, None),
                    Immediate(0x00),
                ),
                vec![0x83, 0x39, 0x00],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(RCX), None, None, None),
                    Immediate(0x1100),
                ),
                vec![0x81, 0x39, 0x00, 0x11, 0x00, 0x00],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(RCX), None, None, None),
                    Immediate(0x221100),
                ),
                vec![0x81, 0x39, 0x00, 0x11, 0x22, 0x00],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(RCX), None, None, None),
                    Immediate(0x33221100),
                ),
                vec![0x81, 0x39, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Cmp(
                    Memory(Some(QWord), Some(RCX), None, None, None),
                    Immediate(0x00),
                ),
                vec![0x48, 0x83, 0x39, 0x00],
            ),
            (
                Cmp(
                    Memory(Some(QWord), Some(RCX), None, None, None),
                    Immediate(0x1100),
                ),
                vec![0x48, 0x81, 0x39, 0x00, 0x11, 0x00, 0x00],
            ),
            (
                Cmp(
                    Memory(Some(QWord), Some(RCX), None, None, None),
                    Immediate(0x221100),
                ),
                vec![0x48, 0x81, 0x39, 0x00, 0x11, 0x22, 0x00],
            ),
            (
                Cmp(
                    Memory(Some(QWord), Some(RCX), None, None, None),
                    Immediate(0x33221100),
                ),
                vec![0x48, 0x81, 0x39, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(ECX), None, None, None),
                    Immediate(0x33221100),
                ),
                vec![0x67, 0x81, 0x39, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(RSP), None, None, None),
                    Immediate(0x33221100),
                ),
                vec![0x81, 0x3C, 0x24, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(RBP), None, None, None),
                    Immediate(0x33221100),
                ),
                vec![0x81, 0x7D, 0x00, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(R12), None, None, None),
                    Immediate(0x33221100),
                ),
                vec![0x41, 0x81, 0x3C, 0x24, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(RCX), Some(RDX), Some(2), None),
                    Immediate(0x33221100),
                ),
                vec![0x81, 0x3C, 0x51, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(RSP), Some(RDX), Some(2), None),
                    Immediate(0x33221100),
                ),
                vec![0x81, 0x3C, 0x54, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(RBP), Some(RDX), Some(2), None),
                    Immediate(0x33221100),
                ),
                vec![0x81, 0x7C, 0x55, 0x00, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(R12), Some(RDX), Some(2), None),
                    Immediate(0x33221100),
                ),
                vec![0x41, 0x81, 0x3C, 0x54, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(RCX), Some(R13), Some(2), None),
                    Immediate(0x33221100),
                ),
                vec![0x42, 0x81, 0x3C, 0x69, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(RSP), Some(R13), Some(2), None),
                    Immediate(0x33221100),
                ),
                vec![0x42, 0x81, 0x3C, 0x6C, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(RBP), Some(R13), Some(2), None),
                    Immediate(0x33221100),
                ),
                vec![0x42, 0x81, 0x7C, 0x6D, 0x00, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(R12), Some(R13), Some(2), None),
                    Immediate(0x33221100),
                ),
                vec![0x43, 0x81, 0x3C, 0x6C, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(RCX), Some(R13), Some(2), Some(0x1100)),
                    Immediate(0x33221100),
                ),
                vec![
                    0x42, 0x81, 0xBC, 0x69, 0x00, 0x11, 0x00, 0x00, 0x00, 0x11, 0x22, 0x33,
                ],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(RSP), Some(R13), Some(2), Some(0x1100)),
                    Immediate(0x33221100),
                ),
                vec![
                    0x42, 0x81, 0xBC, 0x6C, 0x00, 0x11, 0x00, 0x00, 0x00, 0x11, 0x22, 0x33,
                ],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(RBP), Some(R13), Some(2), Some(0x1100)),
                    Immediate(0x33221100),
                ),
                vec![
                    0x42, 0x81, 0xBC, 0x6D, 0x00, 0x11, 0x00, 0x00, 0x00, 0x11, 0x22, 0x33,
                ],
            ),
            (
                Cmp(
                    Memory(Some(DWord), Some(R12), Some(R13), Some(2), Some(0x1100)),
                    Immediate(0x33221100),
                ),
                vec![
                    0x43, 0x81, 0xBC, 0x6C, 0x00, 0x11, 0x00, 0x00, 0x00, 0x11, 0x22, 0x33,
                ],
            ),
            (
                Cmp(
                    Memory(Some(DWord), None, Some(R12), Some(2), Some(0x00)),
                    Immediate(0x33221100),
                ),
                vec![
                    0x42, 0x81, 0x3C, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x22, 0x33,
                ],
            ),
            (
                Cmp(
                    Memory(Some(DWord), None, Some(R12), Some(2), Some(0x1100)),
                    Immediate(0x33221100),
                ),
                vec![
                    0x42, 0x81, 0x3C, 0x65, 0x00, 0x11, 0x00, 0x00, 0x00, 0x11, 0x22, 0x33,
                ],
            ),
            (
                Cmp(
                    Memory(Some(DWord), None, None, None, Some(0x00)),
                    Immediate(0x33221100),
                ),
                vec![
                    0x81, 0x3C, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x22, 0x33,
                ],
            ),
            (
                Cmp(
                    Memory(Some(DWord), None, None, None, Some(0x1100)),
                    Immediate(0x33221100),
                ),
                vec![
                    0x81, 0x3C, 0x25, 0x00, 0x11, 0x00, 0x00, 0x00, 0x11, 0x22, 0x33,
                ],
            ),
        ];
        run_tests(tests);
    }
}
