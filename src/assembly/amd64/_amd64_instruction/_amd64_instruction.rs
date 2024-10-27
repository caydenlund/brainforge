use crate::assembly::amd64::{AMD64Operand, AMD64Register, MemorySize, ModRM, Rex, Sib};
use crate::instruction::IntermediateInstruction;

use crate::{BFError, BFResult};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// Represents a callable function from the C standard library
#[derive(Copy, Clone, Debug)]
pub enum Function {
    /// `getchar` from the C standard library
    GetChar,

    /// `putchar` from the C standard library
    PutChar,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Function::GetChar => "getchar",
                Function::PutChar => "putchar",
            }
        )
    }
}

/// Represents a single assembly instruction in the AMD64 assembly specification.
#[derive(Clone, Debug)]
pub enum AMD64Instruction {
    /// `call <function>`
    Call(Function),
    /// `je <offset>`
    Je(isize, Option<String>),
    /// `jmp <offset>`
    Jmp(isize, Option<String>),
    /// `jne <offset>`
    Jne(isize, Option<String>),

    /// `add <dst>, <src>`
    Add(AMD64Operand, AMD64Operand),

    /// `bsf <dst>, <src>`
    Bsf(AMD64Operand, AMD64Operand),
    /// `bsf <dst>, <src>`
    Bsr(AMD64Operand, AMD64Operand),
    /// `cmovge <dst>, <src>`
    Cmovge(AMD64Operand, AMD64Operand),
    /// `cmp <dst>, <src>`
    Cmp(AMD64Operand, AMD64Operand),
    /// `imul <dst>, <src>`
    Imul(AMD64Operand, AMD64Operand),
    /// `lea <dst>, <src>`
    Lea(AMD64Operand, AMD64Operand),
    /// `mov <dst>, <src>`
    Mov(AMD64Operand, AMD64Operand),
    /// `movzx <dst>, <src>`
    Movzx(AMD64Operand, AMD64Operand),
    /// `xor <dst>, <src>`
    Xor(AMD64Operand, AMD64Operand),

    /// `vmovdqu <dst>, <src>`
    Vmovdqu(AMD64Operand, AMD64Operand),
    /// `vpcmpeqb <dst>, <op1>, <op2>`
    Vpcmpeqb(AMD64Operand, AMD64Operand, AMD64Operand),
    /// `vpmovmskb <dst>, <src>`
    Vpmovmskb(AMD64Operand, AMD64Operand),
    /// `vpor <dst>, <op1>, <op2>`
    Vpor(AMD64Operand, AMD64Operand, AMD64Operand),
    /// `vpxor <dst>, <op1>, <op2>`
    Vpxor(AMD64Operand, AMD64Operand, AMD64Operand),

    /// `ret`
    Ret(),
}

use AMD64Instruction::*;

