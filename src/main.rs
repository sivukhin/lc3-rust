use std::{env, fs};

mod debug;
mod ops;
mod ops_parse;
mod io;
mod vm;
mod vm_spec;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;
    use crate::ops;

    #[test]
    fn generate_program() {
        let mut buf = Vec::with_capacity(1024);
        buf.append(&mut 0x3000u16.to_be_bytes().to_vec());
        buf.append(&mut 0b0001_001_001_1_00101u16.to_be_bytes().to_vec());
        buf.append(&mut 0b0001_000_001_0_00_001u16.to_be_bytes().to_vec());
        buf.append(&mut 0b0001_000_000_0_00_000u16.to_be_bytes().to_vec());
        buf.append(&mut 0b0001_000_000_0_00_001u16.to_be_bytes().to_vec());
        buf.append(&mut 0b0001_000_000_0_00_000u16.to_be_bytes().to_vec());
        buf.append(&mut 0b1101_000_000_0_00_000u16.to_be_bytes().to_vec());
        let mut f = fs::OpenOptions::new().create(true).write(true).open("examples/example.obj").unwrap();
        f.write_all(&buf).unwrap();
    }

    #[test]
    fn parse() {
        println!("{}", ops::Operation::parse(0b1111111100000000).unwrap_err());
    }
}

fn main() {
    io::term_setup().unwrap_or_else(|e| panic!("terminal setup failed: {}", e));
    let obj_path = &env::args().skip(1).next().unwrap_or_else(|| panic!("object path must be provided as first argument"));
    let obj_bytes = fs::read(obj_path).unwrap_or_else(|e| panic!("object file '{}' not found: {}", obj_path, e));
    assert!(obj_bytes.len() % 2 == 0, "object file must have even length: length('{}')={}", obj_path, obj_bytes.len());
    let obj_values = obj_bytes.chunks_exact(2).map(|w| u16::from_be_bytes(w.try_into().unwrap()));
    let mut vm = vm::Vm::new(obj_values).unwrap_or_else(|e| panic!("unable to load vm: {}", e));
    vm_spec::run(&mut vm).unwrap_or_else(|e| panic!("vm failed: {}", e));
}
