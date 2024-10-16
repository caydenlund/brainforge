use crate::assembly::{Instruction, Operand};
use crate::instruction::IntermediateInstruction;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum AMD64Instruction {
    Call(Operand),
    Je(Operand),
    Jmp(Operand),
    Jne(Operand),
    Jnz(Operand),
    Label(Operand),

    Addb(Operand, Operand),
    Addq(Operand, Operand),
    Bsf(Operand, Operand),
    Bsr(Operand, Operand),
    Cmpb(Operand, Operand),
    Imul(Operand, Operand),
    Movb(Operand, Operand),
    Movzbl(Operand, Operand),
    Xor(Operand, Operand),

    Vmovdqu(Operand, Operand),
    Vpcmpeqb(Operand, Operand, Operand),
    Vpmovmskb(Operand, Operand),
    Vpor(Operand, Operand, Operand),
    Vpxor(Operand, Operand, Operand),
}

impl AMD64Instruction {
    fn convert_instruction(
        instr: &IntermediateInstruction,
        label_counter: &mut usize,
    ) -> Vec<Vec<AMD64Instruction>> {
        use AMD64Instruction::*;

        let reg = |val: &str| Operand::Register(String::from("%") + val);
        let imm = |val: i32| Operand::Immediate(val);
        let deref = |op: Operand, offset: i32| Operand::Dereference(Box::new(op), offset);
        let jump =
            |target: &str| Operand::JumpTarget(Box::new(Operand::Register(target.to_string())));

        let mem_pos = || reg("r12");
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
                vec![vec![Call(jump("getchar")), Movb(reg("al"), mem_val())]]
            }
            IntermediateInstruction::Write => vec![vec![
                Xor(reg("rdi"), reg("rdi")),
                Movb(mem_val(), reg("dil")),
                Call(jump("putchar")),
            ]],
            IntermediateInstruction::AddDynamic(target, multiplier) => {
                vec![vec![
                    Movzbl(mem_val(), reg("r13d")),
                    Imul(imm(*multiplier), reg("r13d")),
                    Addb(reg("r13b"), deref(mem_pos(), *target)),
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
                        deref(reg("r12"), if *stride < 0 { -31 } else { 0 }),
                        reg("ymm3"),
                    ),
                    Vpxor(reg("ymm0"), reg("ymm0"), reg("ymm0")),
                    Vpor(
                        reg("ymm3"),
                        reg(&*format!("ymm{}", stride.abs())),
                        reg("ymm3"),
                    ),
                    Vpcmpeqb(reg("ymm3"), reg("ymm0"), reg("ymm3")),
                    Vpmovmskb(reg("ymm3"), reg("eax")),
                    if *stride < 0 {
                        Bsr(reg("eax"), reg("eax"))
                    } else {
                        Bsf(reg("eax"), reg("eax"))
                    },
                    Jnz(jump(&*format!(".scan_finish_{}", label_num))),
                    Addq(imm(if *stride < 0 { -32 } else { 32 }), mem_pos()),
                    Jmp(jump(&*format!(".scan_start_{}", label_num))),
                    Label(jump(&*format!(".scan_finish_{}", label_num))),
                ];
                if *stride < 0 {
                    result.push(Addq(imm(-31), reg("rax")))
                }
                result.push(Addq(reg("rax"), mem_pos()));
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

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operand::Register(reg) => format!("{}", reg),
                Operand::Immediate(imm) => format!("${}", imm),
                Operand::Dereference(op, offset) => format!(
                    "{}({})",
                    if *offset != 0 {
                        offset.to_string()
                    } else {
                        String::from("")
                    },
                    op
                ),
                Operand::JumpTarget(tgt) => format!("{}", tgt),
            }
        )
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

    fn to_binary(&self) -> Vec<u8> {
        todo!()
    }
}
