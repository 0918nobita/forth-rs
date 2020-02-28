mod context;
mod interp;
mod value;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufReader, Read};
use std::mem::{self, MaybeUninit};
use std::slice;

#[repr(C)]
struct MachOHeader {
    magic: u32,
    cpu_type: i32,
    cpu_subtype: i32,
    file_type: u32,
    num_cmds: u32,
    size_of_cmds: u32,
    flags: u32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let data = fs::read(args.get(1).unwrap()).expect("failed to open file");
        let data = data.as_slice();
        let header: MachOHeader = read_struct(BufReader::new(data)).expect("failed to read struct");
        println!("magic: 0x{:08x}", header.magic);
        println!("cpu type: {}", header.cpu_type);
        println!("cpu subtype: {}", header.cpu_subtype);
        println!("file type: {}", header.file_type);
        println!("num cmds: {}", header.num_cmds);
        println!("size of cmds: {}", header.size_of_cmds);
        println!("flags: 0x{:08x}", header.flags);
        return;
    }

    println!("\x1b[1m\x1b[34mForth interpreter\x1b[m");

    let mut context = context::Context {
        dict: &mut HashMap::new(),
        stack: &mut Vec::new(),
        mem: &mut Vec::with_capacity(1000),
    };

    context
        .dict
        .insert(String::from("mul2"), String::from("dup + "));

    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();

        let words: Vec<String> = s
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();

        for word in words {
            interp::interpret(&mut context, word)
        }

        println!("  \x1b[1m\x1b[32mok\x1b[m")
    }
}

fn read_struct<T, R: Read>(mut read: R) -> io::Result<T> {
    let num_bytes = mem::size_of::<T>();
    let mut struct_val = unsafe { MaybeUninit::zeroed().assume_init() };
    let buffer =
        unsafe { slice::from_raw_parts_mut(&mut struct_val as *mut T as *mut u8, num_bytes) };
    match read.read_exact(buffer) {
        Ok(()) => Ok(struct_val),
        Err(e) => {
            mem::forget(struct_val);
            Err(e)
        }
    }
}
