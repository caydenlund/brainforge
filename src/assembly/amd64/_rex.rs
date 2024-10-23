use crate::assembly::amd64::AMD64Register;
use crate::pack_byte;

#[derive(Copy, Clone, Debug)]
pub struct Rex {
    w: bool,
    r: bool,
    x: bool,
    b: bool,
}

impl Rex {
    pub fn new() -> Self {
        Self {
            w: false,
            r: false,
            x: false,
            b: false,
        }
    }

    pub fn as_byte(&self) -> Option<u8> {
        if self.is_some() {
            Some(pack_byte!(0, 1, 0, 0, self.w, self.r, self.x, self.b))
        } else {
            None
        }
    }

    pub fn r_reg(&mut self, reg: &AMD64Register) {
        if reg.size() > 32 {
            self.w();
        }
        if reg.id() > 7 {
            self.r();
        }
    }

    pub fn x_reg(&mut self, reg: &AMD64Register) {
        if reg.id() > 7 {
            self.x();
        }
    }

    pub fn b_reg(&mut self, reg: &AMD64Register) {
        if reg.id() > 7 {
            self.b();
        }
    }

    pub fn w(&mut self) {
        self.w = true;
    }

    pub fn r(&mut self) {
        self.r = true;
    }

    pub fn x(&mut self) {
        self.x = true;
    }

    pub fn b(&mut self) {
        self.b = true;
    }

    pub fn is_some(&self) -> bool {
        self.w || self.r || self.x || self.b
    }
}
