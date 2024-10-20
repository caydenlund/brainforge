use super::{AMD64Register, ModRM, Rex};
use crate::assembly::{Instruction, Operand};
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

    Addb(Operand<AMD64Register>, Operand<AMD64Register>),
    Addq(Operand<AMD64Register>, Operand<AMD64Register>),
    Bsf(Operand<AMD64Register>, Operand<AMD64Register>),
    Bsr(Operand<AMD64Register>, Operand<AMD64Register>),
    Cmpb(Operand<AMD64Register>, Operand<AMD64Register>),
    Imul(Operand<AMD64Register>, Operand<AMD64Register>),
    Leaq(Operand<AMD64Register>, Operand<AMD64Register>),
    Movb(Operand<AMD64Register>, Operand<AMD64Register>),
    Movzbl(Operand<AMD64Register>, Operand<AMD64Register>),
    Xor(Operand<AMD64Register>, Operand<AMD64Register>),

    Vmovdqu(Operand<AMD64Register>, Operand<AMD64Register>),
    Vpcmpeqb(
        Operand<AMD64Register>,
        Operand<AMD64Register>,
        Operand<AMD64Register>,
    ),
    Vpmovmskb(Operand<AMD64Register>, Operand<AMD64Register>),
    Vpor(
        Operand<AMD64Register>,
        Operand<AMD64Register>,
        Operand<AMD64Register>,
    ),
    Vpxor(
        Operand<AMD64Register>,
        Operand<AMD64Register>,
        Operand<AMD64Register>,
    ),
}

