//! Assembly generation for the BF instructions for AMD64

use super::AMD64Generator;
use crate::instruction::IntermediateInstruction;

impl AMD64Generator {
    pub(crate) fn generate_instrs(
        src: &[IntermediateInstruction],
        next_jump: &mut Box<usize>,
        mem_size: usize,
    ) -> Vec<String> {
        let mut bf_instrs = vec![];
        for ind in 0..src.len() {
            bf_instrs.push(
                match &src[ind] {
                    IntermediateInstruction::Loop(instrs) => {
                        let jump = *next_jump.as_ref();
                        *next_jump.as_mut() += 1;
                        vec![
                            "    cmpb $0, (%r12)".to_string(),
                            format!("    je loop_post_{}", jump),
                            format!("loop_pre_{}:", jump),
                            Self::generate_instrs(instrs, next_jump, mem_size).join("\n"),
                            "    cmpb $0, (%r12)".to_string(),
                            format!("    jne loop_pre_{}", jump),
                            format!("loop_post_{}:", jump),
                        ]
                    }
                    IntermediateInstruction::Move(offset) => {
                        vec![
                            format!("    addq ${}, %r12", offset),
                            format!("    addq ${}, %r12", mem_size),
                            "    subq %r13, %r12".to_string(),
                            format!("    andq ${}, %r12", mem_size - 1),
                            "    addq %r13, %r12".to_string(),
                        ]
                    }
                    IntermediateInstruction::Add(offset) => {
                        vec![format!("    addb ${}, (%r12)", offset)]
                    }
                    IntermediateInstruction::Read => {
                        vec![
                            "    call getchar".to_string(),
                            "    movb %al, (%r12)".to_string(),
                        ]
                    }
                    IntermediateInstruction::Write => {
                        vec![
                            "    xor %rdi, %rdi".to_string(),
                            "    movb (%r12), %dil".to_string(),
                            "    call putchar".to_string(),
                        ]
                    }
                    IntermediateInstruction::AddDynamic(target, multiplier) => {
                        vec![
                            "    movzbl (%r12), %r14d".to_string(),
                            format!("    imul ${}, %r14d", multiplier),
                            format!("    addb %r14b, {}(%r12)", target),
                        ]
                    }
                    IntermediateInstruction::SimpleLoop(instrs) => {
                        let jump = *next_jump.as_ref();
                        *next_jump.as_mut() += 1;
                        vec![
                            "    cmpb $0, (%r12)".to_string(),
                            format!("    je simple_loop_post_{}", jump),
                            Self::generate_instrs(instrs, next_jump, mem_size).join("\n"),
                            format!("simple_loop_post_{}:", jump),
                        ]
                    }
                    IntermediateInstruction::Zero => {
                        vec!["    movb $0, (%r12)".to_string()]
                    }
                }
                .join("\n"),
            );
        }
        bf_instrs
    }
}
