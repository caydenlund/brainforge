use crate::instruction::*;

fn generate_instr(instr: &Instr) -> String {
    match instr {
        Instr::Left => vec!["subq $1, %rcx"],
        Instr::Right => vec!["addq $1, %rcx"],
        Instr::Decr => vec!["subq $1, (%rcx)"],
        Instr::Incr => vec!["addq $1, (%rcx)"],
        Instr::Read => todo!(),
        Instr::Write => vec!["movq (%rcx), %rdi", "call putchar"],
        Instr::LBrace(_) => todo!(),
        Instr::RBrace(_) => todo!(),
    }
    .iter()
    .map(|inst| String::from("    ") + inst)
    .collect::<Vec<String>>()
    .join("\n")
}

pub fn generate_bf_prog(src: &[Instruction]) -> String {
    format!(
        r#"
;# <<<Begin BF Program>>>
bf_prog:
    movq %rdi, %rbx
    movq %rdi, %rcx
{}
    ret
;# <<<End BF Program>>>
"#,
        src.iter()
            .map(|instr| generate_instr(&instr.instr))
            .collect::<Vec<String>>()
            .join("\n")
    )
    .trim()
    .to_string()
}
