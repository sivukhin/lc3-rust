use crate::ops::*;
use crate::term;

const MEMORY_MAX: usize = 1 << 16;

const R0: Register = Register(0);
const R1: Register = Register(1);
const R2: Register = Register(2);
const R3: Register = Register(3);
const R4: Register = Register(4);
const R5: Register = Register(5);
const R6: Register = Register(6);
const R7: Register = Register(7);
const R_PC: Register = Register(8);
const R_COND: Register = Register(9);
const R_PC_INIT: u16 = 0x3000;
const REGISTERS: usize = 10;

enum CondState {
    Pos  = 1 << 0 as u16,
    Zero = 1 << 1 as u16,
    Neg  = 1 << 2 as u16,
}

impl CondState {
    fn test(self, value: u16) -> bool {
        value & self as u16 > 0
    }
}

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
    fn set_cond_reg(&mut self, register: Register);
    fn trap(&mut self, trap_vector: u16) -> bool;
}

impl VmMem for Vm {
    fn read_reg(&self, register: Register) -> u16 {
        self.registers[register.0]
    }
    fn write_reg(&mut self, register: Register, value: u16) {
        self.registers[register.0] = value;
    }
    fn set_cond_reg(&mut self, register: Register) {
        let value = self.read_reg(register);
        self.registers[R_COND.0] = if value == 0 {
            CondState::Zero
        } else if value < 1 << 15 {
            CondState::Pos
        } else {
            CondState::Neg
        } as u16;
    }
    fn read_mem(&self, address: u16) -> u16 {
        return match address {
            0xfe00 => {
                if term::hasc() {
                    1u16 << 15
                } else {
                    0
                }
            }
            0xfe02 => term::getc() as u16,
            0xfe04 => panic!("read access to DSR is not implemented"),
            0xfe06 => panic!("read access to DDR is not implemented"),
            0xfffe => panic!("read access to MCR is not implemented"),
            _ => self.memory[address as usize],
        };
    }
    fn write_mem(&mut self, address: u16, value: u16) {
        match address {
            0xfe00 | 0xfe02 | 0xfe04 | 0xfe06 | 0xfffe => panic!("write access to memory-mapped registers are forbidden"),
            _ => self.memory[address as usize] = value,
        }
    }
    fn c_str(&self, address: u16) -> Vec<u8> {
        return self.memory[address as usize..].iter().take_while(|&&x| x != 0).map(|&x| x as u8).collect();
    }
    fn trap(&mut self, trap_vector: u16) -> bool {
        match trap_vector {
            0x20 /* getc */ => self.write_reg(R0, term::getc() as u16),
            0x21 /* out */ => term::putc(self.read_reg(R0) as u8),
            0x22 /* puts */ => term::puts(&self.c_str(self.read_reg(R0))),
            0x25 /* halt */ => return false,
            _ => panic!("not implemented trap vector: {:#x}", trap_vector)
        }
        return true;
    }
}

pub trait VmEval {
    fn debug(&self);
    fn run(&mut self) -> Result<(), OpParseError>;
    fn tick(&mut self) -> Result<bool, OpParseError>;
    fn eval(&mut self, op: Operation) -> bool;
}

