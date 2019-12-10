#![allow(clippy::unreadable_literal)]

/// Section 38.9.1.1, Table 38-10
#[derive(Copy, Clone, Debug)]
pub enum Exception {
    Divider,
    Debug,
    Breakpoint,
    BoundRange,
    InvalidOpCode,
    GeneralProtection,
    PageFault,
    FloatingPoint,
    AlignmentCheck,
    Simd,
}

/// Section 38.9.1.1, Table 38-9
#[derive(Copy, Clone, Debug)]
pub enum ExitType {
    Hardware,
    Software,
}

/// Section 38.9.1.1, Table 38-9
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct ExitInfo(u32);

impl std::fmt::Debug for ExitInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let exc = self.exception();
        let et = self.exit_type();

        let status = et.and_then(|et| exc.map(|e| (et, e)));
        write!(f, "ExitInfo({:?})", status)
    }
}

impl ExitInfo {
    pub fn new(et: ExitType, exc: Exception) -> Self {
        let et = match et {
            ExitType::Hardware => 0b011 << 8,
            ExitType::Software => 0b110 << 8,
        };

        let exc = match exc {
            Exception::Divider => 0x00,
            Exception::Debug => 0x01,
            Exception::Breakpoint => 0x03,
            Exception::BoundRange => 0x05,
            Exception::InvalidOpCode => 0x06,
            Exception::GeneralProtection => 0x0d,
            Exception::PageFault => 0x0e,
            Exception::FloatingPoint => 0x10,
            Exception::AlignmentCheck => 0x11,
            Exception::Simd => 0x13,
        };

        ExitInfo(1 << 31 | et | exc)
    }

    pub fn exit_type(self) -> Option<ExitType> {
        const MASK: u32 = 1 << 31 | 0b111 << 8;

        match self.0 & MASK {
            0x80000300 => Some(ExitType::Hardware),
            0x80000600 => Some(ExitType::Software),
            _ => None,
        }
    }

    pub fn exception(self) -> Option<Exception> {
        const MASK: u32 = 1 << 31 | 0xff;

        match self.0 & MASK {
            0x80000000 => Some(Exception::Divider),
            0x80000001 => Some(Exception::Debug),
            0x80000003 => Some(Exception::Breakpoint),
            0x80000005 => Some(Exception::BoundRange),
            0x80000006 => Some(Exception::InvalidOpCode),
            0x8000000d => Some(Exception::GeneralProtection),
            0x8000000e => Some(Exception::PageFault),
            0x80000010 => Some(Exception::FloatingPoint),
            0x80000011 => Some(Exception::AlignmentCheck),
            0x80000013 => Some(Exception::Simd),
            _ => None,
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct GprSgx {
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rflags: u64,
    pub rip: u64,
    pub ursp: u64,
    pub urbp: u64,
    pub exitinfo: ExitInfo,
    pub reserved: u32,
    pub fsbase: u64,
    pub gsbase: u64,
}

testaso! {
    struct GprSgx: 8, 184 => {
        rax: 0,
        rcx: 8,
        rdx: 16,
        rbx: 24,
        rsp: 32,
        rbp: 40,
        rsi: 48,
        rdi: 56,
        r8: 64,
        r9: 72,
        r10: 80,
        r11: 88,
        r12: 96,
        r13: 104,
        r14: 112,
        r15: 120,
        rflags: 128,
        rip: 136,
        ursp: 144,
        urbp: 152,
        exitinfo: 160,
        reserved: 164,
        fsbase: 168,
        gsbase: 176
    }
}