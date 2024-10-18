use std::collections::HashMap;

pub trait Instruction {
    fn to_string(&self) -> String;

    fn to_binary(&self, index: usize, jump_table: HashMap<String, usize>) -> Vec<u8>;
}
