use crate::assembly::amd64::{AMD64Instruction, Function};
use crate::BFResult;

impl AMD64Instruction {
    pub(crate) fn encode_call(self: &AMD64Instruction, tgt: &Function) -> BFResult<Vec<u8>> {
        let tgt_name = match tgt {
            Function::GetChar => std::ffi::CString::new("getchar").unwrap(),
            Function::PutChar => std::ffi::CString::new("putchar").unwrap(),
        };

        let tgt_addr = unsafe { libc::dlsym(libc::RTLD_DEFAULT, tgt_name.as_ptr()) as usize };
        let imm = self.encode_imm(tgt_addr as isize, 64)?;

        Ok(vec![0x48, 0xB8]
            .into_iter()
            .chain(imm)
            .chain(vec![0xFF, 0xD0])
            .collect())
    }
}