impl AMD64Instruction {
    /// Converts a single abstract BF instruction into a vector of assembly instructions
    fn convert_instruction(instr: &IntermediateInstruction) -> Vec<AMD64Instruction> {
        use AMD64Instruction::*;
        use AMD64Operand::*;
        use AMD64Register::*;
        use Function::*;
        use IntermediateInstruction::*;

        let reg = |reg: AMD64Register| Register(reg);
        let imm = |val: isize| Immediate(val);
        let memory = |size: Option<MemorySize>, base: AMD64Register, offset: i32| {
            Memory(size, Some(base), None, None, Some(offset))
        };

        let mem_pos = reg(R12);
        let mem_val = memory(Some(MemorySize::Byte), R12, 0);

        match instr {
            Loop(instrs) => {
                let body = Self::convert_instructions(instrs).concat();
                let body_len = body.len();
                vec![
                    // If the current cell's value is zero,
                    // jump *over* the body *and* the following loop condition
                    vec![Cmp(mem_val, imm(0)), Je(body_len as isize + 2, None)],
                    body,
                    // If the current cell's value is zero,
                    // jump back to the beginning of the body
                    vec![Cmp(mem_val, imm(0)), Jne(-(body_len as isize + 2), None)],
                ]
                .concat()
            }

            Move(offset) => {
                vec![AMD64Instruction::Add(mem_pos, imm(*offset as isize))]
            }

            IntermediateInstruction::Add(offset) => {
                vec![AMD64Instruction::Add(mem_val, imm(*offset as isize))]
            }

            Read => {
                vec![
                    Call(GetChar),
                    Cmp(Register(EAX), Immediate(0)),
                    Mov(Register(EBX), Immediate(-1)),
                    Cmovge(Register(EBX), Register(EAX)),
                    Mov(mem_val, reg(BL)),
                ]
            }

            Write => vec![
                Xor(reg(RDI), reg(RDI)),
                Mov(reg(DIL), mem_val),
                Call(PutChar),
            ],

            AddDynamic(target, multiplier) => {
                vec![
                    Movzx(reg(R13D), mem_val),
                    Imul(reg(R13D), imm(*multiplier as isize)),
                    AMD64Instruction::Add(memory(None, R12, *target), reg(R13B)),
                ]
            }

            SimpleLoop(instrs) => {
                let body = Self::convert_instructions(instrs).concat();
                vec![
                    // Jump *over* the simple loop if the current cell's value is zero
                    vec![Cmp(mem_val, imm(0)), Je(body.len() as isize, None)],
                    body,
                ]
                .concat()
            }

            Zero => {
                vec![Mov(mem_val, imm(0))]
            }

            Scan(stride) => {
                let mut result = vec![
                    // Beginning of loop.
                    // Instruction 0
                    Vmovdqu(
                        reg(YMM3),
                        memory(
                            Some(MemorySize::YMMWord),
                            R12,
                            if *stride < 0 { -31 } else { 0 },
                        ),
                    ),
                    // Instruction 1
                    Vpxor(reg(YMM0), reg(YMM0), reg(YMM0)),
                    // Instruction 2
                    Vpor(
                        reg(YMM3),
                        reg(match stride.abs() {
                            1 => YMM1,
                            2 => YMM2,
                            4 => YMM4,
                            _ => panic!("Invalid stride: {}", stride),
                        }),
                        reg(YMM3),
                    ),
                    // Instruction 3
                    Vpcmpeqb(reg(YMM3), reg(YMM0), reg(YMM3)),
                    // Instruction 4
                    Vpmovmskb(reg(EAX), reg(YMM3)),
                    // Instruction 5
                    if *stride < 0 {
                        Bsr(reg(EAX), reg(EAX))
                    } else {
                        Bsf(reg(EAX), reg(EAX))
                    },
                    // Instruction 6
                    // Jump to end
                    Jne(2, None),
                    // Instruction 7
                    AMD64Instruction::Add(mem_pos, imm(if *stride < 0 { -32 } else { 32 })),
                    // Instruction 8
                    // Jump to beginning of loop
                    Jmp(-9, None),
                ];
                // Instruction 9 (end of loop)
                if *stride < 0 {
                    result.push(AMD64Instruction::Add(reg(RAX), imm(-31)))
                }
                // Instruction 10
                result.push(AMD64Instruction::Add(mem_pos, reg(RAX)));
                result
            }
        }
    }

    /// Converts a set of abstract BF instructions to a vector of blocks of assembly instructions
    ///
    /// Important note: all jump instructions must be patched. The offset field, rather than
    /// encoding a label name or number of displacement bytes, reports the number of instructions
    /// that must be jumped over.
    pub fn convert_instructions(instrs: &[IntermediateInstruction]) -> Vec<Vec<AMD64Instruction>> {
        let mut blocks = vec![];
        let mut current_basic_block = vec![];

        for instr in instrs {
            if matches!(instr, IntermediateInstruction::Loop(_)) {
                if !current_basic_block.is_empty() {
                    blocks.push(current_basic_block);
                    current_basic_block = vec![];
                }
                blocks.push(Self::convert_instruction(instr));
            } else {
                current_basic_block.extend(Self::convert_instruction(instr));
            }
        }
        if !current_basic_block.is_empty() {
            blocks.push(current_basic_block);
        }
        blocks
    }

