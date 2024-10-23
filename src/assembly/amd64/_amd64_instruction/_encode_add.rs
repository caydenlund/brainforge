use crate::assembly::amd64::AMD64Operand::*;
use crate::assembly::amd64::{AMD64Instruction, AMD64Operand, ModRM, Rex, Sib};
use crate::assembly::Instruction;

impl AMD64Instruction {
    pub(crate) fn encode_add(
        self: &AMD64Instruction,
        dst: &AMD64Operand,
        src: &AMD64Operand,
    ) -> Vec<u8> {
        match (dst, src) {
            // register += register
            (Register(dst_reg), Register(src_reg)) => {
                if dst_reg.size() != src_reg.size() {
                    panic!("Mismatched register sizes: `{}`", self.to_string());
                }

                let size_prefix_16: Option<u8> = if dst_reg.size() == 16 {
                    Some(0x66)
                } else {
                    None
                };

                let rex = {
                    let mut rex = Rex::new();
                    rex.r_reg(src_reg);
                    rex.b_reg(dst_reg);
                    rex.as_byte()
                };

                let mod_rm = {
                    let mut mod_rm = ModRM::new();
                    mod_rm.mode(3);
                    mod_rm.reg_reg(src_reg);
                    mod_rm.rm_reg(dst_reg);
                    mod_rm.as_byte()
                };

                let opcode: u8 = match dst_reg.size() {
                    8 => 0x00,
                    16 | 32 | 64 => 0x01,
                    _ => panic!("Invalid instruction: `{}`", self.to_string()),
                };

                vec![size_prefix_16, rex, Some(opcode), Some(mod_rm)]
                    .into_iter()
                    .flatten()
                    .collect()
            }
            // register += memory
            (Register(dst_reg), Memory(size, base_reg, index_reg, index_scale, offset)) => {
                if let Some(size) = size {
                    if dst_reg.size() != size.size() {
                        panic!("Operand size mismatch: `{}`", self.to_string())
                    }
                }

                let (rex, mem_size_prefix_32) = {
                    let mut rex = Rex::new();
                    rex.r_reg(dst_reg);
                    let mut index_size = None;
                    let mut mem_size_prefix_32: Option<u8> = None;
                    if let Some(index_reg) = index_reg {
                        index_size = Some(index_reg.size());
                        if index_reg.size() == 32 {
                            mem_size_prefix_32 = Some(0x67);
                        } else if index_reg.size() != 64 {
                            panic!("Invalid index size: `{}`", self.to_string());
                        }
                        if index_reg.id() > 7 {
                            rex.x();
                        }
                    }
                    if let Some(base_reg) = base_reg {
                        if let Some(index_size) = index_size {
                            if index_size != base_reg.size() {
                                panic!("Memory operand size mismatch: `{}`", self.to_string());
                            }
                        }
                        if base_reg.size() == 32 {
                            mem_size_prefix_32 = Some(0x67);
                        } else if base_reg.size() != 64 {
                            panic!("Invalid base size: `{}`", self.to_string());
                        }
                        if base_reg.id() > 7 {
                            rex.b();
                        }
                    }
                    (rex.as_byte(), mem_size_prefix_32)
                };

                let reg_size_prefix_16: Option<u8> = if dst_reg.size() == 16 {
                    Some(0x66)
                } else {
                    None
                };

                let opcode: u8 = if dst_reg.size() == 8 { 0x02 } else { 0x03 };

                let (mod_rm, offset, sib) = {
                    let make_offset = |offset: i32, mod_rm: &mut ModRM| -> Vec<u8> {
                        match offset {
                            -0x80..0x80 => {
                                mod_rm.mode(0b01);
                                vec![offset as u8]
                            }
                            _ => {
                                mod_rm.mode(0b10);
                                Vec::from(offset.to_le_bytes())
                            }
                        }
                    };
                    let make_sib = |mod_rm: &mut ModRM, offset: &mut Option<Vec<u8>>| -> u8 {
                        mod_rm.rm(0b100);
                        let mut sib = Sib::new();
                        match base_reg {
                            Some(base_reg) => {
                                sib.base((base_reg.id() & 7) as u8);
                            }
                            None => {
                                sib.base(0b101);
                                mod_rm.mode(0);
                                if let Some(inner_offset) = offset {
                                    if inner_offset.len() == 1 {
                                        *offset = Some(vec![inner_offset[0], 0, 0, 0]);
                                    }
                                } else {
                                    *offset = Some(vec![0, 0, 0, 0]);
                                }
                            }
                        }
                        match index_reg {
                            Some(index_reg) => {
                                if index_reg.id() == 0b100 {
                                    panic!("Illegal index register: `{}`", self.to_string());
                                }
                                sib.index((index_reg.id() & 7) as u8);
                                match index_scale.unwrap_or(1) {
                                    1 => {
                                        sib.scale(0b00);
                                    }
                                    2 => {
                                        sib.scale(0b01);
                                    }
                                    4 => {
                                        sib.scale(0b10);
                                    }
                                    8 => {
                                        sib.scale(0b11);
                                    }
                                    index_scale => {
                                        panic!("Illegal index scale: `{}`", index_scale)
                                    }
                                }
                            }
                            None => {
                                sib.index(0b100);
                            }
                        }
                        sib.as_byte()
                    };

                    let mut mod_rm = ModRM::new();
                    let mut offset: Option<Vec<u8>> = match offset {
                        None => None,
                        Some(val) => Some(make_offset(*val, &mut mod_rm)),
                    };

                    mod_rm.reg_reg(dst_reg);

                    let sib = {
                        if index_reg.is_some() {
                            Some(make_sib(&mut mod_rm, &mut offset))
                        } else if let Some(base_reg) = base_reg {
                            match (base_reg.id() & 7) as u8 {
                                0b100 => Some(make_sib(&mut mod_rm, &mut offset)),
                                0b101 => {
                                    if offset.is_none() {
                                        offset = Some(make_offset(0, &mut mod_rm));
                                    }
                                    None
                                }
                                id => {
                                    mod_rm.rm(id);
                                    None
                                }
                            }
                        } else {
                            Some(make_sib(&mut mod_rm, &mut offset))
                        }
                    };
                    (mod_rm.as_byte(), offset, sib)
                };

                let mut result: Vec<u8> = vec![
                    mem_size_prefix_32,
                    reg_size_prefix_16,
                    rex,
                    Some(opcode),
                    Some(mod_rm),
                    sib,
                ]
                .into_iter()
                .flatten()
                .collect();
                if let Some(offset) = offset {
                    result.extend(offset);
                }
                for byte in &result {
                    print!("{:02x} ", byte);
                }
                println!();
                result
            }
            // register += immediate
            (Register(dst_reg), Immediate(src_imm)) => {
                let size_prefix_16: Option<u8> = if dst_reg.size() == 16 {
                    Some(0x66)
                } else {
                    None
                };

                let rex = {
                    let mut rex = Rex::new();
                    rex.b_reg(dst_reg);
                    rex.as_byte()
                };

                let mod_rm = if dst_reg.id() == 0 {
                    None
                } else {
                    let mut mod_rm = ModRM::new();
                    mod_rm.mode(3);
                    mod_rm.rm_reg(dst_reg);
                    Some(mod_rm.as_byte())
                };

                let (opcode, imm_bytes): (u8, Vec<u8>) = match (dst_reg.id(), dst_reg.size()) {
                    (0, 8) => (0x04, Self::bytes_8(*src_imm)),
                    (0, 16) => (0x05, Self::bytes_16(*src_imm)),
                    (0, 32) => (0x05, Self::bytes_32(*src_imm)),
                    (0, 64) => (0x05, Self::bytes_32(*src_imm)),
                    (_, 8) => (0x80, Self::bytes_8(*src_imm)),
                    (_, 16) => (0x81, Self::bytes_16(*src_imm)),
                    (_, 32) => (0x81, Self::bytes_32(*src_imm)),
                    (_, 64) => (0x81, Self::bytes_32(*src_imm)),
                    _ => panic!("Invalid instruction: `{}`", self.to_string()),
                };

                let mut result: Vec<u8> = vec![size_prefix_16, rex, Some(opcode), mod_rm]
                    .into_iter()
                    .flatten()
                    .collect();
                result.extend_from_slice(&*imm_bytes);
                result
            }
            // memory += register
            (Memory(size, base_reg, index_reg, index_scale, offset), Register(src_reg)) => {
                //
                todo!()
            }
            // memory += immediate
            (Memory(size, base_reg, index_reg, index_scale, offset), Immediate(src_imm)) => {
                //
                todo!()
            }
            (_, _) => panic!("Invalid instruction: `{}`", self.to_string()),
        }
    }
}
