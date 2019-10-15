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

