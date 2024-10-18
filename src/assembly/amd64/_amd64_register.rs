use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum AMD64Register {
    AL,
    AX,
    EAX,
    RAX,

    CL,
    CX,
    ECX,
    RCX,

    DL,
    DX,
    EDX,
    RDX,

    BL,
    BX,
    EBX,
    RBX,

    SPL,
    SP,
    ESP,
    RSP,

    BPL,
    BP,
    EBP,
    RBP,

    SIL,
    SI,
    ESI,
    RSI,

    DIL,
    DI,
    EDI,
    RDI,

    R8B,
    R8L,
    R8W,
    R8D,
    R8,

    R9B,
    R9L,
    R9W,
    R9D,
    R9,

    R10B,
    R10L,
    R10W,
    R10D,
    R10,

    R11B,
    R11L,
    R11W,
    R11D,
    R11,

    R12B,
    R12L,
    R12W,
    R12D,
    R12,

    R13B,
    R13L,
    R13W,
    R13D,
    R13,

    R14B,
    R14L,
    R14W,
    R14D,
    R14,

    R15B,
    R15L,
    R15W,
    R15D,
    R15,

    MMX0,
    XMM0,
    YMM0,

    MMX1,
    XMM1,
    YMM1,

    MMX2,
    XMM2,
    YMM2,

    MMX3,
    XMM3,
    YMM3,

    MMX4,
    XMM4,
    YMM4,

    MMX5,
    XMM5,
    YMM5,

    MMX6,
    XMM6,
    YMM6,

    MMX7,
    XMM7,
    YMM7,

    MMX8,
    XMM8,
    YMM8,

    MMX9,
    XMM9,
    YMM9,

    MMX10,
    XMM10,
    YMM10,

    MMX11,
    XMM11,
    YMM11,

    MMX12,
    XMM12,
    YMM12,

    MMX13,
    XMM13,
    YMM13,

    MMX14,
    XMM14,
    YMM14,

    MMX15,
    XMM15,
    YMM15,
}

impl AMD64Register {
    pub fn id(&self) -> usize {
        use AMD64Register::*;

        match self {
            AL => 0,
            AX => 0,
            EAX => 0,
            RAX => 0,

            CL => 1,
            CX => 1,
            ECX => 1,
            RCX => 1,

            DL => 2,
            DX => 2,
            EDX => 2,
            RDX => 2,

            BL => 3,
            BX => 3,
            EBX => 3,
            RBX => 3,

            SPL => 4,
            SP => 4,
            ESP => 4,
            RSP => 4,

            BPL => 5,
            BP => 5,
            EBP => 5,
            RBP => 5,

            SIL => 6,
            SI => 6,
            ESI => 6,
            RSI => 6,

            DIL => 7,
            DI => 7,
            EDI => 7,
            RDI => 7,

            R8B => 8,
            R8L => 8,
            R8W => 8,
            R8D => 8,
            R8 => 8,

            R9B => 9,
            R9L => 9,
            R9W => 9,
            R9D => 9,
            R9 => 9,

            R10B => 10,
            R10L => 10,
            R10W => 10,
            R10D => 10,
            R10 => 10,

            R11B => 11,
            R11L => 11,
            R11W => 11,
            R11D => 11,
            R11 => 11,

            R12B => 12,
            R12L => 12,
            R12W => 12,
            R12D => 12,
            R12 => 12,

            R13B => 13,
            R13L => 13,
            R13W => 13,
            R13D => 13,
            R13 => 13,

            R14B => 14,
            R14L => 14,
            R14W => 14,
            R14D => 14,
            R14 => 14,

            R15B => 15,
            R15L => 15,
            R15W => 15,
            R15D => 15,
            R15 => 15,

            MMX0 => 0,
            XMM0 => 0,
            YMM0 => 0,

            MMX1 => 1,
            XMM1 => 1,
            YMM1 => 1,

            MMX2 => 2,
            XMM2 => 2,
            YMM2 => 2,

            MMX3 => 3,
            XMM3 => 3,
            YMM3 => 3,

            MMX4 => 4,
            XMM4 => 4,
            YMM4 => 4,

            MMX5 => 5,
            XMM5 => 5,
            YMM5 => 5,

            MMX6 => 6,
            XMM6 => 6,
            YMM6 => 6,

            MMX7 => 7,
            XMM7 => 7,
            YMM7 => 7,

            MMX8 => 8,
            XMM8 => 8,
            YMM8 => 8,

            MMX9 => 9,
            XMM9 => 9,
            YMM9 => 9,

            MMX10 => 10,
            XMM10 => 10,
            YMM10 => 10,

            MMX11 => 11,
            XMM11 => 11,
            YMM11 => 11,

            MMX12 => 12,
            XMM12 => 12,
            YMM12 => 12,

            MMX13 => 13,
            XMM13 => 13,
            YMM13 => 13,

            MMX14 => 14,
            XMM14 => 14,
            YMM14 => 14,

            MMX15 => 15,
            XMM15 => 15,
            YMM15 => 15,
        }
    }

