use crate::assembly::amd64::AMD64Instruction;
use crate::BFResult;

impl AMD64Instruction {
    pub(crate) fn encode_jne(self: &AMD64Instruction, tgt: isize) -> BFResult<Vec<u8>> {
        Ok(vec![0x0F, 0x85]
            .into_iter()
            .chain(self.encode_imm(tgt, 32)?)
            .collect())
    }
}

#[cfg(test)]
pub mod tests {
    use crate::assembly::amd64::AMD64Instruction;
    use crate::assembly::Instruction;

    use AMD64Instruction::*;

    type Tests = Vec<(AMD64Instruction, Vec<u8>)>;

    fn run_tests(tests: Tests) {
        for (instruction, expected) in tests {
            match instruction.to_binary() {
                Ok(actual) => assert_eq!(actual, expected, "{}", instruction.to_string()),
                Err(err) => panic!("{:?}", err),
            }
        }
    }

    #[test]
    fn test_encode_jne() {
        let tests: Tests = vec![(Jne(0x33221100), vec![0x0F, 0x85, 0x00, 0x11, 0x22, 0x33])];
        run_tests(tests);
    }
}
