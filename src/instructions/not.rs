use crate::instructions::update_flags;
use crate::registers::register::Register::Count;
use std::ops::Not;

pub fn not(registers: &mut [u16; (Count as u16) as usize], instruction: u16) {
    let destination_register = (instruction >> 9) & 0x7;
    let source_register = (instruction >> 6) & 0x7;

    registers[destination_register as usize] = registers[source_register as usize].not();

    update_flags(registers, destination_register)
}

#[cfg(test)]
mod tests {
    use crate::instructions::not::not;
    use crate::registers::register::Register;
    use crate::Vm;

    // ========== Basic NOT Operations ==========

    #[test]
    fn test_not_all_ones() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0xFFFF);

        // R2 = NOT R1
        not(&mut vm.registers, 0b1001_010_001_111111);

        assert_eq!(vm.registers[Register::R2 as usize], 0x0000);
    }

    #[test]
    fn test_not_all_zeros() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 0x0000);

        // R4 = NOT R3
        not(&mut vm.registers, 0b1001_100_011_111111);

        assert_eq!(vm.registers[Register::R4 as usize], 0xFFFF);
    }

    #[test]
    fn test_not_alternating_bits_1() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0b1010_1010_1010_1010);

        // R5 = NOT R0
        not(&mut vm.registers, 0b1001_101_000_111111);

        assert_eq!(vm.registers[Register::R5 as usize], 0b0101_0101_0101_0101);
    }

    #[test]
    fn test_not_alternating_bits_2() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R6, 0b0101_0101_0101_0101);

        // R7 = NOT R6
        not(&mut vm.registers, 0b1001_111_110_111111);

        assert_eq!(vm.registers[Register::R7 as usize], 0b1010_1010_1010_1010);
    }

    #[test]
    fn test_not_single_bit_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0b0000_0000_0000_0001);

        // R2 = NOT R1
        not(&mut vm.registers, 0b1001_010_001_111111);

        assert_eq!(vm.registers[Register::R2 as usize], 0b1111_1111_1111_1110);
    }

    #[test]
    fn test_not_single_bit_clear() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 0b1111_1111_1111_1110);

        // R4 = NOT R3
        not(&mut vm.registers, 0b1001_100_011_111111);

        assert_eq!(vm.registers[Register::R4 as usize], 0b0000_0000_0000_0001);
    }

    // ========== Specific Bit Patterns ==========

    #[test]
    fn test_not_high_byte_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R5, 0xFF00);

        // R6 = NOT R5
        not(&mut vm.registers, 0b1001_110_101_111111);

        assert_eq!(vm.registers[Register::R6 as usize], 0x00FF);
    }

    #[test]
    fn test_not_low_byte_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0x00FF);

        // R1 = NOT R0
        not(&mut vm.registers, 0b1001_001_000_111111);

        assert_eq!(vm.registers[Register::R1 as usize], 0xFF00);
    }

    #[test]
    fn test_not_nibbles() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R2, 0x0F0F);

        // R3 = NOT R2
        not(&mut vm.registers, 0b1001_011_010_111111);

        assert_eq!(vm.registers[Register::R3 as usize], 0xF0F0);
    }

    #[test]
    fn test_not_checkerboard_pattern() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R4, 0b1100_1100_1100_1100);

        // R5 = NOT R4
        not(&mut vm.registers, 0b1001_101_100_111111);

        assert_eq!(vm.registers[Register::R5 as usize], 0b0011_0011_0011_0011);
    }

    // ========== Double NOT (Identity) ==========

    #[test]
    fn test_double_not_returns_original() {
        let mut vm = Vm::new();
        let original_value = 0xABCD;
        vm.write_to_register(Register::R0, original_value);

        // R1 = NOT R0
        not(&mut vm.registers, 0b1001_001_000_111111);

        // R2 = NOT R1
        not(&mut vm.registers, 0b1001_010_001_111111);

        assert_eq!(vm.registers[Register::R2 as usize], original_value);
    }

    #[test]
    fn test_not_in_place() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 0x1234);

        // R3 = NOT R3 (overwrite source)
        not(&mut vm.registers, 0b1001_011_011_111111);

        assert_eq!(vm.registers[Register::R3 as usize], 0xEDCB);
    }

    #[test]
    fn test_double_not_in_place() {
        let mut vm = Vm::new();
        let original_value = 0x5678;
        vm.write_to_register(Register::R4, original_value);

        // R4 = NOT R4
        not(&mut vm.registers, 0b1001_100_100_111111);

        // R4 = NOT R4 (should restore original)
        not(&mut vm.registers, 0b1001_100_100_111111);

        assert_eq!(vm.registers[Register::R4 as usize], original_value);
    }

    // ========== All Registers ==========

    #[test]
    fn test_not_to_r0() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x1111);

        // R0 = NOT R1
        not(&mut vm.registers, 0b1001_000_001_111111);

        assert_eq!(vm.registers[Register::R0 as usize], 0xEEEE);
    }

    #[test]
    fn test_not_from_r0() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0x2222);

        // R1 = NOT R0
        not(&mut vm.registers, 0b1001_001_000_111111);

        assert_eq!(vm.registers[Register::R1 as usize], 0xDDDD);
    }

    #[test]
    fn test_not_to_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R6, 0x3333);

        // R7 = NOT R6
        not(&mut vm.registers, 0b1001_111_110_111111);

        assert_eq!(vm.registers[Register::R7 as usize], 0xCCCC);
    }

    #[test]
    fn test_not_from_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R7, 0x4444);

        // R6 = NOT R7
        not(&mut vm.registers, 0b1001_110_111_111111);

        assert_eq!(vm.registers[Register::R6 as usize], 0xBBBB);
    }

    #[test]
    fn test_not_all_registers_to_self() {
        let mut vm = Vm::new();

        for i in 0..8 {
            let register = match i {
                0 => Register::R0,
                1 => Register::R1,
                2 => Register::R2,
                3 => Register::R3,
                4 => Register::R4,
                5 => Register::R5,
                6 => Register::R6,
                7 => Register::R7,
                _ => unreachable!(),
            };

            vm.write_to_register(register, 0xAAAA);

            // Ri = NOT Ri
            let instruction = 0b1001_000_000_111111 | (i << 9) | (i << 6);
            not(&mut vm.registers, instruction);

            assert_eq!(vm.registers[i as usize], 0x5555);
        }
    }

    // ========== Hex Value Tests ==========

    #[test]
    fn test_not_hex_value_1() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0x0000);

        // R1 = NOT R0
        not(&mut vm.registers, 0b1001_001_000_111111);

        assert_eq!(vm.registers[Register::R1 as usize], 0xFFFF);
    }

    #[test]
    fn test_not_hex_value_2() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R2, 0x1234);

        // R3 = NOT R2
        not(&mut vm.registers, 0b1001_011_010_111111);

        assert_eq!(vm.registers[Register::R3 as usize], 0xEDCB);
    }

    #[test]
    fn test_not_hex_value_3() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R4, 0xABCD);

        // R5 = NOT R4
        not(&mut vm.registers, 0b1001_101_100_111111);

        assert_eq!(vm.registers[Register::R5 as usize], 0x5432);
    }

    #[test]
    fn test_not_hex_value_4() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R6, 0xF0F0);

        // R7 = NOT R6
        not(&mut vm.registers, 0b1001_111_110_111111);

        assert_eq!(vm.registers[Register::R7 as usize], 0x0F0F);
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_not_overwrite_existing_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x9999);
        vm.write_to_register(Register::R2, 0x7777);

        // R1 = NOT R2 (overwrite R1's previous value)
        not(&mut vm.registers, 0b1001_001_010_111111);

        assert_eq!(vm.registers[Register::R1 as usize], 0x8888);
    }

    #[test]
    fn test_not_preserves_other_registers() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0x1111);
        vm.write_to_register(Register::R1, 0x2222);
        vm.write_to_register(Register::R2, 0x3333);

        // R3 = NOT R1
        not(&mut vm.registers, 0b1001_011_001_111111);

        // Check that R0 and R2 are unchanged
        assert_eq!(vm.registers[Register::R0 as usize], 0x1111);
        assert_eq!(vm.registers[Register::R2 as usize], 0x3333);
        assert_eq!(vm.registers[Register::R3 as usize], 0xDDDD);
    }

    // ========== Signed Value Tests ==========

    #[test]
    fn test_not_positive_to_negative() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0x7FFF); // Max positive (32767)

        // R1 = NOT R0
        not(&mut vm.registers, 0b1001_001_000_111111);

        assert_eq!(vm.registers[Register::R1 as usize], 0x8000); // Min negative (-32768)
    }

    #[test]
    fn test_not_negative_to_positive() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R2, 0x8000); // Min negative (-32768)

        // R3 = NOT R2
        not(&mut vm.registers, 0b1001_011_010_111111);

        assert_eq!(vm.registers[Register::R3 as usize], 0x7FFF); // Max positive (32767)
    }

    #[test]
    fn test_not_negative_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R4, 0xFFFF); // -1 in two's complement

        // R5 = NOT R4
        not(&mut vm.registers, 0b1001_101_100_111111);

        assert_eq!(vm.registers[Register::R5 as usize], 0x0000); // 0
    }

    // ========== Sequential Operations ==========

    #[test]
    fn test_not_chain() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0x00FF);

        // R1 = NOT R0
        not(&mut vm.registers, 0b1001_001_000_111111);
        assert_eq!(vm.registers[Register::R1 as usize], 0xFF00);

        // R2 = NOT R1
        not(&mut vm.registers, 0b1001_010_001_111111);
        assert_eq!(vm.registers[Register::R2 as usize], 0x00FF);

        // R3 = NOT R2
        not(&mut vm.registers, 0b1001_011_010_111111);
        assert_eq!(vm.registers[Register::R3 as usize], 0xFF00);
    }

    #[test]
    fn test_not_multiple_different_sources() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0x1111);
        vm.write_to_register(Register::R1, 0x2222);
        vm.write_to_register(Register::R2, 0x4444);

        // R3 = NOT R0
        not(&mut vm.registers, 0b1001_011_000_111111);
        assert_eq!(vm.registers[Register::R3 as usize], 0xEEEE);

        // R4 = NOT R1
        not(&mut vm.registers, 0b1001_100_001_111111);
        assert_eq!(vm.registers[Register::R4 as usize], 0xDDDD);

        // R5 = NOT R2
        not(&mut vm.registers, 0b1001_101_010_111111);
        assert_eq!(vm.registers[Register::R5 as usize], 0xBBBB);
    }
}
