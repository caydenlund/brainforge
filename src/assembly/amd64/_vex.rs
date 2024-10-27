use crate::assembly::amd64::{AMD64Register, MemorySize};
use crate::pack_byte;
use MemorySize::*;

/// Encodes the `VEX` prefix for SIMD instructions
pub struct Vex {
    /// Equivalent to rex.r
    pub r: bool,

    /// Equivalent to rex.x
    pub x: bool,

    /// Equivalent to rex.b
    pub b: bool,

    /// Extra byte in the opcode
    pub opcode_extra_byte: Option<u8>,

    /// Instruction-specific bit
    pub w: bool,

    /// Extra register for 3-operand or 4-operand instructions
    pub reg: Option<AMD64Register>,

    /// True if 256-bit mode; false if 128-bit mode
    pub l: bool,

    /// Opcode prefix extension
    ///
    /// 0b01: equivalent to legacy 0x66
    /// 0b10: equivalent to legacy 0xF3
    /// 0b11: equivalent to legacy 0xF2
    pub pp: u8,
}

impl Vex {
    pub fn new() -> Self {
        Self {
            r: false,
            x: false,
            b: false,
            opcode_extra_byte: None,
            w: false,
            reg: None,
            l: true, // Default to 256-bit mode
            pp: 0,
        }
    }

    pub fn as_byte(&self) -> Result<Vec<u8>, ()> {
        // One's complement inverse of `self.reg.id()`
        let reg = match self.reg {
            Some(reg) => {
                if reg.size() != YMMWord.size() {
                    return Err(());
                }
                !(reg.id() as u8)
            }
            None => 0b1111,
        };

        if !self.x && !self.b && !self.w && self.opcode_extra_byte.is_none() {
            // Two-byte VEX prefix
            Ok(vec![
                0xC5,
                pack_byte!(
                    !self.r, // One's complement inverse
                    (reg >> 3) & 1,
                    (reg >> 2) & 1,
                    (reg >> 1) & 1,
                    reg & 1,
                    self.l,
                    (self.pp >> 1) & 1,
                    self.pp & 1
                ),
            ])
        } else {
            // Three-byte VEX prefix
            let opcode_extra_byte_code = match self.opcode_extra_byte {
                None => 0b01_u8,
                Some(0x38) => 0b10_u8,
                Some(0x3A) => 0b11_u8,
                Some(_) => return Err(()),
            };
            Ok(vec![
                0xC4,
                pack_byte!(
                    !self.r, // One's complement inverse
                    !self.x, // One's complement inverse
                    !self.b, // One's complement inverse
                    (opcode_extra_byte_code >> 4) & 1,
                    (opcode_extra_byte_code >> 3) & 1,
                    (opcode_extra_byte_code >> 2) & 1,
                    (opcode_extra_byte_code >> 1) & 1,
                    opcode_extra_byte_code & 1
                ),
                pack_byte!(
                    self.w,
                    (reg >> 3) & 1,
                    (reg >> 2) & 1,
                    (reg >> 1) & 1,
                    reg & 1,
                    self.l,
                    (self.pp >> 1) & 1,
                    self.pp & 1
                ),
            ])
        }
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
}
