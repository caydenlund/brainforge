use crate::instruction::Instruction;

pub trait Generator {
    fn new(src: &[Instruction], mem_size: usize) -> Self;

    fn text(&self) -> String;
}
