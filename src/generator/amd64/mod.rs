//! Assembly generation for the AMD64 architecture

use super::Generator;
use crate::instruction::IntermediateInstruction;

mod _bf_prog;
mod _main;
mod _postamble;
mod _preamble;

/// Assembly generator for the AMD64 architecture
pub struct AMD64Generator {
    /// The size of the tape to allocate at runtime
    mem_size: usize,

    /// A list of instructions that make up the `bf_body` function body
    bf_instrs: Vec<String>,

    /// A list of C standard library functions used
    libc_funcs: Vec<String>,
}

impl Generator for AMD64Generator {
    /// Instantiates a new [`AMD64Generator`] [`Generator`]
    ///
    /// This is where most of the generation logic lives; fills out the `bf_body` function body
    fn new(src: &[IntermediateInstruction], mem_size: usize) -> Self {
        let libc_funcs = vec!["malloc".into(), "getchar".into(), "putchar".into()];

        fn generate_instrs(
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
                                "    cmpb $0, (%r12)".to_string(),
                                format!("    je loop_post_{}", jump),
                                format!("loop_pre_{}:", jump),
                                generate_instrs(instrs, next_jump).join("\n"),
                                "    cmpb $0, (%r12)".to_string(),
                                format!("    jne loop_pre_{}", jump),
                                format!("loop_post_{}:", jump),
                            ]
                        }
                        IntermediateInstruction::Move(offset) => {
                            vec![format!("    addq ${}, %r12", offset)]
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
                        IntermediateInstruction::SimpleLoop(pairs) => {
                            let mut result = vec![
                                "    movzbl (%r12), %r13d".to_string(),
                                "    movb $0, (%r12)".to_string(),
                            ];
                            for (pair_move, pair_mul) in pairs {
                                result.extend(vec![
                                    "    mov %r13d, %r14d".to_string(),
                                    format!("    imul ${}, %r14d", pair_mul),
                                    format!("    movb %r14b, {}(%r12)", pair_move),
                                ]);
                            }
                            result
                        }
                    }
                        .join("\n"),
                );
            }
            bf_instrs
        }

        Self {
            mem_size,
            bf_instrs: generate_instrs(src, &mut Box::new(0)),
            libc_funcs,
        }
    }

    /// Generates a comprehensive assembly program as a single string
    fn text(&self) -> String {
        vec![
            self.preamble(),
            self.main(),
            self.bf_prog(),
            self.postamble(),
        ]
            .join("\n\n")
            + "\n"
    }
}