    /// Converts an abstract BF instruction to a vector of strings of assembly instructions
    pub fn bf_to_assembly(
        instr: &IntermediateInstruction,
        label_counter: &mut usize,
    ) -> Vec<String> {
        let mut mk_label = || {
            let result = *label_counter;
            *label_counter += 1;
            format!(".label_{}", result)
        };

        let mut instrs = Self::convert_instruction(instr);

        let mut labels: HashMap<usize, String> = HashMap::new();

        let mut apply_label = |index: usize, offset: isize, label: &mut Option<String>| {
            let new_label = mk_label();
            *label = Some(new_label.clone());
            labels.insert(((index as isize) + offset + 1) as usize, new_label);
        };

        for index in 0..instrs.len() {
            match &mut instrs[index] {
                Je(offset, label) => apply_label(index, *offset, label),
                Jmp(offset, label) => apply_label(index, *offset, label),
                Jne(offset, label) => apply_label(index, *offset, label),
                _ => {}
            }
        }

        let mut lines = vec![];
        for index in 0..instrs.len() {
            labels
                .get(&index)
                .map(|label| lines.push(label.clone() + ":"));
            lines.push(instrs[index].to_string());
        }

        labels
            .get(&instrs.len())
            .map(|label| lines.push(label.clone() + ":"));

        lines
    }

    /// Converts an abstract BF instruction to a vector of binary-encoded instructions
    pub fn encode_block(instrs: &[AMD64Instruction]) -> BFResult<Vec<u8>> {
        use AMD64Instruction::*;

        let mut bytes = instrs
            .iter()
            .map(|instr| instr.to_binary())
            .collect::<BFResult<Vec<Vec<u8>>>>()?;

        for index in 0..instrs.len() {
            let byte_displacement = |offset: &isize| {
                let index = index as isize;
                let (sign, from, to) = if *offset > 0 {
                    (1, index + 1, index + offset)
                } else {
                    (-1, index + offset + 1, index)
                };
                let size: usize = bytes[(from as usize)..=(to as usize)]
                    .iter()
                    .map(|bv| bv.len())
                    .sum();
                sign * (size as isize)
            };
            let instr = &instrs[index];
            match instr {
                Jmp(offset, _) => {
                    bytes[index] = Jmp(byte_displacement(offset), None).to_binary()?
                }
                Je(offset, _) => bytes[index] = Je(byte_displacement(offset), None).to_binary()?,
                Jne(offset, _) => {
                    bytes[index] = Jne(byte_displacement(offset), None).to_binary()?
                }
                _ => {}
            }
        }

        Ok(bytes.concat())
    }

    /// Encodes an REX prefix for binary instructions
    pub(crate) fn encode_rex(
        &self,
        r: Option<&AMD64Operand>,
        xb: Option<&AMD64Operand>,
    ) -> BFResult<Option<u8>> {
        let mut rex = Rex::new();
        match r {
            Some(AMD64Operand::Register(r_reg)) => {
                rex.r_reg(r_reg);
            }
            None => {}
            _ => return self.encoding_err(),
        }
        match xb {
            Some(AMD64Operand::Register(b_reg)) => {
                if b_reg.size() > 32 {
                    rex.w();
                }
                rex.b_reg(b_reg);
            }
            Some(AMD64Operand::Memory(size, b_reg, x_reg, _, _)) => {
                if let Some(MemorySize::QWord) = size {
                    rex.w();
                }
                if let Some(b_reg) = b_reg {
                    rex.b_reg(b_reg);
                }
                if let Some(x_reg) = x_reg {
                    rex.x_reg(x_reg);
                }
            }
            None => {}
            _ => return self.encoding_err(),
        }
        Ok(rex.as_byte())
    }

