use crate::io;
use crate::ops::*;
use crate::ops_parse;
use crate::vm;

const R0: Register = Register(0);
const R7: Register = Register(7);
const R_PC: Register = Register(8);
const R_COND: Register = Register(9);
const R_PC_INIT: u16 = 0x3000;

const COND_P: u16 = 1 << 0u16;
const COND_Z: u16 = 1 << 1u16;
const COND_N: u16 = 1 << 2u16;

pub enum TickError {
    Io(io::IoError),
    Parse(ops_parse::ParseError),
}

pub enum LoadError {
    EmptyProgram,
}

pub fn run(vm: &mut impl VmSpec) -> Result<(), TickError> {
    loop {
        match vm.tick() {
            Ok(true) => continue,
            Ok(false) => return Ok(()),
            Err(e) => return Err(e),
        }
    }
}

pub trait VmSpec where Self: Sized {
    fn load(obj: &[u16]) -> Result<Self, LoadError>;
    fn tick(&mut self) -> Result<bool, TickError>; 
    fn tick_op(&mut self, op: Operation) -> Result<bool, io::IoError>;
    fn trap(&mut self, trap_vector: u16) -> Result<bool, io::IoError>;
}

fn set_cond_reg(vm_mem: &mut impl vm::VmMem, register: Register) {
    let value = vm_mem.read_reg(register);
    if value == 0 {
        vm_mem.write_reg(R_COND, COND_Z);
    } else if value < 1 << 15 {
        vm_mem.write_reg(R_COND, COND_P);
    } else {
        vm_mem.write_reg(R_COND, COND_N);
    }
}

impl<T: vm::VmMem+Default> VmSpec for T {
    fn load(obj: &[u16]) -> Result<T, LoadError> {
        if obj.is_empty() {
            return Err(LoadError::EmptyProgram);
        }
        let origin = obj[0];
        let mut vm = T::default();
        for (i, &value) in obj[1..].iter().enumerate() {
            vm.write_mem(origin + i as u16, value);
        }
        vm.write_reg(R_PC, R_PC_INIT);
        vm.write_reg(R_COND, COND_Z);
        Ok(vm)
    }
    fn trap(&mut self, trap_vector: u16) -> Result<bool, io::IoError> {
        match trap_vector {
            0x20 /* getc */ => self.write_reg(R0, io::getc()? as u16),
            0x21 /* out */ => io::putc(self.read_reg(R0) as u8)?,
            0x22 /* puts */ => io::puts(&self.c_str(self.read_reg(R0)))?,
            0x25 /* halt */ => return Ok(false),
            _ => panic!("not implemented trap vector: {:#x}", trap_vector)
        }
        Ok(true)
    }
    fn tick(&mut self) -> Result<bool, TickError> {
        let pc = self.read_reg(R_PC);
        let op = Operation::parse(self.read_mem(pc)).map_err(TickError::Parse)?;
        self.write_reg(R_PC, pc.wrapping_add(1));
        self.tick_op(op).map_err(TickError::Io)
    }
    fn tick_op(&mut self, op: Operation) -> Result<bool, io::IoError> {
        match op {
            Operation::Add { dr, sr1, arg: Argument::Register(sr2) } => {
                self.write_reg(dr, self.read_reg(sr1).wrapping_add(self.read_reg(sr2)));
                set_cond_reg(self, dr);
            }
            Operation::Add { dr, sr1, arg: Argument::Immediate(imm) } => {
                self.write_reg(dr, self.read_reg(sr1).wrapping_add(imm));
                set_cond_reg(self, dr);
            }
            Operation::And { dr, sr1, arg: Argument::Register(sr2) } => {
                self.write_reg(dr, self.read_reg(sr1) & self.read_reg(sr2));
                set_cond_reg(self, dr);
            }
            Operation::And { dr, sr1, arg: Argument::Immediate(imm) } => {
                self.write_reg(dr, self.read_reg(sr1) & imm);
                set_cond_reg(self, dr);
            }
            Operation::Br { n, z, p, pc_offset } => {
                let cond = self.read_reg(R_COND);
                if n && (COND_N & cond) != 0 || z && (COND_Z & cond) != 0 || p && (COND_P & cond) != 0 {
                    self.write_reg(R_PC, self.read_reg(R_PC).wrapping_add(pc_offset));
                }
            }
            Operation::Jmp { base_r } => {
                self.write_reg(R_PC, self.read_reg(base_r));
            }
            Operation::Jsr { pc_offset } => {
                self.write_reg(R7, self.read_reg(R_PC));
                self.write_reg(R_PC, self.read_reg(R_PC).wrapping_add(pc_offset));
            }
            Operation::Jsrr { base_r } => {
                self.write_reg(R7, self.read_reg(R_PC));
                self.write_reg(R_PC, self.read_reg(base_r));
            }
            Operation::Ld { dr, pc_offset } => {
                self.write_reg(dr, self.read_mem(self.read_reg(R_PC).wrapping_add(pc_offset)));
                set_cond_reg(self, dr);
            }
            Operation::Ldi { dr, pc_offset } => {
                let address = self.read_mem(self.read_reg(R_PC).wrapping_add(pc_offset));
                self.write_reg(dr, self.read_mem(address));
                set_cond_reg(self, dr);
            }
            Operation::Ldr { dr, base_r, offset } => {
                self.write_reg(dr, self.read_mem(self.read_reg(base_r).wrapping_add(offset)));
                set_cond_reg(self, dr);
            }
            Operation::Lea { dr, pc_offset } => {
                self.write_reg(dr, self.read_reg(R_PC).wrapping_add(pc_offset));
                set_cond_reg(self, dr);
            }
            Operation::Not { dr, sr } => {
                self.write_reg(dr, !self.read_reg(sr));
                set_cond_reg(self, dr);
            }
            Operation::Rti => panic!("rti operation is not implemented"),
            Operation::St { sr, pc_offset } => {
                self.write_mem(self.read_reg(R_PC).wrapping_add(pc_offset), self.read_reg(sr));
            }
            Operation::Sti { sr, pc_offset } => {
                let address = self.read_mem(self.read_reg(R_PC).wrapping_add(pc_offset));
                self.write_mem(address, self.read_reg(sr));
            }
            Operation::Str { sr, base_r, offset } => {
                self.write_mem(self.read_reg(base_r).wrapping_add(offset), self.read_reg(sr));
            }
            Operation::Trap { trap_vector } => {
                self.write_reg(R7, self.read_reg(R_PC));
                return self.trap(trap_vector);
            }
        }
        Ok(true)
    }
}
