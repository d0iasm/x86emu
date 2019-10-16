use std::process;

use crate::*;

pub struct ModRM {
    pub modval: u8,
    pub opecode: u8,
    pub reg_index: u8,
    pub rm: u8,
    pub sib: u8,
    pub disp8: i8,
    pub disp32: u32,
}

impl Default for ModRM {
    fn default () -> ModRM {
        ModRM {
            modval: 0,
            opecode: 0,
            reg_index: 0,
            rm: 0,
            sib: 0,
            disp8: 0,
            disp32: 0,
        }
    }
}

// Set up ModR/M, SIB, and displacement based on the emu argument.
// The emu argument is the Emulator struct whose eip should be the position of ModR/M byte.
// It moves to the position of a next instruction after an execution is done.
pub fn parse_modrm(emu: &mut Emulator) -> ModRM {
    let code = get_code8(emu, 0);
    let mut modrm = ModRM::default();
    modrm.modval = ((code & 0xc0) >> 6).try_into().unwrap();
    modrm.opecode = ((code & 0x38) >> 3).try_into().unwrap();
    modrm.rm = (code & 0x07).try_into().unwrap();

    emu.eip += 1;

    if modrm.modval != 3 && modrm.rm == 4 {
        modrm.sib = get_code8(emu, 0).try_into().unwrap();
        emu.eip += 1;
    } else if modrm.modval == 1 {
        modrm.disp8 = get_sign_code8(emu, 0).try_into().unwrap();
        emu.eip += 1;
    }
    return modrm;
}

// Calculate an effective address based on ModR/M. modrm.mod should be 0, 1, or 2.
pub fn calc_memory_address(emu: &mut Emulator, modrm: &ModRM) -> u32 {
    if modrm.modval == 0 {
        if modrm.rm == 4 {
            println!("not implemented ModRM mod = 0, rm = 4");
            process::exit(1);
        } else if modrm.rm == 5 {
            return modrm.disp32;
        }
        return get_register32(emu, modrm.rm.try_into().unwrap());
    } else if modrm.modval == 1 {
        if modrm.rm == 4 {
            println!("not implemented ModRM mod = 1, rm = 4");
            process::exit(1);
        }
        return (get_register32(emu, modrm.rm.try_into().unwrap()) as i32 + modrm.disp8 as i32) as u32;
    } else if modrm.modval == 2 {
        if modrm.rm == 4 {
            println!("not implemented ModRM mod = 2, rm = 4");
            process::exit(1);
        }
        return get_register32(emu, modrm.rm.try_into().unwrap()) + modrm.disp32;
    }
    println!("not implemented ModRM mod = 3");
    process::exit(1);
}

// Get rm32 register or 32-bit data from a memory.
pub fn get_rm32(emu: &mut Emulator, modrm: &ModRM) -> u32 {
    if modrm.modval == 3 {
        return get_register32(emu, modrm.rm.try_into().unwrap());
    } else {
        let address = calc_memory_address(emu, modrm);
        return get_memory32(emu, address.try_into().unwrap());
    }
}

// Write value into rm32 register or 32-bit data of a memory based on modrm.
pub fn set_rm32(emu: &mut Emulator, modrm: &ModRM, value: u32) {
    if modrm.modval == 3 {
        set_register32(emu, modrm.rm.try_into().unwrap(), value);
    } else {
        let address = calc_memory_address(emu, modrm);
        set_memory32(emu, address.try_into().unwrap(), value);
    }
}

// Get r32 register.
pub fn get_r32(emu: &mut Emulator, modrm: &ModRM) -> u32 {
    return get_register32(emu, modrm.reg_index.try_into().unwrap());
}

// Write value into r32 register.
pub fn set_r32(emu: &mut Emulator, modrm: &ModRM, value: u32) {
    set_register32(emu, modrm.reg_index.try_into().unwrap(), value);
}
