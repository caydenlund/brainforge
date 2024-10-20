use crate::assembly::amd64::AMD64Register;
use crate::pack_byte;

#[derive(Copy, Clone, Debug)]
pub struct ModRM {
    mode: u8,
    reg: u8,
    rm: u8,
}

impl ModRM {
    pub fn new() -> Self {
        Self {
            mode: 0,
            reg: 0,
            rm: 0,
        }
    }

    pub fn as_byte(&self) -> u8 {
        pack_byte!(
            self.mode & 2 > 0,
            self.mode & 1 > 0,
            self.reg & 4 > 0,
            self.reg & 2 > 0,
            self.reg & 1 > 0,
            self.rm & 4 > 0,
            self.rm & 2 > 0,
            self.rm & 1 > 0
        )
    }

    pub fn reg_reg(&mut self, reg: &AMD64Register) {
        self.reg((reg.id() & 7) as u8);
    }

    pub fn rm_reg(&mut self, reg: &AMD64Register) {
        self.rm((reg.id() & 7) as u8);
    }

    pub fn mode(&mut self, mode: u8) {
        self.mode = mode;
    }

    pub fn reg(&mut self, reg: u8) {
        self.reg = reg;
    }

    pub fn rm(&mut self, rm: u8) {
        self.rm = rm;
    }
}
