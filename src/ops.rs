#[derive(Clone, Copy)]
pub struct Register(pub usize);

#[derive(Clone, Copy)]
pub enum Argument {
    Register(Register),
    Immediate(u16),
}

#[derive(Clone, Copy)]
pub enum Operation {
    Add { dr: Register, sr1: Register, arg: Argument },  /* add  */
    And { dr: Register, sr1: Register, arg: Argument },  /* bitwise and */
    Br { n: bool, z: bool, p: bool, pc_offset: u16 },    /* branch */
    Jmp { base_r: Register },                            /* jump */
    Jsr { pc_offset: u16 },                              /* jump register */
    Jsrr { base_r: Register },                           /* jump register */
    Ld { dr: Register, pc_offset: u16 },                 /* load */
    Ldi { dr: Register, pc_offset: u16 },                /* load indirect */
    Ldr { dr: Register, base_r: Register, offset: u16 }, /* load register */
    Lea { dr: Register, pc_offset: u16 },                /* load effective address */
    Not { dr: Register, sr: Register },                  /* bitwise not */
    St { sr: Register, pc_offset: u16 },                 /* store */
    Sti { sr: Register, pc_offset: u16 },                /* store indirect */
    Str { sr: Register, base_r: Register, offset: u16 }, /* store register */
    Trap { trap_vector: u16 },                           /* execute trap */
    Rti,                                                 /* unused */
}

