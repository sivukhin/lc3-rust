use core::fmt;

use crate::io;
use crate::ops;
use crate::ops_parse;
use crate::vm_spec;

#[derive(Clone, Copy)]
pub struct VmInt(pub u16);

impl core::fmt::Debug for ops::Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "R{}", self.0)
    }
}

impl core::fmt::Debug for VmInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}", if self.0 < 1 << 15 { self.0 as i32 } else { -(!self.0 as i32) })
    }
}

impl core::fmt::Debug for ops::Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Self::Register(r) => write!(f, "{:?}", r),
            &Self::Immediate(imm) => write!(f, "{:?}", VmInt(imm)),
        }
    }
}

impl core::fmt::Debug for ops::Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Self::OpAdd { dr, sr1, arg } => write!(f, "add({:?}, {:?}, {:?})", dr, sr1, arg),
            &Self::OpAnd { dr, sr1, arg } => write!(f, "add({:?}, {:?}, {:?})", dr, sr1, arg),
            &Self::OpBr { n, z, p, pc_offset } => write!(f, "br(nzp=({}, {}, {}), {:?})", n, z, p, VmInt(pc_offset)),
            &Self::OpJmp { base_r } => write!(f, "jmp({:?})", base_r),
            &Self::OpJsr { pc_offset } => write!(f, "jsr({:?})", VmInt(pc_offset)),
            &Self::OpJsrr { base_r } => write!(f, "jsrr({:?})", base_r),
            &Self::OpLd { dr, pc_offset } => write!(f, "ld({:?}, {:?})", dr, VmInt(pc_offset)),
            &Self::OpLdi { dr, pc_offset } => write!(f, "ldi({:?}, {:?})", dr, VmInt(pc_offset)),
            &Self::OpLdr { dr, base_r, offset } => write!(f, "ldr({:?}, {:?}, {:?})", dr, base_r, VmInt(offset)),
            &Self::OpLea { dr, pc_offset } => write!(f, "lea({:?}, {:?})", dr, pc_offset),
            &Self::OpNot { dr, sr } => write!(f, "not({:?}, {:?})", dr, sr),
            &Self::OpSt { sr, pc_offset } => write!(f, "st({:?}, {:?})", sr, VmInt(pc_offset)),
            &Self::OpSti { sr, pc_offset } => write!(f, "sti({:?}, {:?})", sr, pc_offset),
            &Self::OpStr { sr, base_r, offset } => write!(f, "str({:?}, {:?}, {:?})", sr, base_r, VmInt(offset)),
            &Self::OpTrap { trap_vector } => write!(f, "trap({:#x})", trap_vector),
            &Self::OpRti => write!(f, "rti"),
        }
    }
}

impl fmt::Display for ops_parse::ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &Self::FixedMismatch { code, segment, expected, actual } => {
                write!(f, "fixed segment mismatch: expected={:0width$b}, actual={:0width$b}, op=", expected, actual, width = (segment.end - segment.start) as usize)?;
                if segment.end < 16 {
                    write!(f, "{:0width$b}", code >> segment.end, width = (16 - segment.end) as usize)?;
                }
                write!(f, "[{:0width$b}]", (code >> segment.start) & ((1 << (segment.end - segment.start)) - 1), width = (segment.end - segment.start) as usize)?;
                if segment.start > 0 {
                    write!(f, "{:0width$b}", code & ((1 << segment.start) - 1), width = segment.start as usize)?;
                }
                Ok(())
            }
            &Self::IllegalOpcode { code } => write!(f, "illegal op code: code={:04b}, op={:016b}", code >> 12, code),
        }
    }
}

impl fmt::Display for io::IoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for vm_spec::TickError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &Self::Io(e) => write!(f, "io error: {}", e),
            &Self::Parse(e) => write!(f, "parse error: {}", e),
        }
    }
}
