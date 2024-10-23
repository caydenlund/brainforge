use crate::assembly::amd64::AMD64Operand::*;
use crate::assembly::amd64::{AMD64Instruction, AMD64Operand};
use crate::assembly::Instruction;

impl AMD64Instruction {
    pub(crate) fn encode_add(
        self: &AMD64Instruction,
        dst: &AMD64Operand,
        src: &AMD64Operand,
    ) -> Vec<u8> {
        match (dst, src) {
            // register += register
            (Register(dst_reg), Register(src_reg)) => {
                self.check_reg_size(dst_reg, src_reg);

                let prefix_reg_16 = (dst_reg.size() == 16).then_some(0x66);

                let rex = self.unwrap(Self::encode_rex(Some(src), Some(dst)));

                let opcode: u8 = if dst_reg.size() == 8 { 0x00 } else { 0x01 };

                let rmi = self.unwrap(Self::encode_reg_rmi(Some(src), Some(dst), dst_reg.size()));

                vec![prefix_reg_16, rex, Some(opcode)]
                    .into_iter()
                    .flatten()
                    .chain(rmi)
                    .collect()
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

                let prefix_addr_32 = self.encode_prefix_addr_32(base_reg, index_reg);

                let rex = self.unwrap(Self::encode_rex(Some(dst), Some(src)));

                let opcode: u8 = if dst_reg.size() == 8 { 0x02 } else { 0x03 };

                let rmi = self.unwrap(Self::encode_reg_rmi(Some(dst), Some(src), dst_reg.size()));

                vec![prefix_reg_16, prefix_addr_32, rex, Some(opcode)]
                    .into_iter()
                    .flatten()
                    .chain(rmi)
                    .collect()
            }
            // register += immediate
            (Register(dst_reg), Immediate(imm)) => {
                let prefix_reg_16 = (dst_reg.size() == 16).then_some(0x66);

                let rex = self.unwrap(Self::encode_rex(None, Some(dst)));

                let opcode: u8 = match (dst_reg.id(), dst_reg.size()) {
                    (0, 8) => 0x04,
                    (0, _) => 0x05,
                    (_, 8) => 0x80,
                    (_, _) => 0x81,
                    _ => panic!("Invalid instruction: `{}`", self.to_string()),
                };

                let imm = self.unwrap(Self::encode_imm(*imm, dst_reg.size().max(32)));

                vec![prefix_reg_16, rex, Some(opcode)]
                    .into_iter()
                    .flatten()
                    .chain(imm)
                    .collect()
            }
            // memory += register
            (Memory(size, base_reg, index_reg, index_scale, offset), Register(src_reg)) => {
                //
                todo!()
            }
            // memory += immediate
            (Memory(size, base_reg, index_reg, index_scale, offset), Immediate(src_imm)) => {
                //
                todo!()
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
        run_tests(&tests);
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
        run_tests(&tests);
    }
}