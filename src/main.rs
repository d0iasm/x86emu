use std::env;
use std::process;
use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::vec::Vec;

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

struct Emulator {
    regs: [u32; REGISTERS_COUNT],
    eflags: u32,
    mem: Vec<u8>,
    eip: usize,
}

fn create_emu(eip: usize, esp: u32) -> Emulator {
    let memory = Vec::new();
    let mut registers = [0; REGISTERS_COUNT];
    registers[ESP] = esp;
    return Emulator {
        regs: registers,
        eflags: 0,
        mem: memory,
        eip: eip,
    };
}

fn dump_registers(emu: &mut Emulator) {
    for i in 0..REGISTERS_COUNT {
        println!("{0} = {1}", REGISTERS_NAME[i], emu.regs[i])
    }
    println!("EIP = {}", emu.eip)
}

fn get_code8(emu: &mut Emulator, index: usize) -> u32 {
    return emu.mem[emu.eip + index] as u32;
}

fn get_sign_code8(emu: &mut Emulator, index: usize) -> i32 {
    return emu.mem[emu.eip + index] as i32;
}

fn get_code32(emu: &mut Emulator, index: usize) -> u32 {
    let mut ret: u32 = 0;

    // Little endian.
    for i in 0..4 {
        ret |= get_code8(emu, index + i) << (i * 8);
    }
    return ret;
}

// MOV r32, imm32: Move imm32 to r32.
fn mov_r32_imm32(emu: &mut Emulator) {
    let reg: usize = (get_code8(emu, 0) - 0xB8).try_into().unwrap();
    let value = get_code32(emu, 1);
    emu.regs[reg] = value;
    emu.eip += 5;
}

// JMP rel8: Jump short, relative, displacement relative to next instruction.
fn short_jump(emu: &mut Emulator) {
    let diff = get_sign_code8(emu, 1) as usize;
    emu.eip += diff + 2;
}

fn nop(_emu: &mut Emulator) {
}

fn init_instructions(instructions: &mut Insts) {
	for i in 0..8 {
        instructions[0xB8 + i] = mov_r32_imm32;
	}
    instructions[0xEB] = short_jump;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: x86emu filename");
        process::exit(1);
    }

    let mut emu = create_emu(0x0000, 0x7c00);

    let path = Path::new(&args[1]);
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
    emu.mem = binary;

    let mut instructions: Insts = [nop; 256];
    init_instructions(&mut instructions);

    while emu.eip < MEMORY_SIZE {
        let code = get_code8(&mut emu, 0) as usize;
        println!("EIP = {}, Code = {}", emu.eip, code);

        if instructions[code] as usize == nop as usize {
            println!("Not implemented: {0}", code);
            break;
        }

        // Execute an instruction.
        instructions[code](&mut emu);

        // TODO: when does a program finish?
        if emu.eip == file_len as usize {
            println!("\nEnd of program.\n");
            break;
        }
    }

    dump_registers(&mut emu);
}
