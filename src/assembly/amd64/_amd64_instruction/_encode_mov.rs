use crate::assembly::amd64::{AMD64Instruction, AMD64Operand};
use crate::BFResult;

use AMD64Operand::*;

impl AMD64Instruction {
    pub(crate) fn encode_mov(
        self: &AMD64Instruction,
        dst: &AMD64Operand,
        src: &AMD64Operand,
    ) -> BFResult<Vec<u8>> {
        match (dst, src) {
            // mov <reg>, <reg>
            (Register(dst_reg), Register(src_reg)) => {
                if dst_reg.size() != src_reg.size() {
                    return self.encoding_err();
                }

                let prefix_reg_16 = (dst_reg.size() == 16).then_some(0x66);

                let rex = self.encode_rex(Some(src), Some(dst))?;

                let opcode: u8 = if dst_reg.size() == 8 { 0x88 } else { 0x89 };

                let rmi = self.encode_reg_rmi(Some(src), Some(dst), dst_reg.size())?;

                Ok(vec![prefix_reg_16, rex, Some(opcode)]
                    .into_iter()
                    .flatten()
                    .chain(rmi)
                    .collect())
            }

            // mov <reg>, <imm>
            (Register(dst_reg), Immediate(imm)) => {
                let prefix_reg_16 = (dst_reg.size() == 16).then_some(0x66);

                let rex = self.encode_rex(None, Some(dst))?;

                let opcode: u8 = (dst_reg.id() & 7) as u8 + {
                    if dst_reg.size() == 8 {
                        0xB0
                    } else {
                        0xB8
                    }
                };

                let imm = self.encode_imm(*imm, dst_reg.size())?;

                Ok(vec![prefix_reg_16, rex, Some(opcode)]
                    .into_iter()
                    .flatten()
                    .chain(imm)
                    .collect())
            }

            // mov <reg>, <mem>
            (Register(dst_reg), Memory(size, base_reg, index_reg, _, _)) => {
                if let Some(size) = size {
                    if dst_reg.size() != size.size() {
                        return self.encoding_err();
                    }
                }

                let prefix_reg_16 = (dst_reg.size() == 16).then_some(0x66);

                let prefix_addr_32 = self.encode_prefix_addr_32(base_reg, index_reg)?;

                let rex = self.encode_rex(Some(dst), Some(src))?;

                let opcode: u8 = if dst_reg.size() == 8 { 0x8A } else { 0x8B };

                let rmi = self.encode_reg_rmi(Some(dst), Some(src), dst_reg.size())?;

                Ok(vec![prefix_reg_16, prefix_addr_32, rex, Some(opcode)]
                    .into_iter()
                    .flatten()
                    .chain(rmi)
                    .collect())
            }

            // mov <mem>, <reg>
            (Memory(size, base_reg, index_reg, _, _), Register(src_reg)) => {
                if let Some(size) = size {
                    if src_reg.size() != size.size() {
                        return self.encoding_err();
                    }
                }

                let prefix_reg_16 = (src_reg.size() == 16).then_some(0x66);

                let prefix_addr_32 = self.encode_prefix_addr_32(base_reg, index_reg)?;

                let rex = self.encode_rex(Some(src), Some(dst))?;

                let opcode: u8 = if src_reg.size() == 8 { 0x88 } else { 0x89 };

                let rmi = self.encode_reg_rmi(Some(src), Some(dst), src_reg.size())?;

                Ok(vec![prefix_reg_16, prefix_addr_32, rex, Some(opcode)]
                    .into_iter()
                    .flatten()
                    .chain(rmi)
                    .collect())
            }

            // mov <mem>, <imm>
            (Memory(size, _, _, _, _), Immediate(imm)) => {
                let size = {
                    let Some(size) = size else {
                        return self.encoding_err();
                    };
                    size.size()
                };

                let prefix_reg_16 = (size == 16).then_some(0x66);

                let rex = self.encode_rex(None, Some(dst))?;

                let opcode: u8 = if size == 8 { 0xC6 } else { 0xC7 };

                let rmi = self.encode_reg_rmi(None, Some(dst), size)?;
                let imm = self.encode_imm(*imm, size.min(32))?;

                Ok(vec![prefix_reg_16, rex, Some(opcode)]
                    .into_iter()
                    .flatten()
                    .chain(rmi)
                    .chain(imm)
                    .collect())
            }

            (_, _) => self.encoding_err(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, AMD64Register, MemorySize};

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
    fn test_encode_mov_reg_reg() {
        let tests: Tests = vec![
            (Mov(Register(CL), Register(DL)), vec![0x88, 0xD1]),
            (Mov(Register(CX), Register(DX)), vec![0x66, 0x89, 0xD1]),
            (Mov(Register(ECX), Register(EDX)), vec![0x89, 0xD1]),
            (Mov(Register(RCX), Register(RDX)), vec![0x48, 0x89, 0xD1]),
            //
            (Mov(Register(RAX), Register(RAX)), vec![0x48, 0x89, 0xC0]),
            (Mov(Register(RAX), Register(RCX)), vec![0x48, 0x89, 0xC8]),
            (Mov(Register(RAX), Register(R12)), vec![0x4C, 0x89, 0xE0]),
            (Mov(Register(RCX), Register(RAX)), vec![0x48, 0x89, 0xC1]),
            (Mov(Register(RCX), Register(RCX)), vec![0x48, 0x89, 0xC9]),
            (Mov(Register(RCX), Register(R12)), vec![0x4C, 0x89, 0xE1]),
            (Mov(Register(R12), Register(RAX)), vec![0x49, 0x89, 0xC4]),
            (Mov(Register(R12), Register(RCX)), vec![0x49, 0x89, 0xCC]),
            (Mov(Register(R12), Register(R12)), vec![0x4D, 0x89, 0xE4]),
            //
            (Mov(Register(RCX), Register(RSP)), vec![0x48, 0x89, 0xE1]),
            (Mov(Register(RCX), Register(RBP)), vec![0x48, 0x89, 0xE9]),
            (Mov(Register(RSP), Register(RCX)), vec![0x48, 0x89, 0xCC]),
            (Mov(Register(RSP), Register(RSP)), vec![0x48, 0x89, 0xE4]),
            (Mov(Register(RSP), Register(RBP)), vec![0x48, 0x89, 0xEC]),
            (Mov(Register(RBP), Register(RCX)), vec![0x48, 0x89, 0xCD]),
            (Mov(Register(RBP), Register(RSP)), vec![0x48, 0x89, 0xE5]),
            (Mov(Register(RBP), Register(RBP)), vec![0x48, 0x89, 0xED]),
        ];
        run_tests(tests);
    }

    #[test]
    fn test_encode_mov_reg_imm() {
        let tests: Tests = vec![
            (Mov(Register(CL), Immediate(0x1)), vec![0xB1, 0x01]),
            (Mov(Register(CL), Immediate(0x100)), vec![0xB1, 0x00]),
            (
                Mov(Register(CX), Immediate(0x1)),
                vec![0x66, 0xB9, 0x01, 0x00],
            ),
            (
                Mov(Register(CX), Immediate(0x100)),
                vec![0x66, 0xB9, 0x00, 0x01],
            ),
            (
                Mov(Register(ECX), Immediate(0x1)),
                vec![0xB9, 0x01, 0x00, 0x00, 0x00],
            ),
            (
                Mov(Register(ECX), Immediate(0x100)),
                vec![0xB9, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Mov(Register(ECX), Immediate(0x10000)),
                vec![0xB9, 0x00, 0x00, 0x01, 0x00],
            ),
            (
                Mov(Register(ECX), Immediate(0x1000000)),
                vec![0xB9, 0x00, 0x00, 0x00, 0x01],
            ),
            (
                Mov(Register(RCX), Immediate(0x1)),
                vec![0x48, 0xB9, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            ),
            (
                Mov(Register(RCX), Immediate(0x100)),
                vec![0x48, 0xB9, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            ),
            (
                Mov(Register(RCX), Immediate(0x10000)),
                vec![0x48, 0xB9, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00],
            ),
            (
                Mov(Register(RCX), Immediate(0x1000000)),
                vec![0x48, 0xB9, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00],
            ),
            //
            (Mov(Register(R12B), Immediate(0x1)), vec![0x41, 0xB4, 0x01]),
            (
                Mov(Register(R12B), Immediate(0x100)),
                vec![0x41, 0xB4, 0x00],
            ),
            (
                Mov(Register(R12W), Immediate(0x1)),
                vec![0x66, 0x41, 0xBC, 0x01, 0x00],
            ),
            (
                Mov(Register(R12W), Immediate(0x100)),
                vec![0x66, 0x41, 0xBC, 0x00, 0x01],
            ),
            (
                Mov(Register(R12D), Immediate(0x1)),
                vec![0x41, 0xBC, 0x01, 0x00, 0x00, 0x00],
            ),
            (
                Mov(Register(R12D), Immediate(0x100)),
                vec![0x41, 0xBC, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Mov(Register(R12D), Immediate(0x10000)),
                vec![0x41, 0xBC, 0x00, 0x00, 0x01, 0x00],
            ),
            (
                Mov(Register(R12D), Immediate(0x1000000)),
                vec![0x41, 0xBC, 0x00, 0x00, 0x00, 0x01],
            ),
            (
                Mov(Register(R12), Immediate(0x1)),
                vec![0x49, 0xBC, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            ),
            (
                Mov(Register(R12), Immediate(0x100)),
                vec![0x49, 0xBC, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            ),
            (
                Mov(Register(R12), Immediate(0x10000)),
                vec![0x49, 0xBC, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00],
            ),
            (
                Mov(Register(R12), Immediate(0x1000000)),
                vec![0x49, 0xBC, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00],
            ),
        ];
        run_tests(tests);
    }

    #[test]
    fn test_encode_mov_reg_mem() {
        let tests: Tests = vec![
            (
                Mov(Register(CL), Memory(None, Some(RCX), None, None, None)),
                vec![0x8A, 0x09],
            ),
            (
                Mov(Register(CX), Memory(None, Some(RCX), None, None, None)),
                vec![0x66, 0x8B, 0x09],
            ),
            (
                Mov(Register(ECX), Memory(None, Some(RCX), None, None, None)),
                vec![0x8B, 0x09],
            ),
            (
                Mov(Register(RCX), Memory(None, Some(RCX), None, None, None)),
                vec![0x48, 0x8B, 0x09],
            ),
            (
                Mov(Register(ECX), Memory(None, Some(ECX), None, None, None)),
                vec![0x67, 0x8B, 0x09],
            ),
            //
            (
                Mov(Register(ECX), Memory(None, None, Some(RCX), Some(2), None)),
                vec![0x8B, 0x0C, 0x4D, 0x00, 0x00, 0x00, 0x00],
            ),
            (
                Mov(Register(ECX), Memory(None, None, None, None, Some(0x0))),
                vec![0x8B, 0x0C, 0x25, 0x00, 0x00, 0x00, 0x00],
            ),
            (
                Mov(Register(ECX), Memory(None, None, None, None, Some(0x1))),
                vec![0x8B, 0x0C, 0x25, 0x01, 0x00, 0x00, 0x00],
            ),
            (
                Mov(Register(ECX), Memory(None, None, None, None, Some(0x100))),
                vec![0x8B, 0x0C, 0x25, 0x00, 0x01, 0x00, 0x00],
            ),
            //
            (
                Mov(Register(ECX), Memory(None, Some(RCX), None, None, None)),
                vec![0x8B, 0x09],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RCX), None, None, Some(0x1)),
                ),
                vec![0x8B, 0x49, 0x01],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RCX), None, None, Some(0x100)),
                ),
                vec![0x8B, 0x89, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Mov(Register(ECX), Memory(None, Some(R12), None, None, None)),
                vec![0x41, 0x8B, 0x0C, 0x24],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(R12), None, None, Some(0x1)),
                ),
                vec![0x41, 0x8B, 0x4C, 0x24, 0x01],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(R12), None, None, Some(0x100)),
                ),
                vec![0x41, 0x8B, 0x8C, 0x24, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Mov(Register(ECX), Memory(None, Some(RSP), None, None, None)),
                vec![0x8B, 0x0C, 0x24],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RSP), None, None, Some(0x1)),
                ),
                vec![0x8B, 0x4C, 0x24, 0x01],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RSP), None, None, Some(0x100)),
                ),
                vec![0x8B, 0x8C, 0x24, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Mov(Register(ECX), Memory(None, Some(RBP), None, None, None)),
                vec![0x8B, 0x4D, 0x00],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RBP), None, None, Some(0x1)),
                ),
                vec![0x8B, 0x4D, 0x01],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RBP), None, None, Some(0x100)),
                ),
                vec![0x8B, 0x8D, 0x00, 0x01, 0x00, 0x00],
            ),
            //
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(RCX), Some(2), None),
                ),
                vec![0x8B, 0x0C, 0x49],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(RCX), Some(2), Some(0x1)),
                ),
                vec![0x8B, 0x4C, 0x49, 0x01],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(RCX), Some(2), Some(0x100)),
                ),
                vec![0x8B, 0x8C, 0x49, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(R12), Some(RCX), Some(2), None),
                ),
                vec![0x41, 0x8B, 0x0C, 0x4C],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(R12), Some(RCX), Some(2), Some(0x1)),
                ),
                vec![0x41, 0x8B, 0x4C, 0x4C, 0x01],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(R12), Some(RCX), Some(2), Some(0x100)),
                ),
                vec![0x41, 0x8B, 0x8C, 0x4C, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(RCX), Some(2), None),
                ),
                vec![0x8B, 0x0C, 0x4C],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(RCX), Some(2), Some(0x1)),
                ),
                vec![0x8B, 0x4C, 0x4C, 0x01],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(RCX), Some(2), Some(0x100)),
                ),
                vec![0x8B, 0x8C, 0x4C, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(RCX), Some(2), None),
                ),
                vec![0x8B, 0x4C, 0x4D, 0x00],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(RCX), Some(2), Some(0x1)),
                ),
                vec![0x8B, 0x4C, 0x4D, 0x01],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(RCX), Some(2), Some(0x100)),
                ),
                vec![0x8B, 0x8C, 0x4D, 0x00, 0x01, 0x00, 0x00],
            ),
            //
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(RBP), Some(2), None),
                ),
                vec![0x8B, 0x0C, 0x69],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(RBP), Some(2), Some(0x1)),
                ),
                vec![0x8B, 0x4C, 0x69, 0x01],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(RBP), Some(2), Some(0x100)),
                ),
                vec![0x8B, 0x8C, 0x69, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(R12), Some(RBP), Some(2), None),
                ),
                vec![0x41, 0x8B, 0x0C, 0x6C],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(R12), Some(RBP), Some(2), Some(0x1)),
                ),
                vec![0x41, 0x8B, 0x4C, 0x6C, 0x01],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(R12), Some(RBP), Some(2), Some(0x100)),
                ),
                vec![0x41, 0x8B, 0x8C, 0x6C, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(RBP), Some(2), None),
                ),
                vec![0x8B, 0x0C, 0x6C],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(RBP), Some(2), Some(0x1)),
                ),
                vec![0x8B, 0x4C, 0x6C, 0x01],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(RBP), Some(2), Some(0x100)),
                ),
                vec![0x8B, 0x8C, 0x6C, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(RBP), Some(2), None),
                ),
                vec![0x8B, 0x4C, 0x6D, 0x00],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(RBP), Some(2), Some(0x1)),
                ),
                vec![0x8B, 0x4C, 0x6D, 0x01],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(RBP), Some(2), Some(0x100)),
                ),
                vec![0x8B, 0x8C, 0x6D, 0x00, 0x01, 0x00, 0x00],
            ),
            //
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(R12), Some(2), None),
                ),
                vec![0x42, 0x8B, 0x0C, 0x61],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(R12), Some(2), Some(0x1)),
                ),
                vec![0x42, 0x8B, 0x4C, 0x61, 0x01],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(R12), Some(2), Some(0x100)),
                ),
                vec![0x42, 0x8B, 0x8C, 0x61, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(R12), Some(R12), Some(2), None),
                ),
                vec![0x43, 0x8B, 0x0C, 0x64],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(R12), Some(R12), Some(2), Some(0x1)),
                ),
                vec![0x43, 0x8B, 0x4C, 0x64, 0x01],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(R12), Some(R12), Some(2), Some(0x100)),
                ),
                vec![0x43, 0x8B, 0x8C, 0x64, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(R12), Some(2), None),
                ),
                vec![0x42, 0x8B, 0x0C, 0x64],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(R12), Some(2), Some(0x1)),
                ),
                vec![0x42, 0x8B, 0x4C, 0x64, 0x01],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(R12), Some(2), Some(0x100)),
                ),
                vec![0x42, 0x8B, 0x8C, 0x64, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(R12), Some(2), None),
                ),
                vec![0x42, 0x8B, 0x4C, 0x65, 0x00],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(R12), Some(2), Some(0x1)),
                ),
                vec![0x42, 0x8B, 0x4C, 0x65, 0x01],
            ),
            (
                Mov(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(R12), Some(2), Some(0x100)),
                ),
                vec![0x42, 0x8B, 0x8C, 0x65, 0x00, 0x01, 0x00, 0x00],
            ),
        ];
        run_tests(tests);
    }

    #[test]
    fn test_encode_mov_mem_reg() {
        let tests: Tests = vec![
            (
                Mov(Memory(None, Some(RCX), None, None, None), Register(CL)),
                vec![0x88, 0x09],
            ),
            (
                Mov(Memory(None, Some(RCX), None, None, None), Register(CX)),
                vec![0x66, 0x89, 0x09],
            ),
            (
                Mov(Memory(None, Some(RCX), None, None, None), Register(ECX)),
                vec![0x89, 0x09],
            ),
            (
                Mov(Memory(None, Some(RCX), None, None, None), Register(RCX)),
                vec![0x48, 0x89, 0x09],
            ),
            (
                Mov(Memory(None, Some(RCX), None, None, None), Register(ECX)),
                vec![0x89, 0x09],
            ),
            (
                Mov(Memory(None, Some(R12), None, None, None), Register(ECX)),
                vec![0x41, 0x89, 0x0C, 0x24],
            ),
            (
                Mov(Memory(None, Some(RSP), None, None, None), Register(ECX)),
                vec![0x89, 0x0C, 0x24],
            ),
            (
                Mov(Memory(None, Some(RBP), None, None, None), Register(ECX)),
                vec![0x89, 0x4D, 0x00],
            ),
            (
                Mov(Memory(None, Some(ECX), None, None, None), Register(ECX)),
                vec![0x67, 0x89, 0x09],
            ),
            (
                Mov(Memory(None, Some(ECX), None, None, None), Register(R12)),
                vec![0x67, 0x4C, 0x89, 0x21],
            ),
            (
                Mov(
                    Memory(None, Some(RCX), Some(RDX), Some(2), Some(0x1100)),
                    Register(ECX),
                ),
                vec![0x89, 0x8C, 0x51, 0x00, 0x11, 0x00, 0x00],
            ),
            (
                Mov(
                    Memory(None, Some(R12), Some(RDX), Some(2), Some(0x1100)),
                    Register(ECX),
                ),
                vec![0x41, 0x89, 0x8C, 0x54, 0x00, 0x11, 0x00, 0x00],
            ),
            (
                Mov(
                    Memory(None, Some(RSP), Some(RDX), Some(2), Some(0x1100)),
                    Register(ECX),
                ),
                vec![0x89, 0x8C, 0x54, 0x00, 0x11, 0x00, 0x00],
            ),
            (
                Mov(
                    Memory(None, Some(RBP), Some(RDX), Some(2), Some(0x1100)),
                    Register(ECX),
                ),
                vec![0x89, 0x8C, 0x55, 0x00, 0x11, 0x00, 0x00],
            ),
            (
                Mov(
                    Memory(None, Some(RCX), Some(R12), Some(2), Some(0x1100)),
                    Register(R12D),
                ),
                vec![0x46, 0x89, 0xA4, 0x61, 0x00, 0x11, 0x00, 0x00],
            ),
            (
                Mov(
                    Memory(None, Some(R12), Some(R12), Some(2), Some(0x1100)),
                    Register(R12D),
                ),
                vec![0x47, 0x89, 0xA4, 0x64, 0x00, 0x11, 0x00, 0x00],
            ),
            (
                Mov(
                    Memory(None, Some(RSP), Some(R12), Some(2), Some(0x1100)),
                    Register(R12D),
                ),
                vec![0x46, 0x89, 0xA4, 0x64, 0x00, 0x11, 0x00, 0x00],
            ),
            (
                Mov(
                    Memory(None, Some(RBP), Some(R12), Some(2), Some(0x1100)),
                    Register(R12D),
                ),
                vec![0x46, 0x89, 0xA4, 0x65, 0x00, 0x11, 0x00, 0x00],
            ),
        ];
        run_tests(tests);
    }

    #[test]
    fn test_encode_mov_mem_imm_size() {
        let tests: Tests = vec![
            // mov BYTE PTR [rcx], 0x11
            (
                Mov(
                    Memory(Some(Byte), Some(RCX), None, None, None),
                    Immediate(0x11),
                ),
                vec![0xC6, 0x01, 0x11],
            ),
            // mov WORD PTR [rcx], 0x11
            (
                Mov(
                    Memory(Some(Word), Some(RCX), None, None, None),
                    Immediate(0x11),
                ),
                vec![0x66, 0xC7, 0x01, 0x11, 0x00],
            ),
            // mov DWORD PTR [rcx], 0x11
            (
                Mov(
                    Memory(Some(DWord), Some(RCX), None, None, None),
                    Immediate(0x11),
                ),
                vec![0xC7, 0x01, 0x11, 0x00, 0x00, 0x00],
            ),
            // mov QWORD PTR [rcx], 0x11
            (
                Mov(
                    Memory(Some(QWord), Some(RCX), None, None, None),
                    Immediate(0x11),
                ),
                vec![0x48, 0xC7, 0x01, 0x11, 0x00, 0x00, 0x00],
            ),
            // mov BYTE PTR [rcx], 0x44332211
            (
                Mov(
                    Memory(Some(Byte), Some(RCX), None, None, None),
                    Immediate(0x44332211),
                ),
                vec![0xC6, 0x01, 0x11],
            ),
            // mov WORD PTR [rcx], 0x44332211
            (
                Mov(
                    Memory(Some(Word), Some(RCX), None, None, None),
                    Immediate(0x44332211),
                ),
                vec![0x66, 0xC7, 0x01, 0x11, 0x22],
            ),
            // mov DWORD PTR [rcx], 0x44332211
            (
                Mov(
                    Memory(Some(DWord), Some(RCX), None, None, None),
                    Immediate(0x44332211),
                ),
                vec![0xC7, 0x01, 0x11, 0x22, 0x33, 0x44],
            ),
            // mov QWORD PTR [rcx], 0x44332211
            (
                Mov(
                    Memory(Some(QWord), Some(RCX), None, None, None),
                    Immediate(0x44332211),
                ),
                vec![0x48, 0xC7, 0x01, 0x11, 0x22, 0x33, 0x44],
            ),
        ];
        run_tests(tests);
    }
}
