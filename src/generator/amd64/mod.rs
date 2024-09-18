use super::Generator;

mod _bf_prog;
mod _main;
mod _postamble;
mod _preamble;

pub struct AMD64Generator {
    mem_size: usize,
    bf_instrs: Vec<String>,
    libc_funcs: Vec<String>,
}

impl Generator for AMD64Generator {
    fn new(src: &[crate::instruction::Instruction], mem_size: usize) -> Self {
        let mut bf_instrs = vec![];

        let libc_funcs = vec!["malloc".into(), "getchar".into(), "putchar".into()];

        for instr in src {
            bf_instrs.push(
                match instr.instr {
                    crate::instruction::Instr::Left => vec!["    subq $1, %r12"],
                    crate::instruction::Instr::Right => vec!["    addq $1, %r12"],
                    crate::instruction::Instr::Decr => vec!["    subq $1, (%r12)"],
                    crate::instruction::Instr::Incr => vec!["    addq $1, (%r12)"],
                    crate::instruction::Instr::Read => {
                        vec!["    call getchar", "    movb %al, (%r12)"]
                    }
                    crate::instruction::Instr::Write => {
                        vec!["    movq (%r12), %rdi", "    call putchar"]
                    }
                    crate::instruction::Instr::LBrace(_) => todo!(),
                    crate::instruction::Instr::RBrace(_) => todo!(),
                }
                .iter()
                .map(|instr| instr.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            );
        }

        Self {
            mem_size,
            bf_instrs,
            libc_funcs,
        }
    }

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