    pub fn size(&self) -> usize {
        use AMD64Register::*;

        match self {
            AL => 8,
            AX => 16,
            EAX => 32,
            RAX => 64,

            CL => 8,
            CX => 16,
            ECX => 32,
            RCX => 64,

            DL => 8,
            DX => 16,
            EDX => 32,
            RDX => 64,

            BL => 8,
            BX => 16,
            EBX => 32,
            RBX => 64,

            SPL => 8,
            SP => 16,
            ESP => 32,
            RSP => 64,

            BPL => 8,
            BP => 16,
            EBP => 32,
            RBP => 64,

            SIL => 8,
            SI => 16,
            ESI => 32,
            RSI => 64,

            DIL => 8,
            DI => 16,
            EDI => 32,
            RDI => 64,

            R8B => 8,
            R8L => 8,
            R8W => 16,
            R8D => 32,
            R8 => 64,

            R9B => 8,
            R9L => 8,
            R9W => 16,
            R9D => 32,
            R9 => 64,

            R10B => 8,
            R10L => 8,
            R10W => 16,
            R10D => 32,
            R10 => 64,

            R11B => 8,
            R11L => 8,
            R11W => 16,
            R11D => 32,
            R11 => 64,

            R12B => 8,
            R12L => 8,
            R12W => 16,
            R12D => 32,
            R12 => 64,

            R13B => 8,
            R13L => 8,
            R13W => 16,
            R13D => 32,
            R13 => 64,

            R14B => 8,
            R14L => 8,
            R14W => 16,
            R14D => 32,
            R14 => 64,

            R15B => 8,
            R15L => 8,
            R15W => 16,
            R15D => 32,
            R15 => 64,

            MMX0 => 64,
            XMM0 => 128,
            YMM0 => 256,

            MMX1 => 64,
            XMM1 => 128,
            YMM1 => 256,

            MMX2 => 64,
            XMM2 => 128,
            YMM2 => 256,

            MMX3 => 64,
            XMM3 => 128,
            YMM3 => 256,

            MMX4 => 64,
            XMM4 => 128,
            YMM4 => 256,

            MMX5 => 64,
            XMM5 => 128,
            YMM5 => 256,

            MMX6 => 64,
            XMM6 => 128,
            YMM6 => 256,

            MMX7 => 64,
            XMM7 => 128,
            YMM7 => 256,

            MMX8 => 64,
            XMM8 => 128,
            YMM8 => 256,

            MMX9 => 64,
            XMM9 => 128,
            YMM9 => 256,

            MMX10 => 64,
            XMM10 => 128,
            YMM10 => 256,

            MMX11 => 64,
            XMM11 => 128,
            YMM11 => 256,

            MMX12 => 64,
            XMM12 => 128,
            YMM12 => 256,

            MMX13 => 64,
            XMM13 => 128,
            YMM13 => 256,

            MMX14 => 64,
            XMM14 => 128,
            YMM14 => 256,

            MMX15 => 64,
            XMM15 => 128,
            YMM15 => 256,
        }
    }
}

