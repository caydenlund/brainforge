use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum JumpTarget<R> {
    Register(R),
    Label(String),
}

impl<R: Display> Display for JumpTarget<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                JumpTarget::Register(reg) => reg.to_string(),
                JumpTarget::Label(label) => label.clone(),
            }
        )
    }
}

#[derive(Clone, Debug)]
pub enum Operand<R> {
    Register(R),
    Immediate(i32),
    Dereference(Box<Operand<R>>, i32),
    JumpTarget(JumpTarget<R>),
}

impl<R: Display> Display for Operand<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operand::Register(reg) => reg.to_string(),
                Operand::Immediate(imm) => format!("${}", imm),
                Operand::Dereference(op, offset) =>
                    if *offset == 0 {
                        format!("({})", op)
                    } else {
                        format!("{}({})", offset, op)
                    },
                Operand::JumpTarget(tgt) => tgt.to_string(),
            }
        )
    }
}
