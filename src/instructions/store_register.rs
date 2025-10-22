use crate::instructions::sign_extend;
use crate::registers::register::Register::Count;
use crate::MEMORY_MAX;

pub fn str(
    registers: &mut [u16; (Count as u16) as usize],
    memory: &mut [u16; MEMORY_MAX],
    instruction: u16,
) {
    let source_register = (instruction >> 9) & 0x7;
    let base_register = (instruction >> 6) & 0x7;
    let offset_6 = sign_extend(instruction & 0x3F, 6);

    memory[registers[base_register as usize].wrapping_add(offset_6) as usize] =
        registers[source_register as usize];
}

#[cfg(test)]
mod tests {
    use crate::instructions::store_register::str;
    use crate::registers::register::Register;
    use crate::Vm;

    // ========== Basic STR Operations ==========

    #[test]
    fn test_str_basic_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x3000);
        vm.write_to_register(Register::R2, 42);

        // STR R2, R1, 5 (store to R1 + 5)
        str(&mut vm.registers, &mut vm.memory, 0b0111_010_001_000101);

        assert_eq!(vm.memory[0x3005], 42);
    }

    #[test]
    fn test_str_zero_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 0x3000);
        vm.write_to_register(Register::R4, 123);

        // STR R4, R3, 0 (store to R3 + 0)
        str(&mut vm.registers, &mut vm.memory, 0b0111_100_011_000000);

        assert_eq!(vm.memory[0x3000], 123);
    }

    #[test]
    fn test_str_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x3010);
        vm.write_to_register(Register::R2, 99);

        // STR R2, R1, -8 (0x38 in 6-bit two's complement)
        str(&mut vm.registers, &mut vm.memory, 0b0111_010_001_111000);

        assert_eq!(vm.memory[0x3008], 99);
    }

    #[test]
    fn test_str_max_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0x3000);
        vm.write_to_register(Register::R1, 255);

        // STR R1, R0, 31 (max positive 6-bit offset)
        str(&mut vm.registers, &mut vm.memory, 0b0111_001_000_011111);

        assert_eq!(vm.memory[0x301F], 255);
    }

    #[test]
    fn test_str_max_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R2, 0x3020);
        vm.write_to_register(Register::R3, 77);

        // STR R3, R2, -32 (max negative 6-bit offset)
        str(&mut vm.registers, &mut vm.memory, 0b0111_011_010_100000);

        assert_eq!(vm.memory[0x3000], 77);
    }

    // ========== Store from Different Source Registers ==========

    #[test]
    fn test_str_from_r0() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 111);
        vm.write_to_register(Register::R1, 0x3000);

        // STR R0, R1, 1
        str(&mut vm.registers, &mut vm.memory, 0b0111_000_001_000001);

        assert_eq!(vm.memory[0x3001], 111);
    }

    #[test]
    fn test_str_from_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R7, 222);
        vm.write_to_register(Register::R6, 0x3000);

        // STR R7, R6, 2
        str(&mut vm.registers, &mut vm.memory, 0b0111_111_110_000010);

        assert_eq!(vm.memory[0x3002], 222);
    }

    // ========== Store to Different Base Registers ==========

    #[test]
    fn test_str_to_base_r0() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0x4000);
        vm.write_to_register(Register::R1, 333);

        // STR R1, R0, 5
        str(&mut vm.registers, &mut vm.memory, 0b0111_001_000_000101);

        assert_eq!(vm.memory[0x4005], 333);
    }

    #[test]
    fn test_str_to_base_r1() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x5000);
        vm.write_to_register(Register::R2, 444);

        // STR R2, R1, 3
        str(&mut vm.registers, &mut vm.memory, 0b0111_010_001_000011);

        assert_eq!(vm.memory[0x5003], 444);
    }

    #[test]
    fn test_str_to_base_r2() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R2, 0x6000);
        vm.write_to_register(Register::R3, 555);

        // STR R3, R2, 7
        str(&mut vm.registers, &mut vm.memory, 0b0111_011_010_000111);

        assert_eq!(vm.memory[0x6007], 555);
    }

    #[test]
    fn test_str_to_base_r3() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 0x7000);
        vm.write_to_register(Register::R4, 666);

        // STR R4, R3, 2
        str(&mut vm.registers, &mut vm.memory, 0b0111_100_011_000010);

        assert_eq!(vm.memory[0x7002], 666);
    }

    #[test]
    fn test_str_to_base_r4() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R4, 0x8000);
        vm.write_to_register(Register::R5, 777);

        // STR R5, R4, 4
        str(&mut vm.registers, &mut vm.memory, 0b0111_101_100_000100);

        assert_eq!(vm.memory[0x8004], 777);
    }

    #[test]
    fn test_str_to_base_r5() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R5, 0x9000);
        vm.write_to_register(Register::R6, 888);

        // STR R6, R5, 1
        str(&mut vm.registers, &mut vm.memory, 0b0111_110_101_000001);

        assert_eq!(vm.memory[0x9001], 888);
    }

    #[test]
    fn test_str_to_base_r6() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R6, 0xA000);
        vm.write_to_register(Register::R7, 999);

        // STR R7, R6, 6
        str(&mut vm.registers, &mut vm.memory, 0b0111_111_110_000110);

        assert_eq!(vm.memory[0xA006], 999);
    }

    #[test]
    fn test_str_to_base_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R7, 0xB000);
        vm.write_to_register(Register::R0, 1111);

        // STR R0, R7, 3
        str(&mut vm.registers, &mut vm.memory, 0b0111_000_111_000011);

        assert_eq!(vm.memory[0xB003], 1111);
    }

    // ========== Same Register as Source and Base ==========

    #[test]
    fn test_str_source_same_as_base() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 0x3000);

        // STR R3, R3, 5 (store R3's value at R3 + 5)
        str(&mut vm.registers, &mut vm.memory, 0b0111_011_011_000101);

        assert_eq!(vm.memory[0x3005], 0x3000);
    }

    // ========== Different Value Types ==========

    #[test]
    fn test_str_zero_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x3000);
        vm.write_to_register(Register::R2, 0);
        vm.write_to_memory(0x3001, 0xFFFF); // Pre-existing value

        // STR R2, R1, 1
        str(&mut vm.registers, &mut vm.memory, 0b0111_010_001_000001);

        assert_eq!(vm.memory[0x3001], 0);
    }

    #[test]
    fn test_str_max_positive_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 0x3000);
        vm.write_to_register(Register::R4, 0x7FFF);

        // STR R4, R3, 1
        str(&mut vm.registers, &mut vm.memory, 0b0111_100_011_000001);

        assert_eq!(vm.memory[0x3001], 0x7FFF);
    }

    #[test]
    fn test_str_negative_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R5, 0x3000);
        vm.write_to_register(Register::R6, 0xFFFF);

        // STR R6, R5, 1
        str(&mut vm.registers, &mut vm.memory, 0b0111_110_101_000001);

        assert_eq!(vm.memory[0x3001], 0xFFFF);
    }

    #[test]
    fn test_str_various_hex_values() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x3000);

        let test_values = [0x1234, 0xABCD, 0x5678, 0xDEAD, 0xBEEF];

        for (i, &value) in test_values.iter().enumerate() {
            vm.write_to_register(Register::R0, value);

            let instruction = 0b0111_000_001_000000 | (i as u16);
            str(&mut vm.registers, &mut vm.memory, instruction);

            assert_eq!(vm.memory[0x3000 + i], value);
        }
    }

    // ========== Base Register Tests ==========

    #[test]
    fn test_str_to_different_base_addresses() {
        let base_addresses = [0x0100, 0x1000, 0x3000, 0x5000, 0x7000, 0xA000];

        for &base in &base_addresses {
            let mut vm = Vm::new();
            vm.write_to_register(Register::R1, base);
            vm.write_to_register(Register::R2, 42);

            // STR R2, R1, 10
            str(&mut vm.registers, &mut vm.memory, 0b0111_010_001_001010);

            assert_eq!(vm.memory[(base + 10) as usize], 42);
        }
    }

    #[test]
    fn test_str_to_low_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R2, 0x0010);
        vm.write_to_register(Register::R3, 99);

        // STR R3, R2, 5
        str(&mut vm.registers, &mut vm.memory, 0b0111_011_010_000101);

        assert_eq!(vm.memory[0x0015], 99);
    }

    #[test]
    fn test_str_to_high_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R4, 0xFE00);
        vm.write_to_register(Register::R5, 88);

        // STR R5, R4, 16
        str(&mut vm.registers, &mut vm.memory, 0b0111_101_100_010000);

        assert_eq!(vm.memory[0xFE10], 88);
    }

    // ========== Preserving Registers ==========

    #[test]
    fn test_str_does_not_modify_source_register() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x3000);
        vm.write_to_register(Register::R2, 123);

        // STR R2, R1, 5
        str(&mut vm.registers, &mut vm.memory, 0b0111_010_001_000101);

        // Source register should remain unchanged
        assert_eq!(vm.registers[Register::R2 as usize], 123);
        assert_eq!(vm.memory[0x3005], 123);
    }

    #[test]
    fn test_str_does_not_modify_base_register() {
        let mut vm = Vm::new();
        let base_address = 0x3000;
        vm.write_to_register(Register::R1, base_address);
        vm.write_to_register(Register::R2, 42);

        // STR R2, R1, 5
        str(&mut vm.registers, &mut vm.memory, 0b0111_010_001_000101);

        // Base register should be unchanged
        assert_eq!(vm.registers[Register::R1 as usize], base_address);
    }

    #[test]
    fn test_str_preserves_other_registers() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0x1111);
        vm.write_to_register(Register::R1, 0x3000);
        vm.write_to_register(Register::R2, 42);
        vm.write_to_register(Register::R3, 0x3333);

        // STR R2, R1, 5
        str(&mut vm.registers, &mut vm.memory, 0b0111_010_001_000101);

        // Check other registers unchanged
        assert_eq!(vm.registers[Register::R0 as usize], 0x1111);
        assert_eq!(vm.registers[Register::R1 as usize], 0x3000);
        assert_eq!(vm.registers[Register::R2 as usize], 42);
        assert_eq!(vm.registers[Register::R3 as usize], 0x3333);
    }

    #[test]
    fn test_str_preserves_other_memory_locations() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x3000);
        vm.write_to_register(Register::R2, 42);
        vm.write_to_memory(0x3004, 0xAAAA);
        vm.write_to_memory(0x3006, 0xBBBB);

        // STR R2, R1, 5 (to 0x3005)
        str(&mut vm.registers, &mut vm.memory, 0b0111_010_001_000101);

        // Check neighboring memory unchanged
        assert_eq!(vm.memory[0x3004], 0xAAAA);
        assert_eq!(vm.memory[0x3005], 42);
        assert_eq!(vm.memory[0x3006], 0xBBBB);
    }

    // ========== Overwriting Memory ==========

    #[test]
    fn test_str_overwrites_existing_memory_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x3000);
        vm.write_to_register(Register::R2, 42);
        vm.write_to_memory(0x3005, 9999); // Pre-existing value

        // STR R2, R1, 5
        str(&mut vm.registers, &mut vm.memory, 0b0111_010_001_000101);

        assert_eq!(vm.memory[0x3005], 42);
    }

    // ========== Sequential Stores ==========

    #[test]
    fn test_str_multiple_values_sequentially() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0x3000);
        vm.write_to_register(Register::R1, 10);
        vm.write_to_register(Register::R2, 20);
        vm.write_to_register(Register::R3, 30);

        // STR R1, R0, 1
        str(&mut vm.registers, &mut vm.memory, 0b0111_001_000_000001);
        assert_eq!(vm.memory[0x3001], 10);

        // STR R2, R0, 2
        str(&mut vm.registers, &mut vm.memory, 0b0111_010_000_000010);
        assert_eq!(vm.memory[0x3002], 20);

        // STR R3, R0, 3
        str(&mut vm.registers, &mut vm.memory, 0b0111_011_000_000011);
        assert_eq!(vm.memory[0x3003], 30);
    }

    #[test]
    fn test_str_same_location_multiple_times() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0x3000);
        vm.write_to_register(Register::R1, 10);
        vm.write_to_register(Register::R2, 20);
        vm.write_to_register(Register::R3, 30);

        // STR R1, R0, 5
        str(&mut vm.registers, &mut vm.memory, 0b0111_001_000_000101);
        assert_eq!(vm.memory[0x3005], 10);

        // STR R2, R0, 5 (overwrite)
        str(&mut vm.registers, &mut vm.memory, 0b0111_010_000_000101);
        assert_eq!(vm.memory[0x3005], 20);

        // STR R3, R0, 5 (overwrite again)
        str(&mut vm.registers, &mut vm.memory, 0b0111_011_000_000101);
        assert_eq!(vm.memory[0x3005], 30);
    }

    #[test]
    fn test_str_array_population() {
        let mut vm = Vm::new();
        let array_base = 0x3000;
        vm.write_to_register(Register::R0, array_base);

        // Populate array
        for i in 0..10 {
            vm.write_to_register(Register::R1, i * 10);
            let instruction = 0b0111_001_000_000000 | (i as u16);
            str(&mut vm.registers, &mut vm.memory, instruction);
        }

        // Verify array
        for i in 0..10 {
            assert_eq!(vm.memory[(array_base + i) as usize], i * 10);
        }
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_str_offset_minus_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R4, 0x3001);
        vm.write_to_register(Register::R5, 55);

        // STR R5, R4, -1 (0x3F in 6-bit two's complement)
        str(&mut vm.registers, &mut vm.memory, 0b0111_101_100_111111);

        assert_eq!(vm.memory[0x3000], 55);
    }

    #[test]
    fn test_str_offset_plus_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R6, 0x3000);
        vm.write_to_register(Register::R7, 66);

        // STR R7, R6, 1
        str(&mut vm.registers, &mut vm.memory, 0b0111_111_110_000001);

        assert_eq!(vm.memory[0x3001], 66);
    }

    // ========== Pointer-Based Storage ==========

    #[test]
    fn test_str_pointer_write() {
        let mut vm = Vm::new();
        let pointer = 0x4000;
        vm.write_to_register(Register::R1, pointer);
        vm.write_to_register(Register::R2, 123);

        // STR R2, R1, 0 (write through pointer)
        str(&mut vm.registers, &mut vm.memory, 0b0111_010_001_000000);

        assert_eq!(vm.memory[pointer as usize], 123);
    }

    #[test]
    fn test_str_struct_field_write() {
        let mut vm = Vm::new();
        let struct_base = 0x3000;
        vm.write_to_register(Register::R0, struct_base);
        vm.write_to_register(Register::R1, 100);
        vm.write_to_register(Register::R2, 200);
        vm.write_to_register(Register::R3, 300);

        // Store field 0
        str(&mut vm.registers, &mut vm.memory, 0b0111_001_000_000000);
        assert_eq!(vm.memory[struct_base as usize], 100);

        // Store field 1
        str(&mut vm.registers, &mut vm.memory, 0b0111_010_000_000001);
        assert_eq!(vm.memory[(struct_base + 1) as usize], 200);

        // Store field 2
        str(&mut vm.registers, &mut vm.memory, 0b0111_011_000_000010);
        assert_eq!(vm.memory[(struct_base + 2) as usize], 300);
    }

    // ========== Store and Load Pattern ==========

    #[test]
    fn test_str_then_ldr_pattern() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x3000);
        vm.write_to_register(Register::R2, 42);

        // STR R2, R1, 10
        str(&mut vm.registers, &mut vm.memory, 0b0111_010_001_001010);

        // Verify the store
        assert_eq!(vm.memory[0x300A], 42);

        // Could now LDR R3, R1, 10 to load it back
    }

    // ========== Boundary Conditions ==========

    #[test]
    fn test_str_at_memory_boundary() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R7, 0xFFE0);
        vm.write_to_register(Register::R0, 0xDEAD);

        // STR R0, R7, 31
        str(&mut vm.registers, &mut vm.memory, 0b0111_000_111_011111);

        assert_eq!(vm.memory[0xFFFF], 0xDEAD);
    }

    #[test]
    fn test_str_to_address_zero() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x0005);
        vm.write_to_register(Register::R0, 0xBEEF);

        // STR R0, R1, -5 (0x3B in 6-bit two's complement)
        str(&mut vm.registers, &mut vm.memory, 0b0111_000_001_111011);

        assert_eq!(vm.memory[0x0000], 0xBEEF);
    }

    // ========== Pattern Tests ==========

    #[test]
    fn test_str_alternating_pattern() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x3000);
        vm.write_to_register(Register::R2, 0xAAAA);

        // STR R2, R1, 1
        str(&mut vm.registers, &mut vm.memory, 0b0111_010_001_000001);

        assert_eq!(vm.memory[0x3001], 0xAAAA);
    }

    #[test]
    fn test_str_all_bits_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 0x3000);
        vm.write_to_register(Register::R4, 0xFFFF);

        // STR R4, R3, 1
        str(&mut vm.registers, &mut vm.memory, 0b0111_100_011_000001);

        assert_eq!(vm.memory[0x3001], 0xFFFF);
    }

    // ========== ASCII and Character Data ==========

    #[test]
    fn test_str_ascii_string() {
        let mut vm = Vm::new();
        let string_base = 0x3000;
        vm.write_to_register(Register::R0, string_base);
        vm.write_to_register(Register::R1, 0x0041); // 'A'
        vm.write_to_register(Register::R2, 0x0042); // 'B'
        vm.write_to_register(Register::R3, 0x0043); // 'C'

        // Store characters
        str(&mut vm.registers, &mut vm.memory, 0b0111_001_000_000000);
        assert_eq!(vm.memory[string_base as usize], 0x0041);

        str(&mut vm.registers, &mut vm.memory, 0b0111_010_000_000001);
        assert_eq!(vm.memory[(string_base + 1) as usize], 0x0042);

        str(&mut vm.registers, &mut vm.memory, 0b0111_011_000_000010);
        assert_eq!(vm.memory[(string_base + 2) as usize], 0x0043);
    }

    // ========== Round-trip Test ==========

    #[test]
    fn test_str_ldr_round_trip() {
        let mut vm = Vm::new();
        let original_value = 0xABCD;
        vm.write_to_register(Register::R0, 0x3000);
        vm.write_to_register(Register::R1, original_value);

        // STR R1, R0, 10
        str(&mut vm.registers, &mut vm.memory, 0b0111_001_000_001010);

        // Clear R1
        vm.write_to_register(Register::R1, 0);

        // Verify stored value
        assert_eq!(vm.memory[0x300A], original_value);

        // Could now LDR R1, R0, 10 to load it back
    }

    // ========== Comparison with ST ==========

    #[test]
    fn test_str_vs_st_difference() {
        let mut vm = Vm::new();
        // STR uses base register + offset
        vm.write_to_register(Register::R1, 0x3000);
        vm.write_to_register(Register::R2, 42);

        // STR R2, R1, 5
        str(&mut vm.registers, &mut vm.memory, 0b0111_010_001_000101);

        assert_eq!(vm.memory[0x3005], 42);

        // ST would use PC + offset instead
        // This test just confirms STR uses the base register
    }
}
