use std::convert::TryInto;

use crate::*;

const CARRY_FLAG: u32 = 1;
const ZERO_FLAG: u32 = 1 << 6;
const SIGN_FLAG: u32 = 1 << 7;
const OVERFLOW_FLAG: u32 = 1 << 11;

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

pub fn push32(emu: &mut Emulator, value: u32) {
    let address = get_register32(emu, ESP) - 4;
    set_register32(emu, ESP, address.try_into().unwrap());
    set_memory32(emu, address.try_into().unwrap(), value);
}

pub fn pop32(emu: &mut Emulator) -> u32 {
    let address = get_register32(emu, ESP);
    let ret = get_memory32(emu, address.try_into().unwrap());
    set_register32(emu, ESP, (address + 4).try_into().unwrap());
    return ret;
}

pub fn set_carry(emu: &mut Emulator, is_carry: bool) {
    if is_carry {
        emu.eflags |= CARRY_FLAG;
    } else {
        emu.eflags &= !CARRY_FLAG;
    }
}

pub fn set_zero(emu: &mut Emulator, is_zero: bool) {
    if is_zero {
        emu.eflags |= ZERO_FLAG;
    } else {
        emu.eflags &= !ZERO_FLAG;
    }
}

pub fn set_sign(emu: &mut Emulator, is_sign: bool) {
    if is_sign {
        emu.eflags |= SIGN_FLAG;
    } else {
        emu.eflags &= !SIGN_FLAG;
    }
}

pub fn set_overflow(emu: &mut Emulator, is_overflow: bool) {
    if is_overflow {
        emu.eflags |= OVERFLOW_FLAG;
    } else {
        emu.eflags &= !OVERFLOW_FLAG;
    }
}

pub fn is_carry(emu: &mut Emulator) -> bool {
    return (emu.eflags & CARRY_FLAG) != 0;
}

pub fn is_zero(emu: &mut Emulator) -> bool {
    return (emu.eflags & ZERO_FLAG) != 0;
}

pub fn is_sign(emu: &mut Emulator) -> bool {
    return (emu.eflags & SIGN_FLAG) != 0;
}

pub fn is_overflow(emu: &mut Emulator) -> bool {
    return (emu.eflags & OVERFLOW_FLAG) != 0;
}

pub fn update_eflags_sub(emu: &mut Emulator, v1: u32, v2: u32, result: u64) {
    // Get a sign.
    let sign1 = v1 >> 31;
    let sign2 = v2 >> 31;
    let signr = (result >> 31) & 1;

    // Set a carry flag if the result is greater than 0.
    set_carry(emu, (result >> 32) != 0);

    // Set a zero flag if the result is 0.
    set_zero(emu, result == 0);

    // Set a sign flag if the result has a sign.
    set_sign(emu, signr != 0);

    // Set a overflow flag is the result overflows.
    set_overflow(emu, sign1 != sign2 && sign1 as u64 != signr);
}
