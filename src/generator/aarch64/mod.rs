//! Assembly generation for the AArch64 architecture

use std::collections::HashMap;

use super::Generator;

mod _bf_prog;
mod _main;
mod _postamble;
mod _preamble;

/// Assembly generator for the AArch64 architecture
pub struct AArch64Generator {
    /// The size of the tape to allocate at runtime
    mem_size: usize,

    /// A list of instructions that make up the `bf_body` function body
    bf_instrs: Vec<String>,

    /// A list of C standard library functions used
    libc_funcs: Vec<String>,
}

impl Generator for AArch64Generator {
    /// Instantiates a new [`AArch64Generator`] [`Generator`]
    ///
    /// This is where most of the generation logic lives; fills out the `bf_body` function body
    fn new(src: &[crate::instruction::Instruction], mem_size: usize) -> Self {
        let libc_funcs = vec!["malloc".into(), "getchar".into(), "putchar".into()];

        let mut bf_instrs = vec![];
        let mut jumps: HashMap<usize, String> = HashMap::new();
        let mut next_jump = 0;
        let mut get_next_jump = |ind, jumps: &mut HashMap<usize, String>| {
            let jump = format!("lj{}", next_jump);
            jumps.insert(ind, jump.clone());
            next_jump += 1;
            jump
        };

        for ind in 0..src.len() {
            let instr = &src[ind];
            bf_instrs.push(
                match instr.instr {
                    crate::instruction::Instr::Left => vec!["    subq $1, %r12".to_string()],
                    crate::instruction::Instr::Right => vec!["    addq $1, %r12".to_string()],
                    crate::instruction::Instr::Decr => vec!["    subq $1, (%r12)".to_string()],
                    crate::instruction::Instr::Incr => vec!["    addq $1, (%r12)".to_string()],
                    crate::instruction::Instr::Read => {
                        vec![
                            "    call getchar".to_string(),
                            "    movb %al, (%r12)".to_string(),
                        ]
                    }
                    crate::instruction::Instr::Write => {
                        vec![
                            "    movq (%r12), %rdi".to_string(),
                            "    call putchar".to_string(),
                        ]
                    }
                    crate::instruction::Instr::LBrace(r_ind) => {
                        let tmp = vec![
                            "    cmpb $0, (%r12)".to_string(),
                            format!("    je {}", get_next_jump(r_ind, &mut jumps)).to_string(),
                            format!("{}:", get_next_jump(ind, &mut jumps)),
                        ];
                        tmp
                    }
                    crate::instruction::Instr::RBrace(l_ind) => {
                        let tmp = vec![
                            "    cmpb $0, (%r12)".to_string(),
                            format!("    jne {}", jumps.get(&l_ind).unwrap()).to_string(),
                            format!("{}:", jumps.get(&ind).unwrap()),
                        ];
                        tmp
                    }
                }
                .join("\n"),
            );
        }

        Self {
            mem_size,
            bf_instrs,
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
