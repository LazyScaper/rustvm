use crate::instructions::{sign_extend, update_flags};
use crate::registers::register::Register::Count;

pub fn add(registers: &mut [u16; (Count as u16) as usize], instruction: u16) {
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

#[cfg(test)]
mod tests {
    use crate::instructions::add::add;
    use crate::registers::register::Register;
    use crate::Vm;

    #[test]
    fn test_add_immediate_mode() {
        let mut vm = Vm::new();

        // Add 4 to the value 5 and place it in R2
        add(&mut vm.registers, 0b0001_010_100_1_00101);

        assert_eq!(vm.registers[Register::R2 as usize], 9);
    }
}
