pub trait Instruction {
    fn to_string(&self) -> String;

    fn to_binary(&self) -> Vec<u8>;
}
