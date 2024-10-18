use super::AMD64Register;
use crate::assembly::{Instruction, JumpTarget, Operand};
use crate::instruction::IntermediateInstruction;

use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum AMD64Instruction {
    Call(Operand<AMD64Register>),
    Je(Operand<AMD64Register>),
    Jmp(Operand<AMD64Register>),
    Jne(Operand<AMD64Register>),
    Jnz(Operand<AMD64Register>),
    Label(Operand<AMD64Register>),

    Addb(Operand<AMD64Register>, Operand<AMD64Register>),
    Addq(Operand<AMD64Register>, Operand<AMD64Register>),
    Bsf(Operand<AMD64Register>, Operand<AMD64Register>),
    Bsr(Operand<AMD64Register>, Operand<AMD64Register>),
    Cmpb(Operand<AMD64Register>, Operand<AMD64Register>),
    Imul(Operand<AMD64Register>, Operand<AMD64Register>),
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
    fn convert_instruction(
        instr: &IntermediateInstruction,
        label_counter: &mut usize,
    ) -> Vec<Vec<AMD64Instruction>> {
        use AMD64Instruction::*;
        use AMD64Register::*;

        let reg = |reg: AMD64Register| Operand::Register(reg);
        let imm = |val: i32| Operand::Immediate(val);
        let deref =
            |op: Operand<AMD64Register>, offset: i32| Operand::Dereference(Box::new(op), offset);
        let jump = |target: &str| Operand::JumpTarget(JumpTarget::Label(target.to_string()));

        let mem_pos = || reg(R12);
        let mem_val = || deref(mem_pos(), 0);

        match instr {
            IntermediateInstruction::Loop(instrs) => {
                let label_num = *label_counter;
                *label_counter += 1;
                vec![
                    vec![
                        Cmpb(imm(0), mem_val()),
                        Je(jump(&*format!(".loop_post_{}", label_num))),
                        Label(jump(&*format!(".loop_pre_{}", label_num))),
                    ],
                    Self::convert_instructions(instrs, label_counter).concat(),
                    vec![
                        Cmpb(imm(0), mem_val()),
                        Jne(jump(&*format!(".loop_pre_{}", label_num))),
                        Label(jump(&*format!(".loop_post_{}", label_num))),
                    ],
                ]
            }
            IntermediateInstruction::Move(offset) => {
                vec![vec![Addq(imm(*offset), mem_pos())]]
            }
            IntermediateInstruction::Add(offset) => {
                vec![vec![Addq(imm(*offset), mem_val())]]
            }
            IntermediateInstruction::Read => {
                vec![vec![Call(jump("getchar")), Movb(reg(AL), mem_val())]]
            }
            IntermediateInstruction::Write => vec![vec![
                Xor(reg(RDI), reg(RDI)),
                Movb(mem_val(), reg(DIL)),
                Call(jump("putchar")),
            ]],
            IntermediateInstruction::AddDynamic(target, multiplier) => {
                vec![vec![
                    Movzbl(mem_val(), reg(R13D)),
                    Imul(imm(*multiplier), reg(R13D)),
                    Addb(reg(R13B), deref(mem_pos(), *target)),
                ]]
            }
            IntermediateInstruction::SimpleLoop(instrs) => {
                let label_num = *label_counter;
                *label_counter += 1;
                vec![
                    vec![
                        Cmpb(imm(0), mem_val()),
                        Je(jump(&*format!(".simple_loop_post_{}", label_num))),
                    ],
                    Self::convert_instructions(instrs, label_counter).concat(),
                    vec![Label(jump(&*format!(".simple_loop_post_{}", label_num)))],
                ]
            }
            IntermediateInstruction::Zero => {
                vec![vec![Movb(imm(0), mem_val())]]
            }
            IntermediateInstruction::Scan(stride) => {
                let label_num = *label_counter;
                *label_counter += 1;
                let mut result = vec![
                    Label(jump(&*format!(".scan_start_{}", label_num))),
                    Vmovdqu(
                        deref(reg(R12), if *stride < 0 { -31 } else { 0 }),
                        reg(YMM3),
                    ),
                    Vpxor(reg(YMM0), reg(YMM0), reg(YMM0)),
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
                    Vpcmpeqb(reg(YMM3), reg(YMM0), reg(YMM3)),
                    Vpmovmskb(reg(YMM3), reg(EAX)),
                    if *stride < 0 {
                        Bsr(reg(EAX), reg(EAX))
                    } else {
                        Bsf(reg(EAX), reg(EAX))
                    },
                    Jnz(jump(&*format!(".scan_finish_{}", label_num))),
                    Addq(imm(if *stride < 0 { -32 } else { 32 }), mem_pos()),
                    Jmp(jump(&*format!(".scan_start_{}", label_num))),
                    Label(jump(&*format!(".scan_finish_{}", label_num))),
                ];
                if *stride < 0 {
                    result.push(Addq(imm(-31), reg(RAX)))
                }
                result.push(Addq(reg(RAX), mem_pos()));
                vec![result]
            }
        }
    }
    pub fn convert_instructions(
        instrs: &[IntermediateInstruction],
        label_counter: &mut usize,
    ) -> Vec<Vec<AMD64Instruction>> {
        instrs
            .iter()
            .map(|instr| Self::convert_instruction(instr, label_counter))
            .collect::<Vec<Vec<Vec<AMD64Instruction>>>>()
            .concat()
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
            AMD64Instruction::Label(name) => format!("{}:", name),

            AMD64Instruction::Addb(src, dst) => format!("    addb {}, {}", src, dst),
            AMD64Instruction::Addq(src, dst) => format!("    addq {}, {}", src, dst),
            AMD64Instruction::Bsf(src, dst) => format!("    bsf {}, {}", src, dst),
            AMD64Instruction::Bsr(src, dst) => format!("    bsr {}, {}", src, dst),
            AMD64Instruction::Cmpb(src, dst) => format!("    cmpb {}, {}", src, dst),
            AMD64Instruction::Imul(src, dst) => format!("    imul {}, {}", src, dst),
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

    fn to_binary(&self, index: usize, jump_table: HashMap<String, usize>) -> Vec<u8> {
        match self {
            AMD64Instruction::Call(tgt) => {
                //
                todo!()
            }
            AMD64Instruction::Je(tgt) => {
                //
                todo!()
            }
            AMD64Instruction::Jmp(tgt) => {
                //
                todo!()
            }
            AMD64Instruction::Jne(tgt) => {
                //
                todo!()
            }
            AMD64Instruction::Jnz(tgt) => {
                //
                todo!()
            }
            AMD64Instruction::Label(name) => {
                //
                todo!()
            }

            AMD64Instruction::Addb(src, dst) => {
                //
                todo!()
            }
            AMD64Instruction::Addq(src, dst) => {
                //
                todo!()
            }
            AMD64Instruction::Bsf(src, dst) => {
                //
                todo!()
            }
            AMD64Instruction::Bsr(src, dst) => {
                //
                todo!()
            }
            AMD64Instruction::Cmpb(src, dst) => {
                //
                todo!()
            }
            AMD64Instruction::Imul(src, dst) => {
                //
                todo!()
            }
            AMD64Instruction::Movb(src, dst) => {
                //
                todo!()
            }
            AMD64Instruction::Movzbl(src, dst) => {
                //
                todo!()
            }
            AMD64Instruction::Xor(src, dst) => {
                //
                todo!()
            }

            AMD64Instruction::Vmovdqu(src, dst) => {
                //
                todo!()
            }
            AMD64Instruction::Vpmovmskb(src, dst) => {
                //
                todo!()
            }
            AMD64Instruction::Vpcmpeqb(op1, op2, dst) => {
                //
                todo!()
            }
            AMD64Instruction::Vpor(op1, op2, dst) => {
                //
                todo!()
            }
            AMD64Instruction::Vpxor(op1, op2, dst) => {
                //
                todo!()
            }
        }
    }
}
