use crate::assembly::amd64::AMD64Operand::*;
use crate::assembly::amd64::{AMD64Instruction, Function};

impl AMD64Instruction {
    pub(crate) fn encode_call(self: &AMD64Instruction, tgt: &Function) -> Vec<u8> {
        let tgt_name = match tgt {
            Function::GetChar => std::ffi::CString::new("getchar").unwrap(),
            Function::PutChar => std::ffi::CString::new("putchar").unwrap(),
        };
        let tgt_addr = unsafe { libc::dlsym(libc::RTLD_DEFAULT, tgt_name.as_ptr()) as usize };

        vec![0x48, 0xB8]
            .into_iter()
            .chain(self.unwrap(Self::encode_imm(tgt_addr as isize, 64)))
            .chain(vec![0xFF, 0xD0])
            .collect()
    }
}
