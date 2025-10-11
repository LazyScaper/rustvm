use crate::instructions::{sign_extend, update_flags};
use crate::registers::register::Register::Count;

pub fn add(registers: &mut [u16; (Count as u16) as usize], instruction: u16) {
    let destination_register = (instruction >> 9) & 0x7;
    let source_register = (instruction >> 6) & 0x7;
    let immediate_mode_flag = (instruction >> 5) & 0x1;

    if immediate_mode_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);
        registers[destination_register as usize] =
            registers[source_register as usize].wrapping_add(imm5);
    } else {
        let address_of_second_argument = instruction & 0x7;
        registers[destination_register as usize] = registers[source_register as usize]
            .wrapping_add(registers[address_of_second_argument as usize]);
    }

    update_flags(registers, destination_register)
}

#[cfg(test)]
mod tests {
    use crate::instructions::add::add;
    use crate::registers::register::Register;
    use crate::Vm;

    // ========== Immediate Mode Tests ==========

    #[test]
    fn test_add_immediate_mode_positive() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R4, 10);

        // R2 = R4 + 5
        add(&mut vm.registers, 0b0001_010_100_1_00101);

        assert_eq!(vm.registers[Register::R2 as usize], 15);
    }

    #[test]
    fn test_add_immediate_mode_zero() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 42);

        // R2 = R3 + 0
        add(&mut vm.registers, 0b0001_010_011_1_00000);

        assert_eq!(vm.registers[Register::R2 as usize], 42);
    }

    #[test]
    fn test_add_immediate_mode_max_positive() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 10);

        // R2 = R3 + 15 (max positive IMM5)
        add(&mut vm.registers, 0b0001_010_011_1_01111);

        assert_eq!(vm.registers[Register::R2 as usize], 25);
    }

    #[test]
    fn test_add_immediate_mode_negative() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 10);

        // R2 = R3 + (-1)
        add(&mut vm.registers, 0b0001_010_011_1_11111);

        assert_eq!(vm.registers[Register::R2 as usize], 9);
    }

    #[test]
    fn test_add_immediate_mode_max_negative() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 20);

        // R2 = R3 + (-16) (max negative IMM5)
        add(&mut vm.registers, 0b0001_010_011_1_10000);

        assert_eq!(vm.registers[Register::R2 as usize], 4);
    }

    #[test]
    fn test_add_immediate_mode_result_zero() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R5, 5);

        // R1 = R5 + (-5)
        add(&mut vm.registers, 0b0001_001_101_1_11011);

        assert_eq!(vm.registers[Register::R1 as usize], 0);
    }

    #[test]
    fn test_add_immediate_mode_overflow() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 65530);

        // R7 = R0 + 10 (should wrap around)
        add(&mut vm.registers, 0b0001_111_000_1_01010);

        assert_eq!(vm.registers[Register::R7 as usize], 4);
    }

    #[test]
    fn test_add_immediate_mode_negative_overflow() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R2, 5);

        // R3 = R2 + (-10)
        add(&mut vm.registers, 0b0001_011_010_1_10110);

        assert_eq!(vm.registers[Register::R3 as usize], 65531); // wraps to -5 in two's complement
    }

    // ========== Register Mode Tests ==========

    #[test]
    fn test_add_register_mode_positive() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 10);
        vm.write_to_register(Register::R2, 20);

        // R0 = R1 + R2
        add(&mut vm.registers, 0b0001_000_001_0_00_010);

        assert_eq!(vm.registers[Register::R0 as usize], 30);
    }

    #[test]
    fn test_add_register_mode_with_zero() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 42);
        vm.write_to_register(Register::R4, 0);

        // R5 = R3 + R4
        add(&mut vm.registers, 0b0001_101_011_0_00_100);

        assert_eq!(vm.registers[Register::R5 as usize], 42);
    }

    #[test]
    fn test_add_register_mode_same_register() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R2, 15);

        // R2 = R2 + R2 (double the value)
        add(&mut vm.registers, 0b0001_010_010_0_00_010);

        assert_eq!(vm.registers[Register::R2 as usize], 30);
    }

    #[test]
    fn test_add_register_mode_three_different_registers() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R4, 100);
        vm.write_to_register(Register::R6, 200);

        // R7 = R4 + R6
        add(&mut vm.registers, 0b0001_111_100_0_00_110);

        assert_eq!(vm.registers[Register::R7 as usize], 300);
    }

    #[test]
    fn test_add_register_mode_overflow() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 40000);
        vm.write_to_register(Register::R1, 30000);

        // R2 = R0 + R1 (should wrap)
        add(&mut vm.registers, 0b0001_010_000_0_00_001);

        assert_eq!(vm.registers[Register::R2 as usize], 4464); // 70000 - 65536
    }

    #[test]
    fn test_add_register_mode_result_zero() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 0);
        vm.write_to_register(Register::R5, 0);

        // R1 = R3 + R5
        add(&mut vm.registers, 0b0001_001_011_0_00_101);

        assert_eq!(vm.registers[Register::R1 as usize], 0);
    }

    #[test]
    fn test_add_register_mode_max_values() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R6, 65535);
        vm.write_to_register(Register::R7, 65535);

        // R0 = R6 + R7
        add(&mut vm.registers, 0b0001_000_110_0_00_111);

        assert_eq!(vm.registers[Register::R0 as usize], 65534); // wraps around
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_add_to_r0() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 5);

        // R0 = R1 + 3
        add(&mut vm.registers, 0b0001_000_001_1_00011);

        assert_eq!(vm.registers[Register::R0 as usize], 8);
    }

    #[test]
    fn test_add_from_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R7, 99);

        // R3 = R7 + 1
        add(&mut vm.registers, 0b0001_011_111_1_00001);

        assert_eq!(vm.registers[Register::R3 as usize], 100);
    }

    #[test]
    fn test_add_all_zeros() {
        let mut vm = Vm::new();

        // R0 = R0 + 0 (all registers start at 0)
        add(&mut vm.registers, 0b0001_000_000_1_00000);

        assert_eq!(vm.registers[Register::R0 as usize], 0);
    }

    #[test]
    fn test_add_destination_same_as_source() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R4, 10);

        // R4 = R4 + 5 (increment in place)
        add(&mut vm.registers, 0b0001_100_100_1_00101);

        assert_eq!(vm.registers[Register::R4 as usize], 15);
    }

    #[test]
    fn test_add_register_mode_source_and_dest_overlap() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R2, 10);
        vm.write_to_register(Register::R5, 7);

        // R2 = R2 + R5 (dest is also first source)
        add(&mut vm.registers, 0b0001_010_010_0_00_101);

        assert_eq!(vm.registers[Register::R2 as usize], 17);
    }
}