impl<T: VmMem> VmEval for T {
    fn debug(&self) {
        eprintln!("vm state: r0={}, r1={}, r2={}, r3={}, r4={}, r5={}, r6={}, r7={}, PC={:#x}, COND={:#05b}", self.read_reg(R0), self.read_reg(R1), self.read_reg(R2), self.read_reg(R3), self.read_reg(R4), self.read_reg(R5), self.read_reg(R6), self.read_reg(R7), self.read_reg(R_PC), self.read_reg(R_COND));
    }
    fn run(&mut self) -> Result<(), OpParseError> {
        loop {
            match self.tick() {
                Ok(true) => continue,
                Ok(false) => break,
                Err(e) => {
                    self.debug();
                    panic!("vm failed: error={}", e);
                }
            }
        }
        return Ok(());
    }
    fn tick(&mut self) -> Result<bool, OpParseError> {
        let pc = self.read_reg(R_PC);
        let op = Operation::parse(self.read_mem(pc))?;
        self.write_reg(R_PC, pc.wrapping_add(1));
        return Ok(self.eval(op));
    }
    fn eval(&mut self, op: Operation) -> bool {
        match op {
            Operation::OpAdd { dr, sr1, arg: Argument::Register(sr2) } => {
                self.write_reg(dr, self.read_reg(sr1).wrapping_add(self.read_reg(sr2)));
                self.set_cond_reg(dr);
            }
            Operation::OpAdd { dr, sr1, arg: Argument::Immediate(imm) } => {
                self.write_reg(dr, self.read_reg(sr1).wrapping_add(imm));
                self.set_cond_reg(dr);
            }
            Operation::OpAnd { dr, sr1, arg: Argument::Register(sr2) } => {
                self.write_reg(dr, self.read_reg(sr1).wrapping_add(self.read_reg(sr2)));
                self.set_cond_reg(dr);
            }
            Operation::OpAnd { dr, sr1, arg: Argument::Immediate(imm) } => {
                self.write_reg(dr, self.read_reg(sr1).wrapping_add(imm));
                self.set_cond_reg(dr);
            }
            Operation::OpBr { n, z, p, pc_offset } => {
                let cond = self.read_reg(R_COND);
                if n && CondState::Neg.test(cond) || z && CondState::Zero.test(cond) || p && CondState::Pos.test(cond) {
                    self.write_reg(R_PC, self.read_reg(R_PC).wrapping_add(pc_offset));
                }
            }
            Operation::OpJmp { base_r } => {
                self.write_reg(R_PC, self.read_reg(base_r));
            }
            Operation::OpJsr { pc_offset } => {
                self.write_reg(R7, self.read_reg(R_PC));
                self.write_reg(R_PC, self.read_reg(R_PC).wrapping_add(pc_offset));
            }
            Operation::OpJsrr { base_r } => {
                self.write_reg(R7, self.read_reg(R_PC));
                self.write_reg(R_PC, self.read_reg(base_r));
            }
            Operation::OpLd { dr, pc_offset } => {
                self.write_reg(dr, self.read_mem(self.read_reg(R_PC).wrapping_add(pc_offset)));
                self.set_cond_reg(dr);
            }
            Operation::OpLdi { dr, pc_offset } => {
                let address = self.read_mem(self.read_reg(R_PC).wrapping_add(pc_offset));
                self.write_reg(dr, self.read_mem(address));
                self.set_cond_reg(dr);
            }
            Operation::OpLdr { dr, base_r, offset } => {
                self.write_reg(dr, self.read_mem(self.read_reg(base_r).wrapping_add(offset)));
                self.set_cond_reg(dr);
            }
            Operation::OpLea { dr, pc_offset } => {
                self.write_reg(dr, self.read_reg(R_PC).wrapping_add(pc_offset));
                self.set_cond_reg(dr);
            }
            Operation::OpNot { dr, sr } => {
                self.write_reg(dr, !self.read_reg(sr));
                self.set_cond_reg(dr);
            }
            Operation::OpRti => panic!("rti operation is not implemented"),
            Operation::OpSt { sr, pc_offset } => {
                self.write_mem(self.read_reg(R_PC).wrapping_add(pc_offset), self.read_reg(sr));
            }
            Operation::OpSti { sr, pc_offset } => {
                let address = self.read_mem(self.read_reg(R_PC).wrapping_add(pc_offset));
                self.write_mem(address, self.read_reg(sr));
            }
            Operation::OpStr { sr, base_r, offset } => {
                self.write_mem(self.read_reg(base_r).wrapping_add(offset), self.read_reg(sr));
            }
            Operation::OpTrap { trap_vector } => {
                self.write_reg(R7, self.read_reg(R_PC));
                return self.trap(trap_vector);
            }
        }
        return true;
    }
}

impl Vm {
    pub fn new<T: IntoIterator<Item = u16>>(program: T) -> Result<Self, &'static str> {
        let mut program = program.into_iter();
        let origin = program.next().ok_or("empty program provided")?;
        let mut vm = Vm { memory: [0u16; MEMORY_MAX], registers: [0u16; REGISTERS] };
        for (i, value) in program.enumerate() {
            vm.memory[origin as usize + i] = value;
        }
        vm.write_reg(R_PC, R_PC_INIT);
        vm.write_reg(R_COND, CondState::Zero as u16);
        Ok(vm)
    }
}
