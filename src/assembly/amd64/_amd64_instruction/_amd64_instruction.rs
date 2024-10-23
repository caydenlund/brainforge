use crate::assembly::amd64::{AMD64Operand, AMD64Register, MemorySize, ModRM, Rex, Sib};
use crate::assembly::Instruction;
use crate::instruction::IntermediateInstruction;

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
    Je(isize),
    /// `jmp <offset>`
    Jmp(isize),
    /// `jne <offset>`
    Jne(isize),
    /// `jnz <offset>`
    Jnz(isize),

    /// `add <dst>, <src>`
    Add(AMD64Operand, AMD64Operand),

    /// `bsf <dst>, <src>`
    Bsf(AMD64Operand, AMD64Operand),
    /// `bsf <dst>, <src>`
    Bsr(AMD64Operand, AMD64Operand),
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
}

impl AMD64Instruction {
    /// Converts a single abstract BF instruction into a vector of assembly instructions
    fn convert_instruction(instr: &IntermediateInstruction) -> Vec<AMD64Instruction> {
        use AMD64Instruction::*;
        use AMD64Register::*;
        use Function::*;
        use IntermediateInstruction::*;

        let reg = |reg: AMD64Register| AMD64Operand::Register(reg);
        let imm = |val: isize| AMD64Operand::Immediate(val);
        let memory = |size: Option<MemorySize>, base: AMD64Register, offset: i32| {
            AMD64Operand::Memory(size, Some(base), None, None, Some(offset))
        };

        let mem_pos = reg(R12);
        let mem_val = memory(Some(MemorySize::Byte), R12, 0);

        match instr {
            Loop(instrs) => {
                let body = Self::convert_instructions(instrs);
                let body_len = body.len();
                vec![
                    // If the current cell's value is zero,
                    // jump *over* the body *and* the following loop condition
                    vec![Cmp(mem_val, imm(0)), Je(body_len as isize + 2)],
                    body,
                    // If the current cell's value is zero,
                    // jump back to the beginning of the body
                    vec![Cmp(mem_val, imm(0)), Jne(-(body_len as isize + 2))],
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
                vec![Call(GetChar), Mov(mem_val, reg(AL))]
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
                let body = Self::convert_instructions(instrs);
                vec![
                    // Jump *over* the simple loop if the current cell's value is zero
                    vec![Cmp(mem_val, imm(0)), Je(body.len() as isize)],
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
                    Jnz(2),
                    // Instruction 7
                    AMD64Instruction::Add(mem_pos, imm(if *stride < 0 { -32 } else { 32 })),
                    // Instruction 8
                    // Jump to beginning of loop
                    Jmp(-9),
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

    /// Converts a set of abstract BF instructions to a vector of assembly instructions
    fn convert_instructions(instrs: &[IntermediateInstruction]) -> Vec<AMD64Instruction> {
        instrs
            .iter()
            .map(|instr| Self::convert_instruction(instr))
            .collect::<Vec<Vec<AMD64Instruction>>>()
            .concat()
    }

    /// Converts an abstract BF instruction to a vector of strings of assembly instructions
    pub fn bf_to_assembly(
        instr: &IntermediateInstruction,
        label_counter: &mut usize,
    ) -> Vec<String> {
        let instrs = Self::convert_instruction(instr);

        let mut labels = HashMap::new();

        for index in 0..instrs.len() {
            let mut add_label = |offset: isize| {
                let key = ((index as isize) + offset + 1) as usize;
                if !labels.contains_key(&key) {
                    labels.insert(key, *label_counter);
                    *label_counter += 1;
                }
            };
            match instrs[index] {
                AMD64Instruction::Je(offset) => add_label(offset),
                AMD64Instruction::Jmp(offset) => add_label(offset),
                AMD64Instruction::Jne(offset) => add_label(offset),
                AMD64Instruction::Jnz(offset) => add_label(offset),
                _ => {}
            }
        }

        let mut lines = vec![];
        for index in 0..instrs.len() {
            let get_label =
                |offset: &isize| labels.get(&(((index as isize) + offset + 1) as usize));

            if labels.contains_key(&index) {
                lines.push(format!(".label_{}:", labels.get(&index).unwrap()));
            }
            if index == instrs.len() - 1 && labels.contains_key(&(index + 1)) {
                lines.push(format!(".label_{}:", labels.get(&(index + 1)).unwrap()));
            }

            let instr = &instrs[index];
            lines.push(match instr {
                AMD64Instruction::Je(offset) => instr.to_string_with_label(get_label(offset)),
                AMD64Instruction::Jmp(offset) => instr.to_string_with_label(get_label(offset)),
                AMD64Instruction::Jne(offset) => instr.to_string_with_label(get_label(offset)),
                AMD64Instruction::Jnz(offset) => instr.to_string_with_label(get_label(offset)),
                _ => instr.to_string(),
            });
        }
        lines
    }

    /// Converts an abstract BF instruction to a vector of binary-encoded instructions
    pub fn bf_to_binary(instr: &IntermediateInstruction) -> Vec<u8> {
        use AMD64Instruction::*;

        let instrs = Self::convert_instruction(instr);
        let mut bytes = instrs
            .iter()
            .map(|instr| instr.to_binary())
            .collect::<Vec<Vec<u8>>>();

        for index in 0..instrs.len() {
            let byte_jump = |offset: &isize| {
                let sign = if *offset > 0 { 1 } else { -1 };
                let size: usize = bytes[(((index as isize) + offset) as usize)..index]
                    .iter()
                    .map(|bv| bv.len())
                    .sum();
                sign * (size as isize)
            };
            let instr = &instrs[index];
            match instr {
                Je(offset) => bytes[index] = instr.to_binary_with_offset(byte_jump(offset)),
                Jmp(offset) => bytes[index] = instr.to_binary_with_offset(byte_jump(offset)),
                Jne(offset) => bytes[index] = instr.to_binary_with_offset(byte_jump(offset)),
                Jnz(offset) => bytes[index] = instr.to_binary_with_offset(byte_jump(offset)),
                _ => {}
            }
        }

        bytes.concat()
    }

    /// Like `to_string`, but with added label names instead of index offsets
    fn to_string_with_label(&self, label: Option<&usize>) -> String {
        use AMD64Instruction::*;
        match self {
            Je(_) => format!("    je .label_{}", label.unwrap()),
            Jmp(_) => format!("    jmp .label_{}", label.unwrap()),
            Jne(_) => format!("    jne .label_{}", label.unwrap()),
            Jnz(_) => format!("    jnz .label_{}", label.unwrap()),
            _ => self.to_string(),
        }
    }

    /// Like `to_binary`, but with added byte offsets instead of index offsets
    fn to_binary_with_offset(&self, offset: isize) -> Vec<u8> {
        use AMD64Instruction::*;
        match self {
            Je(_) => Je(offset).to_binary(),
            Jmp(_) => Jmp(offset).to_binary(),
            Jne(_) => Jne(offset).to_binary(),
            Jnz(_) => Jnz(offset).to_binary(),
            _ => self.to_binary(),
        }
    }

    /// Converts an immediate value to an LE byte vector of size 1
    pub(crate) fn bytes_8(imm: isize) -> Vec<u8> {
        Vec::from((imm as u8).to_le_bytes())
    }

    /// Converts an immediate value to an LE byte vector of size 2
    pub(crate) fn bytes_16(imm: isize) -> Vec<u8> {
        Vec::from((imm as u16).to_le_bytes())
    }

    /// Converts an immediate value to an LE byte vector of size 4
    pub(crate) fn bytes_32(imm: isize) -> Vec<u8> {
        Vec::from((imm as u32).to_le_bytes())
    }

    /// Converts an immediate value to an LE byte vector of size 8
    pub(crate) fn bytes_64(imm: isize) -> Vec<u8> {
        Vec::from((imm as u64).to_le_bytes())
    }

    /// Encodes an REX prefix for binary instructions
    pub(crate) fn encode_rex(
        r: Option<&AMD64Operand>,
        xb: Option<&AMD64Operand>,
    ) -> Result<Option<u8>, String> {
        let mut rex = Rex::new();
        match r {
            Some(AMD64Operand::Register(r_reg)) => {
                rex.r_reg(r_reg);
            }
            None => {}
            _ => return Err("Invalid register operand for REX prefix".into()),
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
            _ => return Err("Invalid index/base operand for REX prefix".into()),
        }
        Ok(rex.as_byte())
    }

    /// Encodes the ModR/M byte, SIB byte, and immediate bytes for binary instructions
    pub(crate) fn encode_reg_rmi(
        reg: Option<&AMD64Operand>,
        rmi: Option<&AMD64Operand>,
        op_size: usize,
    ) -> Result<Vec<u8>, String> {
        let mut mod_rm = ModRM::new();
        let mut sib: Option<u8> = None;
        let mut imm: Option<Vec<u8>> = None;

        match reg {
            Some(AMD64Operand::Register(reg)) => {
                mod_rm.reg_reg(reg);
            }
            None => {}
            _ => {
                return Err(format!(
                    "Invalid ModR/M register operand: `{}`",
                    reg.unwrap()
                ))
            }
        }

        match rmi {
            Some(AMD64Operand::Register(reg)) => {
                mod_rm.mode(0b11);
                mod_rm.rm_reg(reg);
            }
            Some(AMD64Operand::Memory(size, base_reg, index_reg, index_scale, displacement)) => {
                if let Some(size) = size {
                    if size.size() != op_size {
                        return Err("Memory size mismatch".into());
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
                    imm = Some(Self::bytes_32(displacement.unwrap_or(0) as isize));
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
                        _ => return Err("Illegal memory index scale".into()),
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
                            imm = Some(Self::bytes_8(displacement as isize));
                        }
                        _ => {
                            mod_rm.mode(0b10);
                            imm = Some(Self::bytes_32(displacement as isize));
                        }
                    }
                }
            }
            Some(AMD64Operand::Immediate(imm_val)) => {
                mod_rm.mode(0b11);
                match op_size {
                    8 => {
                        imm = Some(Self::bytes_8(*imm_val));
                    }
                    16 => {
                        imm = Some(Self::bytes_16(*imm_val));
                    }
                    32 => {
                        imm = Some(Self::bytes_32(*imm_val));
                    }
                    64 => {
                        imm = Some(Self::bytes_64(*imm_val));
                    }
                    _ => return Err("Invalid immediate size".into()),
                }
            }
            None => {
                //
                todo!()
            }
        }

        let mut result = vec![mod_rm.as_byte()];
        if let Some(sib) = sib {
            result.push(sib);
        }
        if let Some(imm) = imm {
            result.extend(imm);
        }

        Ok(result)
    }

    /// Unwraps a `Result<_, String>` with an added error message pertaining to this instruction
    pub(crate) fn unwrap<T>(&self, val: Result<T, String>) -> T {
        val.unwrap_or_else(|msg| panic!("{}: `{}`", msg, self.to_string()))
    }

    /// Ensures that two registers have the same size
    pub(crate) fn check_reg_size(&self, reg1: &AMD64Register, reg2: &AMD64Register) {
        assert_eq!(
            reg1.size(),
            reg2.size(),
            "Register size mismatch: `{}`",
            self.to_string()
        );
    }

    /// For the given base register and index register, return prefix 0x67 as necessary
    pub(crate) fn encode_prefix_addr_32(
        &self,
        base_reg: &Option<AMD64Register>,
        index_reg: &Option<AMD64Register>,
    ) -> Option<u8> {
        let make_prefix = |size: usize| match size {
            32 => Some(0x67),
            64 => None,
            _ => panic!("Memory invalid register size: `{}`", self.to_string()),
        };

        match (base_reg, index_reg) {
            (Some(base_reg), Some(index_reg)) => {
                self.check_reg_size(base_reg, index_reg);
                make_prefix(base_reg.size())
            }
            (Some(base_reg), _) => make_prefix(base_reg.size()),
            (_, Some(index_reg)) => make_prefix(index_reg.size()),
            (_, _) => None,
        }
    }

    /// Given an immediate value and a size, encode the immediate value
    pub(crate) fn encode_imm(imm: isize, size: usize) -> Result<Vec<u8>, String> {
        match size {
            8 => Ok(Self::bytes_8(imm)),
            16 => Ok(Self::bytes_16(imm)),
            32 => Ok(Self::bytes_32(imm)),
            64 => Ok(Self::bytes_64(imm)),
            _ => Err("Invalid immediate size".into()),
        }
    }
}

impl Instruction for AMD64Instruction {
    fn to_string(&self) -> String {
        use AMD64Instruction::*;
        match self {
            Call(func) => format!("    call {}", func),
            Je(displacement) => format!("    je {}", displacement),
            Jmp(displacement) => format!("    jmp {}", displacement),
            Jne(displacement) => format!("    jne {}", displacement),
            Jnz(displacement) => format!("    jnz {}", displacement),

            Add(dst, src) => format!("    add {}, {}", dst, src),
            Bsf(dst, src) => format!("    bsf {}, {}", dst, src),
            Bsr(dst, src) => format!("    bsr {}, {}", dst, src),
            Cmp(dst, src) => format!("    cmp {}, {}", dst, src),
            Imul(dst, src) => format!("    imul {}, {}", dst, src),
            Lea(dst, src) => format!("    lea {}, {}", dst, src),
            Mov(dst, src) => format!("    mov {}, {}", dst, src),
            Movzx(dst, src) => format!("    movzx {}, {}", dst, src),
            Xor(dst, src) => format!("    xor {}, {}", dst, src),

            Vmovdqu(dst, src) => format!("    vmovdqu {}, {}", dst, src),
            Vpmovmskb(dst, src) => format!("    vpmovmskb {}, {}", dst, src),
            Vpcmpeqb(dst, op1, op2) => {
                format!("    vpcmpeqb {}, {}, {}", dst, op1, op2)
            }
            Vpor(dst, op1, op2) => format!("    vpor {}, {}, {}", dst, op1, op2),
            Vpxor(dst, op1, op2) => {
                format!("    vpxor {}, {}, {}", dst, op1, op2)
            }
        }
    }

    fn to_binary(&self) -> Vec<u8> {
        use AMD64Instruction::*;

        match self {
            Call(func) => self.encode_call(func),
            Je(displacement) => self.encode_je(*displacement),
            Jmp(displacement) => self.encode_jmp(*displacement),
            Jne(displacement) => self.encode_jne(*displacement),
            Jnz(displacement) => self.encode_jnz(*displacement),

            Add(dst, src) => self.encode_add(dst, src),
            Bsf(dst, src) => self.encode_bsf(dst, src),
            Bsr(dst, src) => self.encode_bsr(dst, src),
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
        }
    }
}
