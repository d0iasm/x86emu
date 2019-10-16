use std::convert::TryInto;

use crate::*;

pub fn get_code8(emu: &mut Emulator, index: usize) -> u32 {
    return emu.mem[emu.eip + index] as u32;
}

pub fn get_sign_code8(emu: &mut Emulator, index: usize) -> i32 {
    return emu.mem[emu.eip + index] as i32;
}

pub fn get_code32(emu: &mut Emulator, index: usize) -> u32 {
    let mut ret: u32 = 0;

    // Little endian.
    for i in 0..4 {
        ret |= get_code8(emu, index + i) << (i * 8);
    }
    return ret;
}

pub fn get_sign_code32(emu: &mut Emulator, index: usize) -> i32 {
    return get_code32(emu, index) as i32;
}

pub fn get_register32(emu: &mut Emulator, index: usize) -> u32 {
    return emu.regs[index];
}

pub fn set_register32(emu: &mut Emulator, index: usize, value: u32) {
    emu.regs[index] = value;
}

pub fn set_memory8(emu: &mut Emulator, address: usize, value: u32) {
    emu.mem[address] = (value & 0xff).try_into().unwrap();
}

pub fn set_memory32(emu: &mut Emulator, address: usize, value: u32) {
    for i in 0..4 {
        set_memory8(emu, address + i, value >> (i * 8));
    }
}

pub fn get_memory8(emu: &mut Emulator, address: usize) -> u32 {
    return emu.mem[address] as u32;
}

pub fn get_memory32(emu: &mut Emulator, address: usize) -> u32 {
    let mut ret: u32 = 0;
    for i in 0..4 {
        ret |= get_memory8(emu, address + i) << (8 * i);
    }
    return ret;
}
