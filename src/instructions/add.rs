use crate::instructions::sign_extend;
use crate::registers::register::Register::Count;
use crate::update_flags;

pub fn add(mut registers: [u16; (Count as u16) as usize], instruction: u16) {
    let destination_register = (instruction >> 9) & 0x7;
    let first_argument = (instruction >> 6) & 0x7;
    let immediate_mode_flag = (instruction >> 5) & 0x1;

    if immediate_mode_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);
        registers[destination_register as usize] = first_argument + imm5;
    } else {
        let address_of_second_argument = instruction & 0x7;
        registers[destination_register as usize] =
            first_argument + registers[address_of_second_argument as usize];
    }

    update_flags(registers, destination_register)
}
