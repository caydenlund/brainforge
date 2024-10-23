use crate::assembly::amd64::AMD64Register;
use crate::pack_byte;

pub struct Sib {
    pub scale: u8,
    pub index: u8,
    pub base: u8,
}

impl Sib {
    pub fn new() -> Self {
        Self {
            scale: 0,
            index: 0,
            base: 0,
        }
    }

    pub fn index_reg(&mut self, index: &AMD64Register) {
        self.index((index.id() & 7) as u8);
    }

    pub fn base_reg(&mut self, base: &AMD64Register) {
        self.base((base.id() & 7) as u8);
    }

    pub fn scale(&mut self, scale: u8) {
        self.scale = scale;
    }

    pub fn index(&mut self, index: u8) {
        self.index = index;
    }

    pub fn base(&mut self, base: u8) {
        self.base = base;
    }

    pub fn as_byte(&self) -> u8 {
        pack_byte!(
            (self.scale & 2) > 0,
            (self.scale & 1) > 0,
            (self.index & 4) > 0,
            (self.index & 2) > 0,
            (self.index & 1) > 0,
            (self.base & 4) > 0,
            (self.base & 2) > 0,
            (self.base & 1) > 0
        )
    }
}
