// David: You would use std unless you're on #![no_std]
// I would also to import items directly when obvious and fully qualify when not 
use std::fmt::{Debug, Display, Formatter};

use crate::io;
use crate::ops;
use crate::ops_parse;
use crate::vm_spec;

#[derive(Clone, Copy)]
pub struct VmInt(pub u16);

// David: sidenote, I would tend to colocate trait definition with their object rather than having a file dedicated to implementing traits
impl Debug for ops::Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "R{}", self.0)
    }
}

impl Debug for VmInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", if self.0 < 1 << 15 { self.0 as i32 } else { -(!self.0 as i32) })
    }
}

impl Debug for ops::Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Register(r) => write!(f, "{:?}", r),
            Self::Immediate(imm) => write!(f, "{:?}", VmInt(imm)),
        }
    }
}

impl Debug for ops::Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            // David: this is all very rusty, one thing to make it even more readable is that format string support capture since 1.58
            // See example below and: https://blog.rust-lang.org/2022/01/13/Rust-1.58.0.html#captured-identifiers-in-format-strings
            Self::Add { dr, sr1, arg } => write!(f, "add({dr:?}, {sr1:?}, {arg:?})"),
            Self::And { dr, sr1, arg } => write!(f, "add({:?}, {:?}, {:?})", dr, sr1, arg),
            Self::Br { n, z, p, pc_offset } => write!(f, "br(nzp=({}, {}, {}), {:?})", n, z, p, VmInt(pc_offset)),
            Self::Jmp { base_r } => write!(f, "jmp({:?})", base_r),
            Self::Jsr { pc_offset } => write!(f, "jsr({:?})", VmInt(pc_offset)),
            Self::Jsrr { base_r } => write!(f, "jsrr({:?})", base_r),
            Self::Ld { dr, pc_offset } => write!(f, "ld({:?}, {:?})", dr, VmInt(pc_offset)),
            Self::Ldi { dr, pc_offset } => write!(f, "ldi({:?}, {:?})", dr, VmInt(pc_offset)),
            Self::Ldr { dr, base_r, offset } => write!(f, "ldr({:?}, {:?}, {:?})", dr, base_r, VmInt(offset)),
            Self::Lea { dr, pc_offset } => write!(f, "lea({:?}, {:?})", dr, pc_offset),
            Self::Not { dr, sr } => write!(f, "not({:?}, {:?})", dr, sr),
            Self::St { sr, pc_offset } => write!(f, "st({:?}, {:?})", sr, VmInt(pc_offset)),
            Self::Sti { sr, pc_offset } => write!(f, "sti({:?}, {:?})", sr, pc_offset),
            Self::Str { sr, base_r, offset } => write!(f, "str({:?}, {:?}, {:?})", sr, base_r, VmInt(offset)),
            Self::Trap { trap_vector } => write!(f, "trap({:#x})", trap_vector),
            Self::Rti => write!(f, "rti"),
        }
    }
}

impl Display for ops_parse::ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
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

impl Display for io::IoError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for vm_spec::TickError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error: {}", e),
            Self::Parse(e) => write!(f, "parse error: {}", e),
        }
    }
}

impl Display for vm_spec::LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            &Self::EmptyProgram => write!(f, "empty program provided"),
        }
    }
}
