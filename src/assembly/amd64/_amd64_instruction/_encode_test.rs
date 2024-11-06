use crate::assembly::amd64::{AMD64Instruction, AMD64Operand};
use crate::BFResult;

use AMD64Operand::*;

impl AMD64Instruction {
    pub(crate) fn encode_test(
        self: &AMD64Instruction,
        op1: &AMD64Operand,
        op2: &AMD64Operand,
    ) -> BFResult<Vec<u8>> {
        todo!()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, AMD64Register, MemorySize};

    use AMD64Instruction::*;
    use AMD64Operand::*;
    use AMD64Register::*;
    use MemorySize::*;

    type Tests = Vec<(AMD64Instruction, Vec<u8>)>;

    fn run_tests(tests: Tests) {
        for (instruction, expected) in tests {
            match instruction.to_binary() {
                Ok(actual) => assert_eq!(actual, expected, "{}", instruction.to_string()),
                Err(err) => panic!("{:?}", err),
            }
        }
    }

    // #[test]
    // fn test_encode_and_reg_reg() {
    //     let tests: Tests = vec![
    //     ];
    //     run_tests(tests);
    // }
}
