use crate::io;
use crate::ops::*;

pub const MEMORY_MAX: usize = 1 << 16;
pub const REGISTERS: usize = 10;

pub struct Vm {
    memory:    [u16; MEMORY_MAX],
    registers: [u16; REGISTERS],
}

pub trait VmMem {
    fn read_reg(&self, register: Register) -> u16;
    fn write_reg(&mut self, register: Register, value: u16);
    fn read_mem(&self, address: u16) -> u16;
    fn write_mem(&mut self, address: u16, value: u16);
    fn c_str(&self, address: u16) -> Vec<u8>;
}

impl VmMem for Vm {
    fn read_reg(&self, register: Register) -> u16 {
        self.registers[register.0]
    }
    fn write_reg(&mut self, register: Register, value: u16) {
        self.registers[register.0] = value;
    }
    fn read_mem(&self, address: u16) -> u16 {
        match address {
            0xfe00 => match io::hasc() {
                Ok(true) => 1u16 << 15,
                _ => 0,
            },
            0xfe02 => io::getc().unwrap_or(0) as u16,
            0xfe04 => panic!("read access to DSR is not implemented"),
            0xfe06 => panic!("read access to DDR is not implemented"),
            0xfffe => panic!("read access to MCR is not implemented"),
            _ => self.memory[address as usize],
        }
    }
    fn write_mem(&mut self, address: u16, value: u16) {
        match address {
            0xfe00 | 0xfe02 | 0xfe04 | 0xfe06 | 0xfffe => panic!("write access to memory-mapped registers are forbidden"),
            _ => self.memory[address as usize] = value,
        }
    }
    fn c_str(&self, address: u16) -> Vec<u8> {
        self.memory[address as usize..].iter().take_while(|&&x| x != 0).map(|&x| x as u8).collect()
    }
}

impl Default for Vm {
    fn default() -> Self {
        Self { memory: [0u16; MEMORY_MAX], registers: [0u16; REGISTERS] }
    }
}
