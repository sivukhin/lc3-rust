use core::fmt;

use crate::ops;

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
            Self::OpJsrr { base_r } => f.debug_struct("OpJsrr").field("base_r", base_r).finish(),
            Self::OpLd { dr, pc_offset } => f.debug_struct("OpLd").field("dr", dr).field("pc_offset", pc_offset).finish(),
            Self::OpLdi { dr, pc_offset } => f.debug_struct("OpLdi").field("dr", dr).field("pc_offset", pc_offset).finish(),
            Self::OpLdr { dr, base_r, offset } => f.debug_struct("OpLdr").field("dr", dr).field("base_r", base_r).field("offset", offset).finish(),
            Self::OpLea { dr, pc_offset } => f.debug_struct("OpLea").field("dr", dr).field("pc_offset", pc_offset).finish(),
            Self::OpNot { dr, sr } => f.debug_struct("OpNot").field("dr", dr).field("sr", sr).finish(),
            Self::OpSt { sr, pc_offset } => f.debug_struct("OpSt").field("sr", sr).field("pc_offset", pc_offset).finish(),
            Self::OpSti { sr, pc_offset } => f.debug_struct("OpSti").field("sr", sr).field("pc_offset", pc_offset).finish(),
            Self::OpStr { sr, base_r, offset } => f.debug_struct("OpStr").field("sr", sr).field("base_r", base_r).field("offset", offset).finish(),
            Self::OpTrap { trap_vector } => f.debug_struct("OpTrap").field("trap_vector", trap_vector).finish(),
            Self::OpRti => write!(f, "OpRti"),
        }
    }
}

impl fmt::Display for ops::OpParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FixedMismatch { code, segment, expected, actual } => {
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
            Self::IllegalOpcode { code } => write!(f, "illegal op code: code={:04b}, op={:016b}", code >> 12, code),
        }
    }
}


