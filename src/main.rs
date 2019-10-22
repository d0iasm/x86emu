use std::env;
use std::process;
use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::vec::Vec;

pub mod instruction;
pub mod function;
pub mod modrm;
use instruction::*;
use function::*;

// Memory size is 1MiB.
const MEMORY_SIZE: usize = 1024 * 1024;

const EAX: usize = 0;
const ECX: usize = 1;
const EDX: usize = 2;
const EBX: usize = 3;
const ESP: usize = 4;
const EBP: usize = 5;
const ESI: usize = 6;
const EDI: usize = 7;
const REGISTERS_COUNT: usize = 8;
const REGISTERS_NAME: [&str; 8] = ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];

type InstFunc = fn(&mut Emulator);
type Insts = [InstFunc; 256];

pub struct Emulator {
    regs: [u32; REGISTERS_COUNT],
    eflags: u32,
    mem: Vec<u8>,
    eip: usize,
}

fn read_binary(emu: &mut Emulator, filename: &String) -> u64 {
    let path = Path::new(&filename);
	let display = path.display();

	let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };
    let file_len = file.metadata().unwrap().len();

	let mut binary = Vec::<u8>::new();
    match file.read_to_end(&mut binary) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => println!("read file from {}\n", display),
    }

    // BIOS loads MBR (Master Boot Record) into 0x7c00 in x86 system.
    emu.mem = vec![0; 0x7c00];
    emu.mem.extend(binary);

    return file_len;
}

fn create_emu(eip: usize, esp: u32) -> Emulator {
    let mut registers = [0; REGISTERS_COUNT];
    registers[ESP] = esp;
    return Emulator {
        regs: registers,
        eflags: 0,
        mem: Vec::new(),
        eip: eip,
    };
}

fn dump_registers(emu: &mut Emulator) {
    println!("----- registers -----");
    for i in 0..REGISTERS_COUNT {
        println!("{0} = {1}", REGISTERS_NAME[i], get_register32(emu, i));
    }
    println!("EIP = {}", emu.eip);
}

fn dump_stack(emu: &mut Emulator) {
    println!("----- stack -----");
    for i in 0..10 {
        let address = emu.regs[ESP] - 4 * i;
        let value = get_memory32(emu, address.try_into().unwrap());
        println!("stack [{}]: {}", address, value);
    }
}

fn dump_eflags(emu: &Emulator) {
    println!("----- eflags -----");
    println!("carry: {}", emu.eflags & 1);
    println!("zero: {}", (emu.eflags & (1 << 6)) >> 6);
    println!("sign: {}", (emu.eflags & (1 << 7)) >> 7);
    println!("overflow: {}", (emu.eflags & (1 << 11)) >> 11);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: x86emu filename");
        process::exit(1);
    }

    let mut emu = create_emu(0x7c00, 0x7c00);
    let len = read_binary(&mut emu, &args[1]) + 0x7c00;

    let mut instructions: Insts = [nop; 256];
    init_instructions(&mut instructions);

    while emu.eip < MEMORY_SIZE {
        let code = get_code8(&mut emu, 0) as usize;
        println!("eip = {}, code = {} ({:x})", emu.eip, code, code);

        if instructions[code] as usize == nop as usize {
            println!("not implemented: {0}", code);
            break;
        }

        // Execute an instruction.
        instructions[code](&mut emu);

        // TODO: when does a program finish?
        if emu.eip >= len as usize || emu.eip == 0 {
            println!("\nend of program \n");
            break;
        }
    }

    dump_stack(&mut emu);
    dump_registers(&mut emu);
    dump_eflags(&emu);
}
