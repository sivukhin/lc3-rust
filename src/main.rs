use std::{env, fs};

// David: usually cleaner to move mods to lib.rs, this would allow you to have multiple binaries for example in bin/
use lc3_rust::{io, vm, vm_spec};

fn main() {
    io::term_setup().unwrap_or_else(|e| panic!("terminal setup failed: {}", e));
    let obj_path = &env::args().nth(1).unwrap_or_else(|| panic!("object path must be provided as first argument"));
    let obj_bytes = fs::read(obj_path).unwrap_or_else(|e| panic!("object file '{}' not found: {}", obj_path, e));
    assert!(obj_bytes.len() % 2 == 0, "object file must have even length: length('{}')={}", obj_path, obj_bytes.len());
    let obj_values: Vec<u16> = obj_bytes.chunks_exact(2).map(|w| u16::from_be_bytes(w.try_into().unwrap())).collect();
    let mut vm: vm::Vm = vm_spec::VmSpec::load(&obj_values).unwrap_or_else(|e| panic!("unable to load vm: {}", e));
    vm_spec::run(&mut vm).unwrap_or_else(|e| panic!("vm failed: {}", e));
}
