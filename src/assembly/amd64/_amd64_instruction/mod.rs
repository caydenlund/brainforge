mod _amd64_instruction;
pub use _amd64_instruction::*;

mod _encode_call;
mod _encode_je;
mod _encode_jmp;
mod _encode_jne;

mod _encode_add;
mod _encode_bsf;
mod _encode_bsr;
mod _encode_cmovge;
mod _encode_cmp;
mod _encode_imul;
mod _encode_lea;
mod _encode_mov;
mod _encode_movzx;
mod _encode_xor;

mod _encode_vmovdqu;
mod _encode_vpmovmskb;
mod _encode_vpcmpeqb;
mod _encode_vpor;
mod _encode_vpxor;
