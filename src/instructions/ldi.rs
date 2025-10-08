use crate::instructions::sign_extend;
use crate::registers::register::Register::{Count, Pc};
use crate::update_flags;

pub fn load_indirect(mut registers: [u16; (Count as u16) as usize], instruction: u16) {
    let destination_register = (instruction >> 9) & 0x7;
    let pc_offset_9 = sign_extend(instruction & 0x1FF, 9);

    registers[destination_register as usize] = registers[(Pc as u16) as usize] + pc_offset_9;
    update_flags(registers, destination_register)
}
