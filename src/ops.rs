use std::ops::Range;

#[derive(Clone, Copy)]
pub struct Register(pub usize);

#[derive(Clone, Copy)]
pub enum Argument {
    Register(Register),
    Immediate(u16),
}

pub enum Operation {
    OpAdd { dr: Register, sr1: Register, arg: Argument },  /* add  */
    OpAnd { dr: Register, sr1: Register, arg: Argument },  /* bitwise and */
    OpBr { n: bool, z: bool, p: bool, pc_offset: u16 },    /* branch */
    OpJmp { base_r: Register },                            /* jump */
    OpJsr { pc_offset: u16 },                              /* jump register */
    OpJsrr { base_r: Register },                           /* jump register */
    OpLd { dr: Register, pc_offset: u16 },                 /* load */
    OpLdi { dr: Register, pc_offset: u16 },                /* load indirect */
    OpLdr { dr: Register, base_r: Register, offset: u16 }, /* load register */
    OpLea { dr: Register, pc_offset: u16 },                /* load effective address */
    OpNot { dr: Register, sr: Register },                  /* bitwise not */
    OpSt { sr: Register, pc_offset: u16 },                 /* store */
    OpSti { sr: Register, pc_offset: u16 },                /* store indirect */
    OpStr { sr: Register, base_r: Register, offset: u16 }, /* store register */
    OpTrap { trap_vector: u16 },                           /* execute trap */
    OpRti,                                                 /* unused */
}

#[derive(Debug)]
pub enum OpParseError {
    FixedMismatch { code: u16, segment: Range<i32>, expected: u16, actual: u16 },
    IllegalOpcode { code: u16 },
}
