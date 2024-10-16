#[derive(Clone, Debug)]
pub enum Operand {
    Register(String),
    Immediate(i32),
    Dereference(Box<Operand>, i32),
    JumpTarget(Box<Operand>),
}
