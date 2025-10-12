use crate::instructions::{sign_extend, update_flags};
use crate::registers::register::Register::Count;

pub fn and(registers: &mut [u16; (Count as u16) as usize], instruction: u16) {
    let destination_register = (instruction >> 9) & 0x7;
    let source_register = (instruction >> 6) & 0x7;
    let immediate_mode_flag = (instruction >> 5) & 0x1;

    if immediate_mode_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);
        registers[destination_register as usize] = registers[source_register as usize] & imm5;
    } else {
        let address_of_second_argument = instruction & 0x7;
        registers[destination_register as usize] =
            registers[source_register as usize] & registers[address_of_second_argument as usize];
    }

    update_flags(registers, destination_register)
}

#[cfg(test)]
mod tests {
    use crate::instructions::and::and;
    use crate::registers::register::Register;
    use crate::Vm;

    // ========== Immediate Mode Tests ==========

    #[test]
    fn test_and_immediate_mode_basic() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0b1111_1111_1111_1111);

        // R2 = R1 AND 0b01010 (10 in decimal)
        and(&mut vm.registers, 0b0101_010_001_1_01010);

        assert_eq!(vm.registers[Register::R2 as usize], 0b01010);
    }

    #[test]
    fn test_and_immediate_mode_all_ones() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 0b0000_0000_0001_1111);

        // R4 = R3 AND 0b11111 (all ones in IMM5)
        and(&mut vm.registers, 0b0101_100_011_1_11111);

        assert_eq!(vm.registers[Register::R4 as usize], 0b0000_0000_0001_1111);
    }

    #[test]
    fn test_and_immediate_mode_all_zeros() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R5, 0xFFFF);

        // R6 = R5 AND 0b00000
        and(&mut vm.registers, 0b0101_110_101_1_00000);

        assert_eq!(vm.registers[Register::R6 as usize], 0);
    }

    #[test]
    fn test_and_immediate_mode_masking() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0b1010_1010_1010_1111);

        // R1 = R0 AND 0b01111 (mask lower 4 bits)
        and(&mut vm.registers, 0b0101_001_000_1_01111);

        assert_eq!(vm.registers[Register::R1 as usize], 0b1111);
    }

    #[test]
    fn test_and_immediate_mode_single_bit() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R2, 0b0000_0000_0000_0101);

        // R3 = R2 AND 0b00001 (check if bit 0 is set)
        and(&mut vm.registers, 0b0101_011_010_1_00001);

        assert_eq!(vm.registers[Register::R3 as usize], 0b00001);
    }

    #[test]
    fn test_and_immediate_mode_no_overlap() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R4, 0b1111_1111_1110_0000);

        // R5 = R4 AND 0b01111 (no overlapping bits)
        and(&mut vm.registers, 0b0101_101_100_1_01111);

        assert_eq!(vm.registers[Register::R5 as usize], 0);
    }

    #[test]
    fn test_and_immediate_mode_sign_extended_negative() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R6, 0xFFFF);

        // R7 = R6 AND 0b11111 (-1 sign-extended to 0xFFFF)
        and(&mut vm.registers, 0b0101_111_110_1_11111);

        assert_eq!(vm.registers[Register::R7 as usize], 0xFFFF);
    }

    // ========== Register Mode Tests ==========

    #[test]
    fn test_and_register_mode_basic() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0b1111_0000_1111_0000);
        vm.write_to_register(Register::R2, 0b1010_1010_1010_1010);

        // R3 = R1 AND R2
        and(&mut vm.registers, 0b0101_011_001_0_00_010);

        assert_eq!(vm.registers[Register::R3 as usize], 0b1010_0000_1010_0000);
    }

    #[test]
    fn test_and_register_mode_all_ones() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R4, 0xFFFF);
        vm.write_to_register(Register::R5, 0xABCD);

        // R6 = R4 AND R5
        and(&mut vm.registers, 0b0101_110_100_0_00_101);

        assert_eq!(vm.registers[Register::R6 as usize], 0xABCD);
    }

    #[test]
    fn test_and_register_mode_all_zeros() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0);
        vm.write_to_register(Register::R1, 0xFFFF);

        // R2 = R0 AND R1
        and(&mut vm.registers, 0b0101_010_000_0_00_001);

        assert_eq!(vm.registers[Register::R2 as usize], 0);
    }

    #[test]
    fn test_and_register_mode_same_register() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 0b1010_1010_1010_1010);

        // R4 = R3 AND R3 (should equal R3)
        and(&mut vm.registers, 0b0101_100_011_0_00_011);

        assert_eq!(vm.registers[Register::R4 as usize], 0b1010_1010_1010_1010);
    }

    #[test]
    fn test_and_register_mode_complementary() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R5, 0b1111_0000_1111_0000);
        vm.write_to_register(Register::R6, 0b0000_1111_0000_1111);

        // R7 = R5 AND R6 (should be 0)
        and(&mut vm.registers, 0b0101_111_101_0_00_110);

        assert_eq!(vm.registers[Register::R7 as usize], 0);
    }

    #[test]
    fn test_and_register_mode_identical_values() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x1234);
        vm.write_to_register(Register::R2, 0x1234);

        // R0 = R1 AND R2
        and(&mut vm.registers, 0b0101_000_001_0_00_010);

        assert_eq!(vm.registers[Register::R0 as usize], 0x1234);
    }

    #[test]
    fn test_and_register_mode_byte_masking() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 0xABCD);
        vm.write_to_register(Register::R4, 0x00FF);

        // R5 = R3 AND R4 (extract lower byte)
        and(&mut vm.registers, 0b0101_101_011_0_00_100);

        assert_eq!(vm.registers[Register::R5 as usize], 0x00CD);
    }

    #[test]
    fn test_and_register_mode_high_byte_masking() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R6, 0xABCD);
        vm.write_to_register(Register::R7, 0xFF00);

        // R0 = R6 AND R7 (extract upper byte)
        and(&mut vm.registers, 0b0101_000_110_0_00_111);

        assert_eq!(vm.registers[Register::R0 as usize], 0xAB00);
    }

    // ========== Destination Register Tests ==========

    #[test]
    fn test_and_destination_same_as_source1() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R2, 0xFF0F);
        vm.write_to_register(Register::R3, 0x0F0F);

        // R2 = R2 AND R3 (overwrite source)
        and(&mut vm.registers, 0b0101_010_010_0_00_011);

        assert_eq!(vm.registers[Register::R2 as usize], 0x0F0F);
    }

    #[test]
    fn test_and_destination_same_as_source2() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R4, 0xFF0F);
        vm.write_to_register(Register::R5, 0x0F0F);

        // R5 = R4 AND R5 (overwrite second source)
        and(&mut vm.registers, 0b0101_101_100_0_00_101);

        assert_eq!(vm.registers[Register::R5 as usize], 0x0F0F);
    }

    #[test]
    fn test_and_all_same_register() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0b1111_0000_1111_0000);

        // R1 = R1 AND R1 (idempotent operation)
        and(&mut vm.registers, 0b0101_001_001_0_00_001);

        assert_eq!(vm.registers[Register::R1 as usize], 0b1111_0000_1111_0000);
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_and_to_r0() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x1234);

        // R0 = R1 AND 0b11111
        and(&mut vm.registers, 0b0101_000_001_1_11111);

        assert_eq!(vm.registers[Register::R0 as usize], 0x1234);
    }

    #[test]
    fn test_and_to_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R6, 0xABCD);

        // R7 = R6 AND 0b11111
        and(&mut vm.registers, 0b0101_111_110_1_11111);

        assert_eq!(vm.registers[Register::R7 as usize], 0xABCD);
    }

    #[test]
    fn test_and_immediate_overwrite_previous() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R2, 0x9999);
        vm.write_to_register(Register::R3, 0xFF);

        // R2 = R3 AND 0b01111
        and(&mut vm.registers, 0b0101_010_011_1_01111);

        assert_eq!(vm.registers[Register::R2 as usize], 0x0F);
    }

    // ========== Bit Manipulation Tests ==========

    #[test]
    fn test_and_clear_bit() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0b1111);
        vm.write_to_register(Register::R2, 0b1110); // Clear bit 0

        // R3 = R1 AND R2
        and(&mut vm.registers, 0b0101_011_001_0_00_010);

        assert_eq!(vm.registers[Register::R3 as usize], 0b1110);
    }

    #[test]
    fn test_and_extract_nibble() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R4, 0xABCD);

        // R5 = R4 AND 0b01111 (extract lowest nibble)
        and(&mut vm.registers, 0b0101_101_100_1_01111);

        assert_eq!(vm.registers[Register::R5 as usize], 0x000D);
    }

    #[test]
    fn test_and_check_even_odd() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 42); // Even number

        // R1 = R0 AND 1 (check if odd)
        and(&mut vm.registers, 0b0101_001_000_1_00001);

        assert_eq!(vm.registers[Register::R1 as usize], 0); // Even, so result is 0
    }

    #[test]
    fn test_and_check_odd() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R2, 43); // Odd number

        // R3 = R2 AND 1 (check if odd)
        and(&mut vm.registers, 0b0101_011_010_1_00001);

        assert_eq!(vm.registers[Register::R3 as usize], 1); // Odd, so result is 1
    }

    // ========== Complex Patterns ==========

    #[test]
    fn test_and_alternating_bits() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R4, 0b1010_1010_1010_1010);
        vm.write_to_register(Register::R5, 0b0101_0101_0101_0101);

        // R6 = R4 AND R5 (should be 0)
        and(&mut vm.registers, 0b0101_110_100_0_00_101);

        assert_eq!(vm.registers[Register::R6 as usize], 0);
    }

    #[test]
    fn test_and_checkerboard_pattern() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0b1111_1111_1111_1111);
        vm.write_to_register(Register::R1, 0b1010_1010_1010_1010);

        // R2 = R0 AND R1
        and(&mut vm.registers, 0b0101_010_000_0_00_001);

        assert_eq!(vm.registers[Register::R2 as usize], 0b1010_1010_1010_1010);
    }

    #[test]
    fn test_and_sequential_operations() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0xFFFF);

        // R1 = R0 AND 0xFF00
        vm.write_to_register(Register::R2, 0xFF00);
        and(&mut vm.registers, 0b0101_001_000_0_00_010);
        assert_eq!(vm.registers[Register::R1 as usize], 0xFF00);

        // R3 = R1 AND 0x0F00
        vm.write_to_register(Register::R4, 0x0F00);
        and(&mut vm.registers, 0b0101_011_001_0_00_100);
        assert_eq!(vm.registers[Register::R3 as usize], 0x0F00);
    }
}