    /// Encodes the ModR/M byte, SIB byte, and immediate bytes for binary instructions
    pub(crate) fn encode_reg_rmi(
        &self,
        reg: Option<&AMD64Operand>,
        rmi: Option<&AMD64Operand>,
        op_size: usize,
    ) -> BFResult<Vec<u8>> {
        let mut mod_rm = ModRM::new();
        let mut sib: Option<u8> = None;
        let mut imm: Option<Vec<u8>> = None;

        match reg {
            Some(AMD64Operand::Register(reg)) => {
                mod_rm.reg_reg(reg);
            }
            None => {}
            _ => return self.encoding_err(),
        }

        match rmi {
            Some(AMD64Operand::Register(reg)) => {
                mod_rm.mode(0b11);
                mod_rm.rm_reg(reg);
            }
            Some(AMD64Operand::Memory(size, base_reg, index_reg, index_scale, displacement)) => {
                if let Some(size) = size {
                    if size.size() != op_size {
                        return self.encoding_err();
                    }
                }
                let (make_sib, make_displacement) = if let Some(base_reg) = base_reg {
                    match (base_reg.id() & 7, index_reg, displacement) {
                        (0b101, _, _) => (index_reg.is_some(), true),
                        (_, Some(_), _) => (true, displacement.is_some()),
                        (0b100, _, _) => (true, displacement.is_some()),
                        _ => (false, displacement.is_some()),
                    }
                } else {
                    // We need to encode a 32-bit displacement with a ModR/M mode of 0.
                    mod_rm.mode(0b00);
                    imm = Some(self.encode_imm(displacement.unwrap_or(0) as isize, 32)?);
                    (true, false)
                };
                if make_sib {
                    mod_rm.rm(0b100);
                    let mut new_sib = Sib::new();
                    match index_scale.unwrap_or(1) {
                        1 => new_sib.scale(0b00),
                        2 => new_sib.scale(0b01),
                        4 => new_sib.scale(0b10),
                        8 => new_sib.scale(0b11),
                        _ => return self.encoding_err(),
                    }
                    if let Some(index_reg) = index_reg {
                        new_sib.index_reg(index_reg);
                    } else {
                        new_sib.index(0b100);
                    }
                    if let Some(base_reg) = base_reg {
                        new_sib.base_reg(base_reg);
                    } else {
                        new_sib.base(0b101);
                    }
                    sib = Some(new_sib.as_byte());
                } else {
                    if let Some(base_reg) = base_reg {
                        mod_rm.rm_reg(base_reg);
                    }
                }
                if make_displacement {
                    let displacement = displacement.unwrap_or(0);
                    match displacement {
                        -0x80..0x80 => {
                            mod_rm.mode(0b01);
                            imm = Some(self.encode_imm(displacement as isize, 8)?);
                        }
                        _ => {
                            mod_rm.mode(0b10);
                            imm = Some(self.encode_imm(displacement as isize, 32)?);
                        }
                    }
                }
            }
            Some(AMD64Operand::Immediate(imm_val)) => {
                mod_rm.mode(0b11);
                imm = Some(self.encode_imm(*imm_val, op_size)?);
            }
            None => {
                //
                todo!()
            }
        }

        let mut result = vec![mod_rm.as_byte()];
        sib.map(|sib| result.push(sib));
        imm.map(|imm| result.extend(imm));

        Ok(result)
    }

    pub(crate) fn encoding_err<T>(&self) -> BFResult<T> {
        Err(BFError::EncodeError(self.clone()))
    }

    /// For the given base register and index register, return prefix 0x67 as necessary
    pub(crate) fn encode_prefix_addr_32(
        &self,
        base_reg: &Option<AMD64Register>,
        index_reg: &Option<AMD64Register>,
    ) -> BFResult<Option<u8>> {
        let make_prefix = |size: usize| match size {
            32 => Ok(Some(0x67)),
            64 => Ok(None),
            _ => self.encoding_err(),
        };

        match (base_reg, index_reg) {
            (Some(base_reg), Some(index_reg)) => {
                if base_reg.size() != index_reg.size() {
                    return self.encoding_err();
                }
                make_prefix(base_reg.size())
            }
            (Some(base_reg), _) => make_prefix(base_reg.size()),
            (_, Some(index_reg)) => make_prefix(index_reg.size()),
            (_, _) => Ok(None),
        }
    }

