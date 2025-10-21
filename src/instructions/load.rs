use crate::instructions::{sign_extend, update_flags};
use crate::registers::register::Register::{Count, Pc};
use crate::MEMORY_MAX;

pub fn ld(
    registers: &mut [u16; (Count as u16) as usize],
    memory: [u16; MEMORY_MAX],
    instruction: u16,
) {
    let destination_register = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    registers[destination_register as usize] =
        memory[registers[Pc as usize].wrapping_add(pc_offset) as usize];

    update_flags(registers, destination_register)
}

#[cfg(test)]
mod tests {
    use crate::instructions::load::ld;
    use crate::registers::register::Register;
    use crate::Vm;

    // ========== Basic LD Operations ==========

    #[test]
    fn test_ld_basic_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3005, 42);

        // LD R2, 5 (load from PC + 5)
        ld(&mut vm.registers, vm.memory, 0b0010_010_000000101);

        assert_eq!(vm.registers[Register::R2 as usize], 42);
    }

    #[test]
    fn test_ld_zero_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3000, 123);

        // LD R3, 0 (load from PC + 0)
        ld(&mut vm.registers, vm.memory, 0b0010_011_000000000);

        assert_eq!(vm.registers[Register::R3 as usize], 123);
    }

    #[test]
    fn test_ld_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3010);
        vm.write_to_memory(0x3008, 99);

        // LD R1, -8 (0x1F8 in 9-bit two's complement)
        ld(&mut vm.registers, vm.memory, 0b0010_001_111111000);

        assert_eq!(vm.registers[Register::R1 as usize], 99);
    }

    #[test]
    fn test_ld_max_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x30FF, 255);

        // LD R4, 255 (max positive 9-bit offset)
        ld(&mut vm.registers, vm.memory, 0b0010_100_011111111);

        assert_eq!(vm.registers[Register::R4 as usize], 255);
    }

    #[test]
    fn test_ld_max_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3100);
        vm.write_to_memory(0x3000, 77);

        // LD R5, -256 (max negative 9-bit offset)
        ld(&mut vm.registers, vm.memory, 0b0010_101_100000000);

        assert_eq!(vm.registers[Register::R5 as usize], 77);
    }

    // ========== Load to Different Registers ==========

    #[test]
    fn test_ld_to_r0() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3001, 111);

        // LD R0, 1
        ld(&mut vm.registers, vm.memory, 0b0010_000_000000001);

        assert_eq!(vm.registers[Register::R0 as usize], 111);
    }

    #[test]
    fn test_ld_to_r1() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3002, 222);

        // LD R1, 2
        ld(&mut vm.registers, vm.memory, 0b0010_001_000000010);

        assert_eq!(vm.registers[Register::R1 as usize], 222);
    }

    #[test]
    fn test_ld_to_r2() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3003, 333);

        // LD R2, 3
        ld(&mut vm.registers, vm.memory, 0b0010_010_000000011);

        assert_eq!(vm.registers[Register::R2 as usize], 333);
    }

    #[test]
    fn test_ld_to_r3() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3004, 444);

        // LD R3, 4
        ld(&mut vm.registers, vm.memory, 0b0010_011_000000100);

        assert_eq!(vm.registers[Register::R3 as usize], 444);
    }

    #[test]
    fn test_ld_to_r4() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3005, 555);

        // LD R4, 5
        ld(&mut vm.registers, vm.memory, 0b0010_100_000000101);

        assert_eq!(vm.registers[Register::R4 as usize], 555);
    }

    #[test]
    fn test_ld_to_r5() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3006, 666);

        // LD R5, 6
        ld(&mut vm.registers, vm.memory, 0b0010_101_000000110);

        assert_eq!(vm.registers[Register::R5 as usize], 666);
    }

    #[test]
    fn test_ld_to_r6() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3007, 777);

        // LD R6, 7
        ld(&mut vm.registers, vm.memory, 0b0010_110_000000111);

        assert_eq!(vm.registers[Register::R6 as usize], 777);
    }

    #[test]
    fn test_ld_to_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3008, 888);

        // LD R7, 8
        ld(&mut vm.registers, vm.memory, 0b0010_111_000001000);

        assert_eq!(vm.registers[Register::R7 as usize], 888);
    }

    // ========== Different Value Types ==========

    #[test]
    fn test_ld_zero_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3001, 0);

        // LD R2, 1
        ld(&mut vm.registers, vm.memory, 0b0010_010_000000001);

        assert_eq!(vm.registers[Register::R2 as usize], 0);
    }

    #[test]
    fn test_ld_max_positive_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3001, 0x7FFF); // Max positive 16-bit signed

        // LD R3, 1
        ld(&mut vm.registers, vm.memory, 0b0010_011_000000001);

        assert_eq!(vm.registers[Register::R3 as usize], 0x7FFF);
    }

    #[test]
    fn test_ld_negative_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3001, 0xFFFF); // -1 in two's complement

        // LD R4, 1
        ld(&mut vm.registers, vm.memory, 0b0010_100_000000001);

        assert_eq!(vm.registers[Register::R4 as usize], 0xFFFF);
    }

    #[test]
    fn test_ld_max_unsigned_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3001, 0xFFFF);

        // LD R5, 1
        ld(&mut vm.registers, vm.memory, 0b0010_101_000000001);

        assert_eq!(vm.registers[Register::R5 as usize], 0xFFFF);
    }

    #[test]
    fn test_ld_various_hex_values() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        let test_values = [0x1234, 0xABCD, 0x5678, 0xDEAD, 0xBEEF, 0xCAFE];

        for (i, &value) in test_values.iter().enumerate() {
            vm.write_to_memory(0x3000 + i as u16, value);

            let instruction = 0b0010_000_000000000 | (i as u16);
            ld(&mut vm.registers, vm.memory, instruction);

            assert_eq!(vm.registers[Register::R0 as usize], value);
        }
    }

    // ========== PC-Relative Tests ==========

    #[test]
    fn test_ld_from_different_pc_values() {
        let pc_values = [0x0100, 0x1000, 0x3000, 0x5000, 0x7000, 0xA000];

        for &pc in &pc_values {
            let mut vm = Vm::new();
            vm.write_to_register(Register::Pc, pc);
            vm.write_to_memory(pc + 10, 42);

            // LD R1, 10
            ld(&mut vm.registers, vm.memory, 0b0010_001_000001010);

            assert_eq!(vm.registers[Register::R1 as usize], 42);
        }
    }

    #[test]
    fn test_ld_from_low_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x0010);
        vm.write_to_memory(0x0015, 99);

        // LD R2, 5
        ld(&mut vm.registers, vm.memory, 0b0010_010_000000101);

        assert_eq!(vm.registers[Register::R2 as usize], 99);
    }

    #[test]
    fn test_ld_from_high_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0xFE00);
        vm.write_to_memory(0xFE10, 88);

        // LD R3, 16
        ld(&mut vm.registers, vm.memory, 0b0010_011_000010000);

        assert_eq!(vm.registers[Register::R3 as usize], 88);
    }

    // ========== Overwriting Previous Values ==========

    #[test]
    fn test_ld_overwrites_existing_register_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 9999); // Pre-existing value
        vm.write_to_memory(0x3005, 42);

        // LD R2, 5
        ld(&mut vm.registers, vm.memory, 0b0010_010_000000101);

        assert_eq!(vm.registers[Register::R2 as usize], 42);
    }

    #[test]
    fn test_ld_preserves_other_registers() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x1111);
        vm.write_to_register(Register::R1, 0x2222);
        vm.write_to_register(Register::R3, 0x3333);
        vm.write_to_memory(0x3005, 42);

        // LD R2, 5
        ld(&mut vm.registers, vm.memory, 0b0010_010_000000101);

        // Check other registers unchanged
        assert_eq!(vm.registers[Register::R0 as usize], 0x1111);
        assert_eq!(vm.registers[Register::R1 as usize], 0x2222);
        assert_eq!(vm.registers[Register::R3 as usize], 0x3333);
        assert_eq!(vm.registers[Register::R2 as usize], 42);
    }

    // ========== Sequential Loads ==========

    #[test]
    fn test_ld_multiple_values_sequentially() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3001, 10);
        vm.write_to_memory(0x3002, 20);
        vm.write_to_memory(0x3003, 30);

        // LD R1, 1
        ld(&mut vm.registers, vm.memory, 0b0010_001_000000001);
        assert_eq!(vm.registers[Register::R1 as usize], 10);

        // LD R2, 2
        ld(&mut vm.registers, vm.memory, 0b0010_010_000000010);
        assert_eq!(vm.registers[Register::R2 as usize], 20);

        // LD R3, 3
        ld(&mut vm.registers, vm.memory, 0b0010_011_000000011);
        assert_eq!(vm.registers[Register::R3 as usize], 30);
    }

    #[test]
    fn test_ld_same_location_multiple_times() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3005, 42);

        // LD R1, 5
        ld(&mut vm.registers, vm.memory, 0b0010_001_000000101);
        assert_eq!(vm.registers[Register::R1 as usize], 42);

        // LD R2, 5 (same location)
        ld(&mut vm.registers, vm.memory, 0b0010_010_000000101);
        assert_eq!(vm.registers[Register::R2 as usize], 42);

        // LD R3, 5 (same location again)
        ld(&mut vm.registers, vm.memory, 0b0010_011_000000101);
        assert_eq!(vm.registers[Register::R3 as usize], 42);
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_ld_offset_minus_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3001);
        vm.write_to_memory(0x3000, 55);

        // LD R4, -1 (0x1FF in 9-bit two's complement)
        ld(&mut vm.registers, vm.memory, 0b0010_100_111111111);

        assert_eq!(vm.registers[Register::R4 as usize], 55);
    }

    #[test]
    fn test_ld_offset_plus_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3001, 66);

        // LD R5, 1
        ld(&mut vm.registers, vm.memory, 0b0010_101_000000001);

        assert_eq!(vm.registers[Register::R5 as usize], 66);
    }

    #[test]
    fn test_ld_backward_from_current_pc() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3050);
        vm.write_to_memory(0x3000, 123);

        // LD R6, -80 (0x1B0 in 9-bit two's complement)
        ld(&mut vm.registers, vm.memory, 0b0010_110_110110000);

        assert_eq!(vm.registers[Register::R6 as usize], 123);
    }

    // ========== Loading from Data Section ==========

    #[test]
    fn test_ld_from_data_table() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // Simulate data table at 0x3100
        vm.write_to_memory(0x3100, 100);
        vm.write_to_memory(0x3101, 200);
        vm.write_to_memory(0x3102, 300);

        // Calculate offset to 0x3100 from 0x3000
        let offset = 0x100; // 256 in decimal

        // But this exceeds 9-bit signed range (-256 to 255)
        // So we need to use a closer PC value
        vm.write_to_register(Register::Pc, 0x30F0);

        // LD R0, 16 (0x30F0 + 16 = 0x3100)
        ld(&mut vm.registers, vm.memory, 0b0010_000_000010000);
        assert_eq!(vm.registers[Register::R0 as usize], 100);
    }

    #[test]
    fn test_ld_ascii_values() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // Store ASCII values
        vm.write_to_memory(0x3001, 0x0041); // 'A'
        vm.write_to_memory(0x3002, 0x0042); // 'B'
        vm.write_to_memory(0x3003, 0x0043); // 'C'

        // LD R1, 1
        ld(&mut vm.registers, vm.memory, 0b0010_001_000000001);
        assert_eq!(vm.registers[Register::R1 as usize], 0x0041);

        // LD R2, 2
        ld(&mut vm.registers, vm.memory, 0b0010_010_000000010);
        assert_eq!(vm.registers[Register::R2 as usize], 0x0042);
    }

    // ========== Boundary Conditions ==========

    #[test]
    fn test_ld_at_memory_boundary() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0xFFF0);
        vm.write_to_memory(0xFFFF, 0xDEAD);

        // LD R7, 15
        ld(&mut vm.registers, vm.memory, 0b0010_111_000001111);

        assert_eq!(vm.registers[Register::R7 as usize], 0xDEAD);
    }

    #[test]
    fn test_ld_from_address_zero() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x0005);
        vm.write_to_memory(0x0000, 0xBEEF);

        // LD R0, -5 (0x1FB in 9-bit two's complement)
        ld(&mut vm.registers, vm.memory, 0b0010_000_111111011);

        assert_eq!(vm.registers[Register::R0 as usize], 0xBEEF);
    }

    // ========== Pattern Tests ==========

    #[test]
    fn test_ld_alternating_pattern() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3001, 0xAAAA);

        // LD R1, 1
        ld(&mut vm.registers, vm.memory, 0b0010_001_000000001);

        assert_eq!(vm.registers[Register::R1 as usize], 0xAAAA);
    }

    #[test]
    fn test_ld_all_bits_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_memory(0x3001, 0xFFFF);

        // LD R2, 1
        ld(&mut vm.registers, vm.memory, 0b0010_010_000000001);

        assert_eq!(vm.registers[Register::R2 as usize], 0xFFFF);
    }
}
