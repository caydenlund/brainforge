use crate::assembly::amd64::AMD64Register;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum MemorySize {
    Byte,
    Word,
    DWord,
    QWord,
    YMMWord,
}

impl MemorySize {
    pub fn size(&self) -> usize {
        match self {
            MemorySize::Byte => 8,
            MemorySize::Word => 16,
            MemorySize::DWord => 32,
            MemorySize::QWord => 64,
            MemorySize::YMMWord => 256,
        }
    }
}

impl Display for MemorySize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} PTR",
            match self {
                MemorySize::Byte => "BYTE",
                MemorySize::Word => "WORD",
                MemorySize::DWord => "DWORD",
                MemorySize::QWord => "QWORD",
                MemorySize::YMMWord => "YMMWORD",
            }
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub enum AMD64Operand {
    Register(AMD64Register),
    Immediate(isize),
    Memory(
        Option<MemorySize>,
        Option<AMD64Register>,
        Option<AMD64Register>,
        Option<u8>,
        Option<i32>,
    ),
}

impl Display for AMD64Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AMD64Operand::Register(reg) => write!(f, "{}", reg),
            AMD64Operand::Immediate(val) => write!(f, "{}", val),
            AMD64Operand::Memory(mem_size, base_reg, index_reg, index_scale, offset) => {
                let mut result = String::from("");
                let mut has_had_term = false;
                if let Some(size) = mem_size {
                    result += &*(size.to_string() + " ");
                };
                result += "[";
                if let Some(reg) = base_reg {
                    has_had_term = true;
                    result += &*reg.to_string();
                };
                if let Some(reg) = index_reg {
                    if has_had_term {
                        result += " + ";
                    }
                    has_had_term = true;
                    result += &*reg.to_string();
                };
                if let Some(scale) = index_scale {
                    if index_reg.is_none() {
                        panic!("Scale used without a register");
                    }
                    if !vec![1, 2, 4, 8].contains(scale) {
                        panic!("Invalid scale used: {}", scale);
                    }
                    result += " * ";
                    result += &*scale.to_string();
                };
                if let Some(offset) = offset {
                    if has_had_term && *offset >= 0 {
                        result += " + ";
                    }
                    result += &*offset.to_string();
                }
                result += "]";
                write!(f, "{}", result)
            }
        }
    }
}
