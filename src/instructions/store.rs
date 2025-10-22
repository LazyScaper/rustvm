use crate::instructions::sign_extend;
use crate::registers::register::Register::{Count, Pc};
use crate::MEMORY_MAX;

pub fn st(
    registers: &mut [u16; (Count as u16) as usize],
    memory: &mut [u16; MEMORY_MAX],
    instruction: u16,
) {
    let source_register = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    memory[registers[Pc as usize].wrapping_add(pc_offset) as usize] =
        registers[source_register as usize];
}

#[cfg(test)]
mod tests {
    use crate::instructions::store::st;
    use crate::registers::register::Register;
    use crate::Vm;

    // ========== Basic ST Operations ==========

    #[test]
    fn test_st_basic_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 42);

        // ST R2, 5 (store to PC + 5)
        st(&mut vm.registers, &mut vm.memory, 0b0011_010_000000101);

        assert_eq!(vm.memory[0x3005], 42);
    }

    #[test]
    fn test_st_zero_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R3, 123);

        // ST R3, 0 (store to PC + 0)
        st(&mut vm.registers, &mut vm.memory, 0b0011_011_000000000);

        assert_eq!(vm.memory[0x3000], 123);
    }

    #[test]
    fn test_st_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3010);
        vm.write_to_register(Register::R1, 99);

        // ST R1, -8 (0x1F8 in 9-bit two's complement)
        st(&mut vm.registers, &mut vm.memory, 0b0011_001_111111000);

        assert_eq!(vm.memory[0x3008], 99);
    }

    #[test]
    fn test_st_max_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R4, 255);

        // ST R4, 255 (max positive 9-bit offset)
        st(&mut vm.registers, &mut vm.memory, 0b0011_100_011111111);

        assert_eq!(vm.memory[0x30FF], 255);
    }

    #[test]
    fn test_st_max_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3100);
        vm.write_to_register(Register::R5, 77);

        // ST R5, -256 (max negative 9-bit offset)
        st(&mut vm.registers, &mut vm.memory, 0b0011_101_100000000);

        assert_eq!(vm.memory[0x3000], 77);
    }

    // ========== Store from Different Registers ==========

    #[test]
    fn test_st_from_r0() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 111);

        // ST R0, 1
        st(&mut vm.registers, &mut vm.memory, 0b0011_000_000000001);

        assert_eq!(vm.memory[0x3001], 111);
    }

    #[test]
    fn test_st_from_r1() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 222);

        // ST R1, 2
        st(&mut vm.registers, &mut vm.memory, 0b0011_001_000000010);

        assert_eq!(vm.memory[0x3002], 222);
    }

    #[test]
    fn test_st_from_r2() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 333);

        // ST R2, 3
        st(&mut vm.registers, &mut vm.memory, 0b0011_010_000000011);

        assert_eq!(vm.memory[0x3003], 333);
    }

    #[test]
    fn test_st_from_r3() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R3, 444);

        // ST R3, 4
        st(&mut vm.registers, &mut vm.memory, 0b0011_011_000000100);

        assert_eq!(vm.memory[0x3004], 444);
    }

    #[test]
    fn test_st_from_r4() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R4, 555);

        // ST R4, 5
        st(&mut vm.registers, &mut vm.memory, 0b0011_100_000000101);

        assert_eq!(vm.memory[0x3005], 555);
    }

    #[test]
    fn test_st_from_r5() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R5, 666);

        // ST R5, 6
        st(&mut vm.registers, &mut vm.memory, 0b0011_101_000000110);

        assert_eq!(vm.memory[0x3006], 666);
    }

    #[test]
    fn test_st_from_r6() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R6, 777);

        // ST R6, 7
        st(&mut vm.registers, &mut vm.memory, 0b0011_110_000000111);

        assert_eq!(vm.memory[0x3007], 777);
    }

    #[test]
    fn test_st_from_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R7, 888);

        // ST R7, 8
        st(&mut vm.registers, &mut vm.memory, 0b0011_111_000001000);

        assert_eq!(vm.memory[0x3008], 888);
    }

    // ========== Different Value Types ==========

    #[test]
    fn test_st_zero_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 0);
        vm.write_to_memory(0x3001, 0xFFFF); // Pre-existing value

        // ST R2, 1
        st(&mut vm.registers, &mut vm.memory, 0b0011_010_000000001);

        assert_eq!(vm.memory[0x3001], 0);
    }

    #[test]
    fn test_st_max_positive_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R3, 0x7FFF); // Max positive 16-bit signed

        // ST R3, 1
        st(&mut vm.registers, &mut vm.memory, 0b0011_011_000000001);

        assert_eq!(vm.memory[0x3001], 0x7FFF);
    }

    #[test]
    fn test_st_negative_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R4, 0xFFFF); // -1 in two's complement

        // ST R4, 1
        st(&mut vm.registers, &mut vm.memory, 0b0011_100_000000001);

        assert_eq!(vm.memory[0x3001], 0xFFFF);
    }

    #[test]
    fn test_st_max_unsigned_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R5, 0xFFFF);

        // ST R5, 1
        st(&mut vm.registers, &mut vm.memory, 0b0011_101_000000001);

        assert_eq!(vm.memory[0x3001], 0xFFFF);
    }

    #[test]
    fn test_st_various_hex_values() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        let test_values = [0x1234, 0xABCD, 0x5678, 0xDEAD, 0xBEEF, 0xCAFE];

        for (i, &value) in test_values.iter().enumerate() {
            vm.write_to_register(Register::R0, value);

            let instruction = 0b0011_000_000000000 | (i as u16);
            st(&mut vm.registers, &mut vm.memory, instruction);

            assert_eq!(vm.memory[0x3000 + i], value);
        }
    }

    // ========== PC-Relative Tests ==========

    #[test]
    fn test_st_from_different_pc_values() {
        let pc_values = [0x0100, 0x1000, 0x3000, 0x5000, 0x7000, 0xA000];

        for &pc in &pc_values {
            let mut vm = Vm::new();
            vm.write_to_register(Register::Pc, pc);
            vm.write_to_register(Register::R1, 42);

            // ST R1, 10
            st(&mut vm.registers, &mut vm.memory, 0b0011_001_000001010);

            assert_eq!(vm.memory[(pc + 10) as usize], 42);
        }
    }

    #[test]
    fn test_st_to_low_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x0010);
        vm.write_to_register(Register::R2, 99);

        // ST R2, 5
        st(&mut vm.registers, &mut vm.memory, 0b0011_010_000000101);

        assert_eq!(vm.memory[0x0015], 99);
    }

    #[test]
    fn test_st_to_high_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0xFE00);
        vm.write_to_register(Register::R3, 88);

        // ST R3, 16
        st(&mut vm.registers, &mut vm.memory, 0b0011_011_000010000);

        assert_eq!(vm.memory[0xFE10], 88);
    }

    // ========== Overwriting Memory ==========

    #[test]
    fn test_st_overwrites_existing_memory_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 42);
        vm.write_to_memory(0x3005, 9999); // Pre-existing value

        // ST R2, 5
        st(&mut vm.registers, &mut vm.memory, 0b0011_010_000000101);

        assert_eq!(vm.memory[0x3005], 42);
    }

    #[test]
    fn test_st_does_not_modify_source_register() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R3, 123);

        // ST R3, 5
        st(&mut vm.registers, &mut vm.memory, 0b0011_011_000000101);

        // Source register should remain unchanged
        assert_eq!(vm.registers[Register::R3 as usize], 123);
        assert_eq!(vm.memory[0x3005], 123);
    }

    #[test]
    fn test_st_preserves_other_registers() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x1111);
        vm.write_to_register(Register::R1, 0x2222);
        vm.write_to_register(Register::R2, 42);
        vm.write_to_register(Register::R3, 0x3333);

        // ST R2, 5
        st(&mut vm.registers, &mut vm.memory, 0b0011_010_000000101);

        // Check other registers unchanged
        assert_eq!(vm.registers[Register::R0 as usize], 0x1111);
        assert_eq!(vm.registers[Register::R1 as usize], 0x2222);
        assert_eq!(vm.registers[Register::R2 as usize], 42);
        assert_eq!(vm.registers[Register::R3 as usize], 0x3333);
    }

    #[test]
    fn test_st_preserves_other_memory_locations() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 42);
        vm.write_to_memory(0x3004, 0xAAAA);
        vm.write_to_memory(0x3006, 0xBBBB);

        // ST R1, 5 (to 0x3005)
        st(&mut vm.registers, &mut vm.memory, 0b0011_001_000000101);

        // Check neighboring memory unchanged
        assert_eq!(vm.memory[0x3004], 0xAAAA);
        assert_eq!(vm.memory[0x3005], 42);
        assert_eq!(vm.memory[0x3006], 0xBBBB);
    }

    // ========== Sequential Stores ==========

    #[test]
    fn test_st_multiple_values_sequentially() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 10);
        vm.write_to_register(Register::R2, 20);
        vm.write_to_register(Register::R3, 30);

        // ST R1, 1
        st(&mut vm.registers, &mut vm.memory, 0b0011_001_000000001);
        assert_eq!(vm.memory[0x3001], 10);

        // ST R2, 2
        st(&mut vm.registers, &mut vm.memory, 0b0011_010_000000010);
        assert_eq!(vm.memory[0x3002], 20);

        // ST R3, 3
        st(&mut vm.registers, &mut vm.memory, 0b0011_011_000000011);
        assert_eq!(vm.memory[0x3003], 30);
    }

    #[test]
    fn test_st_same_location_multiple_times() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 10);
        vm.write_to_register(Register::R2, 20);
        vm.write_to_register(Register::R3, 30);

        // ST R1, 5
        st(&mut vm.registers, &mut vm.memory, 0b0011_001_000000101);
        assert_eq!(vm.memory[0x3005], 10);

        // ST R2, 5 (overwrite)
        st(&mut vm.registers, &mut vm.memory, 0b0011_010_000000101);
        assert_eq!(vm.memory[0x3005], 20);

        // ST R3, 5 (overwrite again)
        st(&mut vm.registers, &mut vm.memory, 0b0011_011_000000101);
        assert_eq!(vm.memory[0x3005], 30);
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_st_offset_minus_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3001);
        vm.write_to_register(Register::R4, 55);

        // ST R4, -1 (0x1FF in 9-bit two's complement)
        st(&mut vm.registers, &mut vm.memory, 0b0011_100_111111111);

        assert_eq!(vm.memory[0x3000], 55);
    }

    #[test]
    fn test_st_offset_plus_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R5, 66);

        // ST R5, 1
        st(&mut vm.registers, &mut vm.memory, 0b0011_101_000000001);

        assert_eq!(vm.memory[0x3001], 66);
    }

    #[test]
    fn test_st_backward_from_current_pc() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3050);
        vm.write_to_register(Register::R6, 123);

        // ST R6, -80 (0x1B0 in 9-bit two's complement)
        st(&mut vm.registers, &mut vm.memory, 0b0011_110_110110000);

        assert_eq!(vm.memory[0x3000], 123);
    }

    // ========== Store and Load Pattern ==========

    #[test]
    fn test_st_then_ld_pattern() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 42);

        // ST R1, 10
        st(&mut vm.registers, &mut vm.memory, 0b0011_001_000001010);

        // Verify the store
        assert_eq!(vm.memory[0x300A], 42);

        // Now if we were to LD R2, 10 it would load 42
        assert_eq!(vm.memory[0x300A], 42);
    }

    #[test]
    fn test_st_array_pattern() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // Store array values
        for i in 0..10 {
            vm.write_to_register(Register::R0, i * 10);
            let instruction = 0b0011_000_000000000 | i;
            st(&mut vm.registers, &mut vm.memory, instruction);
        }

        // Verify array
        for i in 0..10 {
            assert_eq!(vm.memory[(0x3000 + i) as usize], i * 10);
        }
    }

    // ========== Storing to Data Section ==========

    #[test]
    fn test_st_to_data_table() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x30F0);
        vm.write_to_register(Register::R0, 100);
        vm.write_to_register(Register::R1, 200);
        vm.write_to_register(Register::R2, 300);

        // ST R0, 16 (to 0x3100)
        st(&mut vm.registers, &mut vm.memory, 0b0011_000_000010000);
        assert_eq!(vm.memory[0x3100], 100);

        // ST R1, 17 (to 0x3101)
        st(&mut vm.registers, &mut vm.memory, 0b0011_001_000010001);
        assert_eq!(vm.memory[0x3101], 200);

        // ST R2, 18 (to 0x3102)
        st(&mut vm.registers, &mut vm.memory, 0b0011_010_000010010);
        assert_eq!(vm.memory[0x3102], 300);
    }

    #[test]
    fn test_st_ascii_values() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 0x0041); // 'A'
        vm.write_to_register(Register::R2, 0x0042); // 'B'
        vm.write_to_register(Register::R3, 0x0043); // 'C'

        // ST R1, 1
        st(&mut vm.registers, &mut vm.memory, 0b0011_001_000000001);
        assert_eq!(vm.memory[0x3001], 0x0041);

        // ST R2, 2
        st(&mut vm.registers, &mut vm.memory, 0b0011_010_000000010);
        assert_eq!(vm.memory[0x3002], 0x0042);

        // ST R3, 3
        st(&mut vm.registers, &mut vm.memory, 0b0011_011_000000011);
        assert_eq!(vm.memory[0x3003], 0x0043);
    }

    // ========== Boundary Conditions ==========

    #[test]
    fn test_st_at_memory_boundary() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0xFFF0);
        vm.write_to_register(Register::R7, 0xDEAD);

        // ST R7, 15
        st(&mut vm.registers, &mut vm.memory, 0b0011_111_000001111);

        assert_eq!(vm.memory[0xFFFF], 0xDEAD);
    }

    #[test]
    fn test_st_to_address_zero() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x0005);
        vm.write_to_register(Register::R0, 0xBEEF);

        // ST R0, -5 (0x1FB in 9-bit two's complement)
        st(&mut vm.registers, &mut vm.memory, 0b0011_000_111111011);

        assert_eq!(vm.memory[0x0000], 0xBEEF);
    }

    // ========== Pattern Tests ==========

    #[test]
    fn test_st_alternating_pattern() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 0xAAAA);

        // ST R1, 1
        st(&mut vm.registers, &mut vm.memory, 0b0011_001_000000001);

        assert_eq!(vm.memory[0x3001], 0xAAAA);
    }

    #[test]
    fn test_st_all_bits_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 0xFFFF);

        // ST R2, 1
        st(&mut vm.registers, &mut vm.memory, 0b0011_010_000000001);

        assert_eq!(vm.memory[0x3001], 0xFFFF);
    }

    // ========== Round-trip Test ==========

    #[test]
    fn test_st_ld_round_trip() {
        let mut vm = Vm::new();
        let original_value = 0xABCD;
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, original_value);

        // ST R1, 10
        st(&mut vm.registers, &mut vm.memory, 0b0011_001_000001010);

        // Clear R1
        vm.write_to_register(Register::R1, 0);

        // LD R1, 10 (would load back the value)
        assert_eq!(vm.memory[0x300A], original_value);
    }
}