impl AMD64Instruction {
    fn convert_instruction(instr: &IntermediateInstruction) -> Vec<AMD64Instruction> {
        use AMD64Instruction::*;
        use AMD64Register::*;
        use Function::*;
        use IntermediateInstruction::*;

        let reg = |reg: AMD64Register| Operand::Register(reg);
        let imm = |val: i32| Operand::Immediate(val);
        let deref =
            |op: Operand<AMD64Register>, offset: i32| Operand::Dereference(Box::new(op), offset);

        let mem_pos = || reg(R12);
        let mem_val = || deref(mem_pos(), 0);

        match instr {
            Loop(instrs) => {
                let body = Self::convert_instructions(instrs);
                let body_len = body.len();
                vec![
                    // If the current cell's value is zero,
                    // jump *over* the body *and* the following loop condition
                    vec![Cmpb(imm(0), mem_val()), Je(body_len as isize + 2)],
                    body,
                    // If the current cell's value is zero,
                    // jump back to the beginning of the body
                    vec![Cmpb(imm(0), mem_val()), Jne(-(body_len as isize + 2))],
                ]
                .concat()
            }

            Move(offset) => {
                vec![Addq(imm(*offset), mem_pos())]
            }

            Add(offset) => {
                vec![Addq(imm(*offset), mem_val())]
            }

            Read => {
                vec![Call(GetChar), Movb(reg(AL), mem_val())]
            }

            Write => vec![
                Xor(reg(RDI), reg(RDI)),
                Movb(mem_val(), reg(DIL)),
                Call(PutChar),
            ],

            AddDynamic(target, multiplier) => {
                vec![
                    Movzbl(mem_val(), reg(R13D)),
                    Imul(imm(*multiplier), reg(R13D)),
                    Addb(reg(R13B), deref(mem_pos(), *target)),
                ]
            }

            SimpleLoop(instrs) => {
                let body = Self::convert_instructions(instrs);
                vec![
                    // Jump *over* the simple loop if the current cell's value is zero
                    vec![Cmpb(imm(0), mem_val()), Je(body.len() as isize)],
                    body,
                ]
                .concat()
            }

            Zero => {
                vec![Movb(imm(0), mem_val())]
            }

            Scan(stride) => {
                let mut result = vec![
                    // Beginning of loop.
                    // Instruction 0
                    Vmovdqu(
                        deref(reg(R12), if *stride < 0 { -31 } else { 0 }),
                        reg(YMM3),
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
                    Vpmovmskb(reg(YMM3), reg(EAX)),
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
                    Addq(imm(if *stride < 0 { -32 } else { 32 }), mem_pos()),
                    // Instruction 8
                    // Jump to beginning of loop
                    Jmp(-9),
                ];
                // Instruction 9 (end of loop)
                if *stride < 0 {
                    result.push(Addq(imm(-31), reg(RAX)))
                }
                // Instruction 10
                result.push(Addq(reg(RAX), mem_pos()));
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

            AMD64Instruction::Addb(src, dst) => format!("    addb {}, {}", src, dst),
            AMD64Instruction::Addq(src, dst) => format!("    addq {}, {}", src, dst),
            AMD64Instruction::Bsf(src, dst) => format!("    bsf {}, {}", src, dst),
            AMD64Instruction::Bsr(src, dst) => format!("    bsr {}, {}", src, dst),
            AMD64Instruction::Cmpb(src, dst) => format!("    cmpb {}, {}", src, dst),
            AMD64Instruction::Imul(src, dst) => format!("    imul {}, {}", src, dst),
            AMD64Instruction::Leaq(src, dst) => format!("    leaq {}, {}", src, dst),
            AMD64Instruction::Movb(src, dst) => format!("    movb {}, {}", src, dst),
            AMD64Instruction::Movzbl(src, dst) => format!("    movzbl {}, {}", src, dst),
            AMD64Instruction::Xor(src, dst) => format!("    xor {}, {}", src, dst),

            AMD64Instruction::Vmovdqu(src, dst) => format!("    vmovdqu {}, {}", src, dst),
            AMD64Instruction::Vpmovmskb(src, dst) => format!("    vpmovmskb {}, {}", src, dst),
            AMD64Instruction::Vpcmpeqb(op1, op2, dst) => {
                format!("    vpcmpeqb {}, {}, {}", op1, op2, dst)
            }
            AMD64Instruction::Vpor(op1, op2, dst) => format!("    vpor {}, {}, {}", op1, op2, dst),
            AMD64Instruction::Vpxor(op1, op2, dst) => {
                format!("    vpxor {}, {}, {}", op1, op2, dst)
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

            Addb(src, dst) => {
                //
                todo!()
            }
            Addq(src, dst) => {
                let opcode: u8;
                let mut src_reg: bool = false;
                match src {
                    Operand::Register(reg) => {
                        src_reg = true;
                        rex.b_reg(reg);
                        // TODO: ...
                    }
                    _ => panic!(
                        "Invalid source `{}` for instruction `{}`",
                        src,
                        self.to_string()
                    ),
                }
                match dst {
                    Operand::Register(reg) => {
                        // TODO: ...
                        if src_reg {
                            opcode = 0x3;
                            rex.r_reg(reg);
                        }
                    }
                    _ => panic!(
                        "Invalid destination `{}` for instruction `{}`",
                        dst,
                        self.to_string()
                    ),
                }
                todo!()
            }
            Bsf(src, dst) => {
                //
                todo!()
            }
            Bsr(src, dst) => {
                //
                todo!()
            }
            Cmpb(src, dst) => {
                //
                todo!()
            }
            Imul(src, dst) => {
                //
                todo!()
            }
            Leaq(src, dst) => {
                match src {
                    Operand::Dereference(sub_src, offset) => match **sub_src {
                        Operand::Register(reg) => {
                            rex.b_reg(&reg);
                            mod_rm.rm_reg(&reg);
                            if *offset != 0 {
                                imm = Some(*offset);
                                mod_rm.mode(2);
                            }
                            if (reg.id() & 7) == 0b100 {
                                sib = Some(0b00_100_100);
                            }
                        }
                        _ => panic!(
                            "Invalid dereference `{}` for instruction `{}`",
                            src,
                            self.to_string()
                        ),
                    },
                    _ => panic!(
                        "Invalid destination `{}` for instruction `{}`",
                        dst,
                        self.to_string()
                    ),
                }
                match dst {
                    Operand::Register(reg) => {
                        rex.r_reg(reg);
                        mod_rm.reg_reg(reg);
                    }
                    _ => panic!(
                        "Invalid destination `{}` for instruction `{}`",
                        dst,
                        self.to_string()
                    ),
                }

                let mut result = vec![];
                // REX prefix
                if rex.is_some() {
                    result.push(rex.as_byte());
                }
                // Opcode
                result.push(0x8d);
                // ModR/M field (source & destination)
                result.push(mod_rm.as_byte());
                if let Some(sib) = sib {
                    result.push(sib);
                }
                if let Some(imm) = imm {
                    result.extend(encode_u32(imm as u32));
                }

                result
            }
            Movb(src, dst) => {
                //
                todo!()
            }
            Movzbl(src, dst) => {
                //
                todo!()
            }
            Xor(src, dst) => {
                //
                todo!()
            }

            Vmovdqu(src, dst) => {
                //
                todo!()
            }
            Vpmovmskb(src, dst) => {
                //
                todo!()
            }
            Vpcmpeqb(op1, op2, dst) => {
                //
                todo!()
            }
            Vpor(op1, op2, dst) => {
                //
                todo!()
            }
            Vpxor(op1, op2, dst) => {
                //
                todo!()
            }
        }
    }
}
