//! Assembly generation for the BF instructions for AMD64

use super::AMD64Generator;
use crate::instruction::IntermediateInstruction;

impl AMD64Generator {
    pub(crate) fn generate_instrs(
        src: &[IntermediateInstruction],
        next_jump: &mut Box<usize>,
    ) -> Vec<String> {
        let mut bf_instrs = vec![];
        for ind in 0..src.len() {
            bf_instrs.push(
                match &src[ind] {
                    IntermediateInstruction::Loop(instrs) => {
                        let jump = *next_jump.as_ref();
                        *next_jump.as_mut() += 1;
                        vec![
                            "    cmpb $0, (%r10)".to_string(),
                            format!("    je loop_post_{}", jump),
                            format!("loop_pre_{}:", jump),
                            Self::generate_instrs(instrs, next_jump).join("\n"),
                            "    cmpb $0, (%r10)".to_string(),
                            format!("    jne loop_pre_{}", jump),
                            format!("loop_post_{}:", jump),
                        ]
                    }
                    IntermediateInstruction::Move(offset) => {
                        vec![format!("    addq ${}, %r10", offset)]
                    }
                    IntermediateInstruction::Add(offset) => {
                        vec![format!("    addb ${}, (%r10)", offset)]
                    }
                    IntermediateInstruction::Read => {
                        vec![
                            "    call getchar".to_string(),
                            "    movb %al, (%r10)".to_string(),
                        ]
                    }
                    IntermediateInstruction::Write => {
                        vec![
                            "    xor %rdi, %rdi".to_string(),
                            "    movb (%r10), %dil".to_string(),
                            "    call putchar".to_string(),
                        ]
                    }
                    IntermediateInstruction::SimpleLoop(pairs) => {
                        let mut result = vec![
                            "    movzbl (%r10), %r13d".to_string(),
                            "    movb $0, (%r10)".to_string(),
                        ];
                        for (pair_move, pair_mul) in pairs {
                            if *pair_move != 0 {
                                result.extend(vec![
                                    "    mov %r13d, %r14d".to_string(),
                                    format!("    imul ${}, %r14d", pair_mul),
                                    format!("    movb %r14b, {}(%r10)", pair_move),
                                ]);
                            }
                        }
                        result
                    }
                }
                .join("\n"),
            );
        }
        bf_instrs
    }
}