    /// Given an immediate value and a size, encode the immediate value
    pub(crate) fn encode_imm(&self, imm: isize, size: usize) -> BFResult<Vec<u8>> {
        match size {
            8 => Ok(Vec::from((imm as u8).to_le_bytes())),
            16 => Ok(Vec::from((imm as u16).to_le_bytes())),
            32 => Ok(Vec::from((imm as u32).to_le_bytes())),
            64 => Ok(Vec::from((imm as u64).to_le_bytes())),
            _ => self.encoding_err(),
        }
    }

    /// Returns a string representation of this instruction
    pub fn to_string(&self) -> String {
        use AMD64Instruction::*;
        match self {
            Call(func) => format!("call {}", func),
            Je(displacement, label) => {
                format!("je {}", label.clone().unwrap_or(displacement.to_string()))
            }
            Jmp(displacement, label) => {
                format!("jmp {}", label.clone().unwrap_or(displacement.to_string()))
            }
            Jne(displacement, label) => {
                format!("jne {}", label.clone().unwrap_or(displacement.to_string()))
            }

            Add(dst, src) => format!("add {}, {}", dst, src),
            Bsf(dst, src) => format!("bsf {}, {}", dst, src),
            Bsr(dst, src) => format!("bsr {}, {}", dst, src),
            Cmovge(dst, src) => format!("cmovge {}, {}", dst, src),
            Cmp(dst, src) => format!("cmp {}, {}", dst, src),
            Imul(dst, src) => format!("imul {}, {}", dst, src),
            Lea(dst, src) => format!("lea {}, {}", dst, src),
            Mov(dst, src) => format!("mov {}, {}", dst, src),
            Movzx(dst, src) => format!("movzx {}, {}", dst, src),
            Xor(dst, src) => format!("xor {}, {}", dst, src),

            Vmovdqu(dst, src) => format!("vmovdqu {}, {}", dst, src),
            Vpmovmskb(dst, src) => format!("vpmovmskb {}, {}", dst, src),
            Vpcmpeqb(dst, op1, op2) => {
                format!("vpcmpeqb {}, {}, {}", dst, op1, op2)
            }
            Vpor(dst, op1, op2) => format!("vpor {}, {}, {}", dst, op1, op2),
            Vpxor(dst, op1, op2) => {
                format!("vpxor {}, {}, {}", dst, op1, op2)
            }

            Ret() => "ret".into(),
        }
    }

    /// Encodes this instruction in binary
    pub fn to_binary(&self) -> BFResult<Vec<u8>> {
        use AMD64Instruction::*;

        match self {
            Call(func) => self.encode_call(func),
            Je(displacement, _) => self.encode_je(*displacement),
            Jmp(displacement, _) => self.encode_jmp(*displacement),
            Jne(displacement, _) => self.encode_jne(*displacement),

            Add(dst, src) => self.encode_add(dst, src),
            Bsf(dst, src) => self.encode_bsf(dst, src),
            Bsr(dst, src) => self.encode_bsr(dst, src),
            Cmovge(dst, src) => self.encode_cmovge(dst, src),
            Cmp(dst, src) => self.encode_cmp(dst, src),
            Imul(dst, src) => self.encode_imul(dst, src),
            Lea(dst, src) => self.encode_lea(dst, src),
            Mov(dst, src) => self.encode_mov(dst, src),
            Movzx(dst, src) => self.encode_movzx(dst, src),
            Xor(dst, src) => self.encode_xor(dst, src),

            Vmovdqu(dst, src) => self.encode_vmovdqu(dst, src),
            Vpmovmskb(dst, src) => self.encode_vpmovmskb(dst, src),
            Vpcmpeqb(dst, op1, op2) => self.encode_vpcmpeqb(dst, op1, op2),
            Vpor(dst, op1, op2) => self.encode_vpor(dst, op1, op2),
            Vpxor(dst, op1, op2) => self.encode_vpxor(dst, op1, op2),

            Ret() => Ok(vec![0xC3]),
        }
    }
}
