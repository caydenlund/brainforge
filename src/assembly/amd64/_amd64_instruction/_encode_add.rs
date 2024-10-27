use crate::assembly::amd64::{AMD64Instruction, AMD64Operand};
use crate::BFResult;

use AMD64Operand::*;

impl AMD64Instruction {
    pub(crate) fn encode_add(
        self: &AMD64Instruction,
        dst: &AMD64Operand,
        src: &AMD64Operand,
    ) -> BFResult<Vec<u8>> {
        match (dst, src) {
            // register += register
            (Register(dst_reg), Register(src_reg)) => {
                if dst_reg.size() != src_reg.size() {
                    return self.encoding_err();
                }

                let prefix_reg_16 = (dst_reg.size() == 16).then_some(0x66);

                let rex = self.encode_rex(Some(src), Some(dst))?;

                let opcode: u8 = if dst_reg.size() == 8 { 0x00 } else { 0x01 };

                let rmi = self.encode_reg_rmi(Some(src), Some(dst), dst_reg.size())?;

                Ok(vec![prefix_reg_16, rex, Some(opcode)]
                    .into_iter()
                    .flatten()
                    .chain(rmi)
                    .collect())
            }

            // register += memory
            (Register(dst_reg), Memory(size, base_reg, index_reg, _, _)) => {
                if let Some(size) = size {
                    assert_eq!(
                        size.size(),
                        dst_reg.size(),
                        "Operand size mismatch: `{}`",
                        self.to_string()
                    );
                }

                let prefix_reg_16 = (dst_reg.size() == 16).then_some(0x66);

                let prefix_addr_32 = self.encode_prefix_addr_32(base_reg, index_reg)?;

                let rex = self.encode_rex(Some(dst), Some(src))?;

                let opcode: u8 = if dst_reg.size() == 8 { 0x02 } else { 0x03 };

                let rmi = self.encode_reg_rmi(Some(dst), Some(src), dst_reg.size())?;

                Ok(vec![prefix_reg_16, prefix_addr_32, rex, Some(opcode)]
                    .into_iter()
                    .flatten()
                    .chain(rmi)
                    .collect())
            }

            // register += immediate
            (Register(dst_reg), Immediate(imm)) => {
                let prefix_reg_16 = (dst_reg.size() == 16).then_some(0x66);

                let rex = self.encode_rex(None, Some(dst))?;

                let opcode: u8 = match (dst_reg.id(), dst_reg.size()) {
                    (0, 8) => 0x04,
                    (0, _) => 0x05,
                    (_, 8) => 0x80,
                    (_, _) => 0x81,
                };

                let rmi = self.encode_reg_rmi(None, Some(dst), dst_reg.size())?;

                let imm = self.encode_imm(*imm, dst_reg.size().min(32))?;

                Ok(vec![prefix_reg_16, rex, Some(opcode)]
                    .into_iter()
                    .flatten()
                    .chain(rmi)
                    .chain(imm)
                    .collect())
            }

            // memory += register
            (Memory(size, base_reg, index_reg, _, _), Register(src_reg)) => {
                if let Some(size) = size {
                    assert_eq!(
                        size.size(),
                        src_reg.size(),
                        "Operand size mismatch: `{}`",
                        self.to_string()
                    );
                }

                let prefix_reg_16 = (src_reg.size() == 16).then_some(0x66);

                let prefix_addr_32 = self.encode_prefix_addr_32(base_reg, index_reg)?;

                let rex = self.encode_rex(Some(src), Some(dst))?;

                let opcode: u8 = if src_reg.size() == 8 { 0x00 } else { 0x01 };

                let rmi = self.encode_reg_rmi(Some(src), Some(dst), src_reg.size())?;

                Ok(vec![prefix_reg_16, prefix_addr_32, rex, Some(opcode)]
                    .into_iter()
                    .flatten()
                    .chain(rmi)
                    .collect())
            }

            // memory += immediate
            (Memory(size, base_reg, index_reg, _, _), Immediate(imm)) => {
                let Some(size) = size else {
                    return self.encoding_err();
                };

                let prefix_reg_16 = (size.size() == 16).then_some(0x66);

                let prefix_addr_32 = self.encode_prefix_addr_32(base_reg, index_reg)?;

                let rex = self.encode_rex(None, Some(dst))?;

                let rmi = self.encode_reg_rmi(None, Some(dst), size.size())?;

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

            (_, _) => panic!("Invalid instruction: `{}`", self.to_string()),
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
    fn test_encode_add_reg_reg() {
        let tests: Tests = vec![
            (Add(Register(CL), Register(DL)), vec![0x00, 0xD1]),
            (Add(Register(CX), Register(DX)), vec![0x66, 0x01, 0xD1]),
            (Add(Register(ECX), Register(EDX)), vec![0x01, 0xD1]),
            (Add(Register(RCX), Register(RDX)), vec![0x48, 0x01, 0xD1]),
            //
            (Add(Register(RAX), Register(RAX)), vec![0x48, 0x01, 0xC0]),
            (Add(Register(RAX), Register(RCX)), vec![0x48, 0x01, 0xC8]),
            (Add(Register(RAX), Register(R12)), vec![0x4C, 0x01, 0xE0]),
            (Add(Register(RCX), Register(RAX)), vec![0x48, 0x01, 0xC1]),
            (Add(Register(RCX), Register(RCX)), vec![0x48, 0x01, 0xC9]),
            (Add(Register(RCX), Register(R12)), vec![0x4C, 0x01, 0xE1]),
            (Add(Register(R12), Register(RAX)), vec![0x49, 0x01, 0xC4]),
            (Add(Register(R12), Register(RCX)), vec![0x49, 0x01, 0xCC]),
            (Add(Register(R12), Register(R12)), vec![0x4D, 0x01, 0xE4]),
            //
            (Add(Register(RCX), Register(RSP)), vec![0x48, 0x01, 0xE1]),
            (Add(Register(RCX), Register(RBP)), vec![0x48, 0x01, 0xE9]),
            (Add(Register(RSP), Register(RCX)), vec![0x48, 0x01, 0xCC]),
            (Add(Register(RSP), Register(RSP)), vec![0x48, 0x01, 0xE4]),
            (Add(Register(RSP), Register(RBP)), vec![0x48, 0x01, 0xEC]),
            (Add(Register(RBP), Register(RCX)), vec![0x48, 0x01, 0xCD]),
            (Add(Register(RBP), Register(RSP)), vec![0x48, 0x01, 0xE5]),
            (Add(Register(RBP), Register(RBP)), vec![0x48, 0x01, 0xED]),
        ];
        run_tests(tests);
    }

    #[test]
    fn test_encode_add_reg_mem() {
        let tests: Tests = vec![
            (
                Add(Register(CL), Memory(None, Some(RCX), None, None, None)),
                vec![0x02, 0x09],
            ),
            (
                Add(Register(CX), Memory(None, Some(RCX), None, None, None)),
                vec![0x66, 0x03, 0x09],
            ),
            (
                Add(Register(ECX), Memory(None, Some(RCX), None, None, None)),
                vec![0x03, 0x09],
            ),
            (
                Add(Register(RCX), Memory(None, Some(RCX), None, None, None)),
                vec![0x48, 0x03, 0x09],
            ),
            (
                Add(Register(ECX), Memory(None, Some(ECX), None, None, None)),
                vec![0x67, 0x03, 0x09],
            ),
            //
            (
                Add(Register(ECX), Memory(None, None, Some(RCX), Some(2), None)),
                vec![0x03, 0x0C, 0x4D, 0x00, 0x00, 0x00, 0x00],
            ),
            (
                Add(Register(ECX), Memory(None, None, None, None, Some(0x0))),
                vec![0x03, 0x0C, 0x25, 0x00, 0x00, 0x00, 0x00],
            ),
            (
                Add(Register(ECX), Memory(None, None, None, None, Some(0x1))),
                vec![0x03, 0x0C, 0x25, 0x01, 0x00, 0x00, 0x00],
            ),
            (
                Add(Register(ECX), Memory(None, None, None, None, Some(0x100))),
                vec![0x03, 0x0C, 0x25, 0x00, 0x01, 0x00, 0x00],
            ),
            //
            (
                Add(Register(ECX), Memory(None, Some(RCX), None, None, None)),
                vec![0x03, 0x09],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RCX), None, None, Some(0x1)),
                ),
                vec![0x03, 0x49, 0x01],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RCX), None, None, Some(0x100)),
                ),
                vec![0x03, 0x89, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Add(Register(ECX), Memory(None, Some(R12), None, None, None)),
                vec![0x41, 0x03, 0x0C, 0x24],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(R12), None, None, Some(0x1)),
                ),
                vec![0x41, 0x03, 0x4C, 0x24, 0x01],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(R12), None, None, Some(0x100)),
                ),
                vec![0x41, 0x03, 0x8C, 0x24, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Add(Register(ECX), Memory(None, Some(RSP), None, None, None)),
                vec![0x03, 0x0C, 0x24],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RSP), None, None, Some(0x1)),
                ),
                vec![0x03, 0x4C, 0x24, 0x01],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RSP), None, None, Some(0x100)),
                ),
                vec![0x03, 0x8C, 0x24, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Add(Register(ECX), Memory(None, Some(RBP), None, None, None)),
                vec![0x03, 0x4D, 0x00],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RBP), None, None, Some(0x1)),
                ),
                vec![0x03, 0x4D, 0x01],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RBP), None, None, Some(0x100)),
                ),
                vec![0x03, 0x8D, 0x00, 0x01, 0x00, 0x00],
            ),
            //
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(RCX), Some(2), None),
                ),
                vec![0x03, 0x0C, 0x49],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(RCX), Some(2), Some(0x1)),
                ),
                vec![0x03, 0x4C, 0x49, 0x01],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(RCX), Some(2), Some(0x100)),
                ),
                vec![0x03, 0x8C, 0x49, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(R12), Some(RCX), Some(2), None),
                ),
                vec![0x41, 0x03, 0x0C, 0x4C],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(R12), Some(RCX), Some(2), Some(0x1)),
                ),
                vec![0x41, 0x03, 0x4C, 0x4C, 0x01],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(R12), Some(RCX), Some(2), Some(0x100)),
                ),
                vec![0x41, 0x03, 0x8C, 0x4C, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(RCX), Some(2), None),
                ),
                vec![0x03, 0x0C, 0x4C],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(RCX), Some(2), Some(0x1)),
                ),
                vec![0x03, 0x4C, 0x4C, 0x01],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(RCX), Some(2), Some(0x100)),
                ),
                vec![0x03, 0x8C, 0x4C, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(RCX), Some(2), None),
                ),
                vec![0x03, 0x4C, 0x4D, 0x00],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(RCX), Some(2), Some(0x1)),
                ),
                vec![0x03, 0x4C, 0x4D, 0x01],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(RCX), Some(2), Some(0x100)),
                ),
                vec![0x03, 0x8C, 0x4D, 0x00, 0x01, 0x00, 0x00],
            ),
            //
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(RBP), Some(2), None),
                ),
                vec![0x03, 0x0C, 0x69],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(RBP), Some(2), Some(0x1)),
                ),
                vec![0x03, 0x4C, 0x69, 0x01],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(RBP), Some(2), Some(0x100)),
                ),
                vec![0x03, 0x8C, 0x69, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(R12), Some(RBP), Some(2), None),
                ),
                vec![0x41, 0x03, 0x0C, 0x6C],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(R12), Some(RBP), Some(2), Some(0x1)),
                ),
                vec![0x41, 0x03, 0x4C, 0x6C, 0x01],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(R12), Some(RBP), Some(2), Some(0x100)),
                ),
                vec![0x41, 0x03, 0x8C, 0x6C, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(RBP), Some(2), None),
                ),
                vec![0x03, 0x0C, 0x6C],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(RBP), Some(2), Some(0x1)),
                ),
                vec![0x03, 0x4C, 0x6C, 0x01],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(RBP), Some(2), Some(0x100)),
                ),
                vec![0x03, 0x8C, 0x6C, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(RBP), Some(2), None),
                ),
                vec![0x03, 0x4C, 0x6D, 0x00],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(RBP), Some(2), Some(0x1)),
                ),
                vec![0x03, 0x4C, 0x6D, 0x01],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(RBP), Some(2), Some(0x100)),
                ),
                vec![0x03, 0x8C, 0x6D, 0x00, 0x01, 0x00, 0x00],
            ),
            //
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(R12), Some(2), None),
                ),
                vec![0x42, 0x03, 0x0C, 0x61],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(R12), Some(2), Some(0x1)),
                ),
                vec![0x42, 0x03, 0x4C, 0x61, 0x01],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RCX), Some(R12), Some(2), Some(0x100)),
                ),
                vec![0x42, 0x03, 0x8C, 0x61, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(R12), Some(R12), Some(2), None),
                ),
                vec![0x43, 0x03, 0x0C, 0x64],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(R12), Some(R12), Some(2), Some(0x1)),
                ),
                vec![0x43, 0x03, 0x4C, 0x64, 0x01],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(R12), Some(R12), Some(2), Some(0x100)),
                ),
                vec![0x43, 0x03, 0x8C, 0x64, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(R12), Some(2), None),
                ),
                vec![0x42, 0x03, 0x0C, 0x64],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(R12), Some(2), Some(0x1)),
                ),
                vec![0x42, 0x03, 0x4C, 0x64, 0x01],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RSP), Some(R12), Some(2), Some(0x100)),
                ),
                vec![0x42, 0x03, 0x8C, 0x64, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(R12), Some(2), None),
                ),
                vec![0x42, 0x03, 0x4C, 0x65, 0x00],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(R12), Some(2), Some(0x1)),
                ),
                vec![0x42, 0x03, 0x4C, 0x65, 0x01],
            ),
            (
                Add(
                    Register(ECX),
                    Memory(None, Some(RBP), Some(R12), Some(2), Some(0x100)),
                ),
                vec![0x42, 0x03, 0x8C, 0x65, 0x00, 0x01, 0x00, 0x00],
            ),
        ];
        run_tests(tests);
    }

    #[test]
    fn test_encode_add_mem_reg() {
        let tests: Tests = vec![
            (
                Add(Memory(None, Some(ECX), None, None, None), Register(DL)),
                vec![0x67, 0x00, 0x11],
            ),
            (
                Add(Memory(None, Some(ECX), None, None, None), Register(DX)),
                vec![0x66, 0x67, 0x01, 0x11],
            ),
            (
                Add(Memory(None, Some(ECX), None, None, None), Register(EDX)),
                vec![0x67, 0x01, 0x11],
            ),
            (
                Add(Memory(None, Some(ECX), None, None, None), Register(RDX)),
                vec![0x67, 0x48, 0x01, 0x11],
            ),
            //
            (
                Add(Memory(None, Some(RCX), None, None, None), Register(DL)),
                vec![0x00, 0x11],
            ),
            (
                Add(Memory(None, Some(RCX), None, None, None), Register(DX)),
                vec![0x66, 0x01, 0x11],
            ),
            (
                Add(Memory(None, Some(RCX), None, None, None), Register(EDX)),
                vec![0x01, 0x11],
            ),
            (
                Add(Memory(None, Some(RCX), None, None, None), Register(RDX)),
                vec![0x48, 0x01, 0x11],
            ),
            //
            (
                Add(Memory(None, Some(R12), None, None, None), Register(EDX)),
                vec![0x41, 0x01, 0x14, 0x24],
            ),
            (
                Add(Memory(None, Some(RDX), None, None, None), Register(R12D)),
                vec![0x44, 0x01, 0x22],
            ),
            (
                Add(Memory(None, None, Some(R12), Some(2), None), Register(EDX)),
                vec![0x42, 0x01, 0x14, 0x65, 0x00, 0x00, 0x00, 0x00],
            ),
            (
                Add(Memory(None, None, Some(RDX), Some(2), None), Register(R12D)),
                vec![0x44, 0x01, 0x24, 0x55, 0x00, 0x00, 0x00, 0x00],
            ),
            //
            (
                Add(Memory(None, None, None, None, None), Register(EDX)),
                vec![0x01, 0x14, 0x25, 0x00, 0x00, 0x00, 0x00],
            ),
            (
                Add(Memory(None, None, None, None, Some(0x1)), Register(EDX)),
                vec![0x01, 0x14, 0x25, 0x01, 0x00, 0x00, 0x00],
            ),
            (
                Add(Memory(None, None, None, None, Some(0x100)), Register(EDX)),
                vec![0x01, 0x14, 0x25, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Add(Memory(None, Some(RCX), None, None, None), Register(EDX)),
                vec![0x01, 0x11],
            ),
            (
                Add(
                    Memory(None, Some(RCX), None, None, Some(0x1)),
                    Register(EDX),
                ),
                vec![0x01, 0x51, 0x01],
            ),
            (
                Add(
                    Memory(None, Some(RCX), None, None, Some(0x100)),
                    Register(EDX),
                ),
                vec![0x01, 0x91, 0x00, 0x01, 0x00, 0x00],
            ),
            (
                Add(
                    Memory(None, Some(RCX), Some(RDX), Some(2), None),
                    Register(EDX),
                ),
                vec![0x01, 0x14, 0x51],
            ),
            (
                Add(
                    Memory(None, Some(RCX), Some(RDX), Some(2), Some(0x1)),
                    Register(EDX),
                ),
                vec![0x01, 0x54, 0x51, 0x01],
            ),
            (
                Add(
                    Memory(None, Some(RCX), Some(RDX), Some(2), Some(0x100)),
                    Register(EDX),
                ),
                vec![0x01, 0x94, 0x51, 0x00, 0x01, 0x00, 0x00],
            ),
        ];
        run_tests(tests);
    }

    #[test]
    fn test_encode_add_mem_imm() {
        let tests: Tests = vec![
            (
                Add(
                    Memory(Some(Byte), Some(ECX), None, None, None),
                    Immediate(0x00),
                ),
                vec![0x67, 0x80, 0x01, 0x00],
            ),
            (
                Add(
                    Memory(Some(DWord), Some(ECX), None, None, None),
                    Immediate(0x33221100),
                ),
                vec![0x67, 0x81, 0x01, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Add(
                    Memory(Some(Byte), Some(R12), None, None, None),
                    Immediate(0x00),
                ),
                vec![0x41, 0x80, 0x04, 0x24, 0x00],
            ),
            (
                Add(
                    Memory(Some(DWord), Some(R12), None, None, None),
                    Immediate(0x33221100),
                ),
                vec![0x41, 0x81, 0x04, 0x24, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Add(
                    Memory(Some(Byte), Some(RBP), None, None, None),
                    Immediate(0x00),
                ),
                vec![0x80, 0x45, 0x00, 0x00],
            ),
            (
                Add(
                    Memory(Some(DWord), Some(RBP), None, None, None),
                    Immediate(0x33221100),
                ),
                vec![0x81, 0x45, 0x00, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Add(
                    Memory(Some(Byte), Some(RSP), None, None, None),
                    Immediate(0x00),
                ),
                vec![0x80, 0x04, 0x24, 0x00],
            ),
            (
                Add(
                    Memory(Some(DWord), Some(RSP), None, None, None),
                    Immediate(0x33221100),
                ),
                vec![0x81, 0x04, 0x24, 0x00, 0x11, 0x22, 0x33],
            ),
            //
            (
                Add(
                    Memory(Some(Byte), Some(RCX), None, None, None),
                    Immediate(0x00),
                ),
                vec![0x80, 0x01, 0x00],
            ),
            (
                Add(
                    Memory(Some(Byte), Some(RCX), None, None, None),
                    Immediate(0x1100),
                ),
                vec![0x80, 0x01, 0x00],
            ),
            (
                Add(
                    Memory(Some(Word), Some(RCX), None, None, None),
                    Immediate(0x00),
                ),
                vec![0x66, 0x83, 0x01, 0x00],
            ),
            (
                Add(
                    Memory(Some(Word), Some(RCX), None, None, None),
                    Immediate(0x1100),
                ),
                vec![0x66, 0x81, 0x01, 0x00, 0x11],
            ),
            (
                Add(
                    Memory(Some(Word), Some(RCX), None, None, None),
                    Immediate(0x221100),
                ),
                vec![0x66, 0x81, 0x01, 0x00, 0x11],
            ),
            (
                Add(
                    Memory(Some(DWord), Some(RCX), None, None, None),
                    Immediate(0x00),
                ),
                vec![0x83, 0x01, 0x00],
            ),
            (
                Add(
                    Memory(Some(DWord), Some(RCX), None, None, None),
                    Immediate(0x1100),
                ),
                vec![0x81, 0x01, 0x00, 0x11, 0x00, 0x00],
            ),
            (
                Add(
                    Memory(Some(DWord), Some(RCX), None, None, None),
                    Immediate(0x33221100),
                ),
                vec![0x81, 0x01, 0x00, 0x11, 0x22, 0x33],
            ),
            (
                Add(
                    Memory(Some(QWord), Some(RCX), None, None, None),
                    Immediate(0x00),
                ),
                vec![0x48, 0x83, 0x01, 0x00],
            ),
            (
                Add(
                    Memory(Some(QWord), Some(RCX), None, None, None),
                    Immediate(0x1100),
                ),
                vec![0x48, 0x81, 0x01, 0x00, 0x11, 0x00, 0x00],
            ),
            (
                Add(
                    Memory(Some(QWord), Some(RCX), None, None, None),
                    Immediate(0x33221100),
                ),
                vec![0x48, 0x81, 0x01, 0x00, 0x11, 0x22, 0x33],
            ),
        ];
        run_tests(tests);
    }
}