impl Display for AMD64Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use AMD64Register::*;
        write!(
            f,
            "{}",
            match self {
                AL => "%al",
                AX => "%ax",
                EAX => "%eax",
                RAX => "%rax",

                CL => "%cl",
                CX => "%cx",
                ECX => "%ecx",
                RCX => "%rcx",

                DL => "%dl",
                DX => "%dx",
                EDX => "%edx",
                RDX => "%rdx",

                BL => "%bl",
                BX => "%bx",
                EBX => "%ebx",
                RBX => "%rbx",

                SPL => "%spl",
                SP => "%sp",
                ESP => "%esp",
                RSP => "%rsp",

                BPL => "%bpl",
                BP => "%bp",
                EBP => "%ebp",
                RBP => "%rbp",

                SIL => "%sil",
                SI => "%si",
                ESI => "%esi",
                RSI => "%rsi",

                DIL => "%dil",
                DI => "%di",
                EDI => "%edi",
                RDI => "%rdi",

                R8B => "%r8b",
                R8L => "%r8l",
                R8W => "%r8w",
                R8D => "%r8d",
                R8 => "%r8",

                R9B => "%r9b",
                R9L => "%r9l",
                R9W => "%r9w",
                R9D => "%r9d",
                R9 => "%r9",

                R10B => "%r10b",
                R10L => "%r10l",
                R10W => "%r10w",
                R10D => "%r10d",
                R10 => "%r10",

                R11B => "%r11b",
                R11L => "%r11l",
                R11W => "%r11w",
                R11D => "%r11d",
                R11 => "%r11",

                R12B => "%r12b",
                R12L => "%r12l",
                R12W => "%r12w",
                R12D => "%r12d",
                R12 => "%r12",

                R13B => "%r13b",
                R13L => "%r13l",
                R13W => "%r13w",
                R13D => "%r13d",
                R13 => "%r13",

                R14B => "%r14b",
                R14L => "%r14l",
                R14W => "%r14w",
                R14D => "%r14d",
                R14 => "%r14",

                R15B => "%r15b",
                R15L => "%r15l",
                R15W => "%r15w",
                R15D => "%r15d",
                R15 => "%r15",

                MMX0 => "%mmx0",
                XMM0 => "%xmm0",
                YMM0 => "%ymm0",

                MMX1 => "%mmx1",
                XMM1 => "%xmm1",
                YMM1 => "%ymm1",

                MMX2 => "%mmx2",
                XMM2 => "%xmm2",
                YMM2 => "%ymm2",

                MMX3 => "%mmx3",
                XMM3 => "%xmm3",
                YMM3 => "%ymm3",

                MMX4 => "%mmx4",
                XMM4 => "%xmm4",
                YMM4 => "%ymm4",

                MMX5 => "%mmx5",
                XMM5 => "%xmm5",
                YMM5 => "%ymm5",

                MMX6 => "%mmx6",
                XMM6 => "%xmm6",
                YMM6 => "%ymm6",

                MMX7 => "%mmx7",
                XMM7 => "%xmm7",
                YMM7 => "%ymm7",

                MMX8 => "%mmx8",
                XMM8 => "%xmm8",
                YMM8 => "%ymm8",

                MMX9 => "%mmx9",
                XMM9 => "%xmm9",
                YMM9 => "%ymm9",

                MMX10 => "%mmx10",
                XMM10 => "%xmm10",
                YMM10 => "%ymm10",

                MMX11 => "%mmx11",
                XMM11 => "%xmm11",
                YMM11 => "%ymm11",

                MMX12 => "%mmx12",
                XMM12 => "%xmm12",
                YMM12 => "%ymm12",

                MMX13 => "%mmx13",
                XMM13 => "%xmm13",
                YMM13 => "%ymm13",

                MMX14 => "%mmx14",
                XMM14 => "%xmm14",
                YMM14 => "%ymm14",

                MMX15 => "%mmx15",
                XMM15 => "%xmm15",
                YMM15 => "%ymm15",
            }
        )
    }
}
