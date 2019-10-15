use crate::*;
use function::*;

// MOV r32, imm32: Move imm32 to r32.
pub fn mov_r32_imm32(emu: &mut Emulator) {
    let reg: usize = (get_code8(emu, 0) - 0xB8).try_into().unwrap();
    let value = get_code32(emu, 1);
    emu.regs[reg] = value;
    emu.eip += 5;
}

// JMP rel8: Jump short, relative, displacement relative to next instruction.
pub fn short_jump(emu: &mut Emulator) {
    let diff = get_sign_code8(emu, 1) as usize;
    emu.eip += diff + 2;
}

// JMP rel32: Jump near, relative, RIP = RIP + 32-bit displacement sign extended to 64-bits.
pub fn near_jump(emu: &mut Emulator) {
    let diff = get_sign_code32(emu, 1) as usize;
    emu.eip += diff + 5;
}

pub fn nop(_emu: &mut Emulator) {
}

pub fn init_instructions(instructions: &mut Insts) {
	for i in 0..8 {
        instructions[0xB8 + i] = mov_r32_imm32;
	}
    instructions[0xE9] = near_jump;
    instructions[0xEB] = short_jump;
}
