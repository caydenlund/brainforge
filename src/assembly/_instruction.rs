use crate::BFResult;
use std::fmt::Debug;

pub trait Instruction: Debug {
    fn to_string(&self) -> String;

    fn to_binary(&self) -> BFResult<Vec<u8>>;
}
