use super::{AMD64Operand, AMD64Register, MemorySize, ModRM, Rex, Sib};
use crate::assembly::Instruction;
use crate::instruction::IntermediateInstruction;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum Function {
    GetChar,
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

#[derive(Clone, Debug)]
pub enum AMD64Instruction {
    Call(Function),
    Je(isize),
    Jmp(isize),
    Jne(isize),
    Jnz(isize),

    Add(AMD64Operand, AMD64Operand),
    Bsf(AMD64Operand, AMD64Operand),
    Bsr(AMD64Operand, AMD64Operand),
    Cmp(AMD64Operand, AMD64Operand),
    Imul(AMD64Operand, AMD64Operand),
    Lea(AMD64Operand, AMD64Operand),
    Mov(AMD64Operand, AMD64Operand),
    Movzx(AMD64Operand, AMD64Operand),
    Xor(AMD64Operand, AMD64Operand),

    Vmovdqu(AMD64Operand, AMD64Operand),
    Vpcmpeqb(AMD64Operand, AMD64Operand, AMD64Operand),
    Vpmovmskb(AMD64Operand, AMD64Operand),
    Vpor(AMD64Operand, AMD64Operand, AMD64Operand),
    Vpxor(AMD64Operand, AMD64Operand, AMD64Operand),
}

impl AMD64Instruction {
    fn convert_instruction(instr: &IntermediateInstruction) -> Vec<AMD64Instruction> {
        use AMD64Instruction::*;
        use AMD64Register::*;
        use Function::*;
        use IntermediateInstruction::*;

        let reg = |reg: AMD64Register| AMD64Operand::Register(reg);
        let imm = |val: i32| AMD64Operand::Immediate(val);
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
                vec![AMD64Instruction::Add(mem_pos, imm(*offset))]
            }

            IntermediateInstruction::Add(offset) => {
                vec![AMD64Instruction::Add(mem_val, imm(*offset))]
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
                    Imul(reg(R13D), imm(*multiplier)),
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

    fn convert_instructions(instrs: &[IntermediateInstruction]) -> Vec<AMD64Instruction> {
        instrs
            .iter()
            .map(|instr| Self::convert_instruction(instr))
            .collect::<Vec<Vec<AMD64Instruction>>>()
            .concat()
    }

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

    fn to_string_with_label(&self, label: Option<&usize>) -> String {
        match self {
            AMD64Instruction::Je(_) => format!("    je .label_{}", label.unwrap()),
            AMD64Instruction::Jmp(_) => format!("    jmp .label_{}", label.unwrap()),
            AMD64Instruction::Jne(_) => format!("    jne .label_{}", label.unwrap()),
            AMD64Instruction::Jnz(_) => format!("    jnz .label_{}", label.unwrap()),
            _ => self.to_string(),
        }
    }

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
}

impl Instruction for AMD64Instruction {
    fn to_string(&self) -> String {
        match self {
            AMD64Instruction::Call(tgt) => format!("    call {}", tgt),
            AMD64Instruction::Je(tgt) => format!("    je {}", tgt),
            AMD64Instruction::Jmp(tgt) => format!("    jmp {}", tgt),
            AMD64Instruction::Jne(tgt) => format!("    jne {}", tgt),
            AMD64Instruction::Jnz(tgt) => format!("    jnz {}", tgt),

            AMD64Instruction::Add(dst, src) => format!("    add {}, {}", dst, src),
            AMD64Instruction::Bsf(dst, src) => format!("    bsf {}, {}", dst, src),
            AMD64Instruction::Bsr(dst, src) => format!("    bsr {}, {}", dst, src),
            AMD64Instruction::Cmp(dst, src) => format!("    cmp {}, {}", dst, src),
            AMD64Instruction::Imul(dst, src) => format!("    imul {}, {}", dst, src),
            AMD64Instruction::Lea(dst, src) => format!("    lea {}, {}", dst, src),
            AMD64Instruction::Mov(dst, src) => format!("    mov {}, {}", dst, src),
            AMD64Instruction::Movzx(dst, src) => format!("    movzx {}, {}", dst, src),
            AMD64Instruction::Xor(dst, src) => format!("    xor {}, {}", dst, src),

            AMD64Instruction::Vmovdqu(dst, src) => format!("    vmovdqu {}, {}", dst, src),
            AMD64Instruction::Vpmovmskb(dst, src) => format!("    vpmovmskb {}, {}", dst, src),
            AMD64Instruction::Vpcmpeqb(dst, op1, op2) => {
                format!("    vpcmpeqb {}, {}, {}", dst, op1, op2)
            }
            AMD64Instruction::Vpor(dst, op1, op2) => format!("    vpor {}, {}, {}", dst, op1, op2),
            AMD64Instruction::Vpxor(dst, op1, op2) => {
                format!("    vpxor {}, {}, {}", dst, op1, op2)
            }
        }
    }

    fn to_binary(&self) -> Vec<u8> {
        use AMD64Instruction::*;
        use AMD64Operand::*;

        let bytes_8 = |imm: i32| (imm as u8).to_le_bytes();
        let bytes_16 = |imm: i32| (imm as u16).to_le_bytes();
        let bytes_32 = |imm: i32| (imm as u32).to_le_bytes();

        match self {
            Call(tgt) => {
                //
                todo!()
            }
            Je(tgt) => {
                //
                todo!()
            }

            Jmp(tgt) => {
                //
                todo!()
            }
            Jne(tgt) => {
                //
                todo!()
            }
            Jnz(tgt) => {
                //
                todo!()
            }

            Add(dst, src) => {
                match (dst, src) {
                    // register += register
                    (Register(dst_reg), Register(src_reg)) => {
                        let size_prefix_16: Option<u8> = if dst_reg.size() == 16 {
                            Some(0x66)
                        } else {
                            None
                        };

                        let rex = {
                            let mut rex = Rex::new();
                            rex.r_reg(src_reg);
                            rex.b_reg(dst_reg);
                            rex.as_byte()
                        };

                        let mod_rm = {
                            let mut mod_rm = ModRM::new();
                            mod_rm.mode(3);
                            mod_rm.reg_reg(src_reg);
                            mod_rm.rm_reg(dst_reg);
                            mod_rm.as_byte()
                        };

                        if dst_reg.size() != src_reg.size() {
                            panic!("Mismatched register sizes: `{}`", self.to_string());
                        }
                        let opcode: u8 = match dst_reg.size() {
                            8 => 0x00,
                            16 | 32 | 64 => 0x01,
                            _ => panic!("Invalid instruction: `{}`", self.to_string()),
                        };

                        vec![size_prefix_16, rex, Some(opcode), Some(mod_rm)]
                            .into_iter()
                            .flatten()
                            .collect()
                    }
                    // register += memory
                    (Register(dst_reg), Memory(size, base_reg, index_reg, index_scale, offset)) => {
                        if let Some(size) = size {
                            if dst_reg.size() != size.size() {
                                panic!("Operand size mismatch: `{}`", self.to_string())
                            }
                        }

                        let (rex, mem_size_prefix_32) = {
                            let mut rex = Rex::new();
                            rex.r_reg(dst_reg);
                            let mut index_size = None;
                            let mut mem_size_prefix_32: Option<u8> = None;
                            if let Some(index_reg) = index_reg {
                                index_size = Some(index_reg.size());
                                if index_reg.size() == 32 {
                                    mem_size_prefix_32 = Some(0x67);
                                } else if index_reg.size() != 64 {
                                    panic!("Invalid index size: `{}`", self.to_string());
                                }
                                if index_reg.id() > 7 {
                                    rex.x();
                                }
                            }
                            if let Some(base_reg) = base_reg {
                                if let Some(index_size) = index_size {
                                    if index_size != base_reg.size() {
                                        panic!(
                                            "Memory operand size mismatch: `{}`",
                                            self.to_string()
                                        );
                                    }
                                }
                                if base_reg.size() == 32 {
                                    mem_size_prefix_32 = Some(0x67);
                                } else if base_reg.size() != 64 {
                                    panic!("Invalid base size: `{}`", self.to_string());
                                }
                                if base_reg.id() > 7 {
                                    rex.b();
                                }
                            }
                            (rex.as_byte(), mem_size_prefix_32)
                        };

                        let reg_size_prefix_16: Option<u8> = if dst_reg.size() == 16 {
                            Some(0x66)
                        } else {
                            None
                        };

                        let opcode: u8 = if dst_reg.size() == 8 { 0x02 } else { 0x03 };

                        let (mod_rm, offset, sib) = {
                            let make_offset = |offset: i32, mod_rm: &mut ModRM| -> Vec<u8> {
                                match offset {
                                    -0x80..0x80 => {
                                        mod_rm.mode(0b01);
                                        vec![offset as u8]
                                    }
                                    _ => {
                                        mod_rm.mode(0b10);
                                        Vec::from(offset.to_le_bytes())
                                    }
                                }
                            };
                            let make_sib =
                                |mod_rm: &mut ModRM, offset: &mut Option<Vec<u8>>| -> u8 {
                                    mod_rm.rm(0b100);
                                    let mut sib = Sib::new();
                                    match base_reg {
                                        Some(base_reg) => {
                                            sib.base((base_reg.id() & 7) as u8);
                                        }
                                        None => {
                                            sib.base(0b101);
                                            mod_rm.mode(0);
                                            if let Some(inner_offset) = offset {
                                                if inner_offset.len() == 1 {
                                                    *offset = Some(vec![inner_offset[0], 0, 0, 0]);
                                                }
                                            } else {
                                                *offset = Some(vec![0, 0, 0, 0]);
                                            }
                                        }
                                    }
                                    match index_reg {
                                        Some(index_reg) => {
                                            if index_reg.id() == 0b100 {
                                                panic!(
                                                    "Illegal index register: `{}`",
                                                    self.to_string()
                                                );
                                            }
                                            sib.index((index_reg.id() & 7) as u8);
                                            match index_scale.unwrap_or(1) {
                                                1 => {
                                                    sib.scale(0b00);
                                                }
                                                2 => {
                                                    sib.scale(0b01);
                                                }
                                                4 => {
                                                    sib.scale(0b10);
                                                }
                                                8 => {
                                                    sib.scale(0b11);
                                                }
                                                index_scale => {
                                                    panic!("Illegal index scale: `{}`", index_scale)
                                                }
                                            }
                                        }
                                        None => {
                                            sib.index(0b100);
                                        }
                                    }
                                    sib.as_byte()
                                };

                            let mut mod_rm = ModRM::new();
                            let mut offset: Option<Vec<u8>> = match offset {
                                None => None,
                                Some(val) => Some(make_offset(*val, &mut mod_rm)),
                            };

                            mod_rm.reg_reg(dst_reg);

                            let sib = {
                                if index_reg.is_some() {
                                    Some(make_sib(&mut mod_rm, &mut offset))
                                } else if let Some(base_reg) = base_reg {
                                    match (base_reg.id() & 7) as u8 {
                                        0b100 => Some(make_sib(&mut mod_rm, &mut offset)),
                                        0b101 => {
                                            if offset.is_none() {
                                                offset = Some(make_offset(0, &mut mod_rm));
                                            }
                                            None
                                        }
                                        id => {
                                            mod_rm.rm(id);
                                            None
                                        }
                                    }
                                } else {
                                    Some(make_sib(&mut mod_rm, &mut offset))
                                }
                            };
                            (mod_rm.as_byte(), offset, sib)
                        };

                        let mut result: Vec<u8> = vec![
                            mem_size_prefix_32,
                            reg_size_prefix_16,
                            rex,
                            Some(opcode),
                            Some(mod_rm),
                            sib,
                        ]
                        .into_iter()
                        .flatten()
                        .collect();
                        if let Some(offset) = offset {
                            result.extend(offset);
                        }
                        for byte in &result {
                            print!("{:02x} ", byte);
                        }
                        println!();
                        result
                    }
                    // register += immediate
                    (Register(dst_reg), Immediate(src_imm)) => {
                        let size_prefix_16: Option<u8> = if dst_reg.size() == 16 {
                            Some(0x66)
                        } else {
                            None
                        };

                        let rex = {
                            let mut rex = Rex::new();
                            rex.b_reg(dst_reg);
                            rex.as_byte()
                        };

                        let mod_rm = if dst_reg.id() == 0 {
                            None
                        } else {
                            let mut mod_rm = ModRM::new();
                            mod_rm.mode(3);
                            mod_rm.rm_reg(dst_reg);
                            Some(mod_rm.as_byte())
                        };

                        let (opcode, imm_bytes): (u8, &[u8]) = match (dst_reg.id(), dst_reg.size())
                        {
                            (0, 8) => (0x04, &bytes_8(*src_imm)),
                            (0, 16) => (0x05, &bytes_16(*src_imm)),
                            (0, 32) => (0x05, &bytes_32(*src_imm)),
                            (0, 64) => (0x05, &bytes_32(*src_imm)),
                            (id, 8) => (0x80, &bytes_8(*src_imm)),
                            (id, 16) => (0x81, &bytes_16(*src_imm)),
                            (id, 32) => (0x81, &bytes_32(*src_imm)),
                            (id, 64) => (0x81, &bytes_32(*src_imm)),
                            _ => panic!("Invalid instruction: `{}`", self.to_string()),
                        };

                        let mut result: Vec<u8> = vec![size_prefix_16, rex, Some(opcode), mod_rm]
                            .into_iter()
                            .flatten()
                            .collect();
                        result.extend_from_slice(imm_bytes);
                        result
                    }
                    // memory += register
                    (Memory(size, base_reg, index_reg, index_scale, offset), Register(src_reg)) => {
                        //
                        todo!()
                    }
                    // memory += immediate
                    (
                        Memory(size, base_reg, index_reg, index_scale, offset),
                        Immediate(src_imm),
                    ) => {
                        //
                        todo!()
                    }
                    (_, _) => panic!("Invalid instruction: `{}`", self.to_string()),
                }
            }
            Bsf(dst, src) => {
                //
                todo!()
            }
            Bsr(dst, src) => {
                //
                todo!()
            }
            Cmp(dst, src) => {
                //
                todo!()
            }
            Imul(dst, src) => {
                //
                todo!()
            }
            Lea(dst, src) => {
                //
                todo!()
            }
            Mov(dst, src) => {
                if let AMD64Operand::Register(src_reg) = src {
                    if let AMD64Operand::Register(dst_reg) = dst {
                        let mut rex = Rex::new();
                        rex.r_reg(src_reg);
                        rex.b_reg(dst_reg);

                        let mut mod_rm = ModRM::new();
                        mod_rm.mode(3);
                        mod_rm.reg_reg(src_reg);
                        mod_rm.rm_reg(dst_reg);

                        let mut result = vec![];
                        if rex.is_some() {
                            result.push(rex.as_byte().unwrap());
                        }
                        result.push(0x89);
                        result.push(mod_rm.as_byte());

                        result
                    } else {
                        //
                        todo!()
                    }
                } else {
                    //
                    todo!()
                }
            }
            Movzx(dst, src) => {
                //
                todo!()
            }
            Xor(dst, src) => {
                //
                todo!()
            }

            Vmovdqu(dst, src) => {
                //
                todo!()
            }
            Vpmovmskb(dst, src) => {
                //
                todo!()
            }
            Vpcmpeqb(dst, op1, op2) => {
                //
                todo!()
            }
            Vpor(dst, op1, op2) => {
                //
                todo!()
            }
            Vpxor(dst, op1, op2) => {
                //
                todo!()
            }
        }
    }
}
