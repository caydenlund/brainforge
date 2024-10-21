use super::{AMD64Operand, AMD64Register, MemorySize, ModRM, Rex};
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

        let encode_u32 = |num: u32| {
            vec![
                ((num >> 0) & 0xff) as u8,
                ((num >> 8) & 0xff) as u8,
                ((num >> 16) & 0xff) as u8,
                ((num >> 24) & 0xff) as u8,
            ]
        };

        let mut rex = Rex::new();
        let mut mod_rm = ModRM::new();
        let mut sib: Option<u8> = None;
        let mut imm: Option<i32> = None;

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
                //
                todo!()
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
                //
                todo!()
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
