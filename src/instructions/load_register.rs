use crate::instructions::{sign_extend, update_flags};
use crate::registers::register::Register::Count;
use crate::MEMORY_MAX;

pub fn ldr(
    registers: &mut [u16; (Count as u16) as usize],
    memory: &[u16; MEMORY_MAX],
    instruction: u16,
) {
    let destination_register = (instruction >> 9) & 0x7;
    let base_register = (instruction >> 6) & 0x7;
    let offset_6 = sign_extend(instruction & 0x3F, 6);

    registers[destination_register as usize] =
        memory[registers[base_register as usize].wrapping_add(offset_6) as usize];

    update_flags(registers, destination_register);
}

#[cfg(test)]
mod tests {
    use crate::instructions::load_register::ldr;
    use crate::registers::register::Register;
    use crate::Vm;

    // ========== Basic LDR Operations ==========

    #[test]
    fn test_ldr_basic_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x3000);
        vm.mem_write(0x3005, 42);

        // LDR R2, R1, 5 (load from R1 + 5)
        ldr(&mut vm.registers, &vm.memory, 0b0110_010_001_000101);

        assert_eq!(vm.registers[Register::R2 as usize], 42);
    }

    #[test]
    fn test_ldr_zero_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 0x3000);
        vm.mem_write(0x3000, 123);

        // LDR R4, R3, 0 (load from R3 + 0)
        ldr(&mut vm.registers, &vm.memory, 0b0110_100_011_000000);

        assert_eq!(vm.registers[Register::R4 as usize], 123);
    }

    #[test]
    fn test_ldr_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x3010);
        vm.mem_write(0x3008, 99);

        // LDR R2, R1, -8 (0x38 in 6-bit two's complement)
        ldr(&mut vm.registers, &vm.memory, 0b0110_010_001_111000);

        assert_eq!(vm.registers[Register::R2 as usize], 99);
    }

    #[test]
    fn test_ldr_max_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0x3000);
        vm.mem_write(0x301F, 255);

        // LDR R1, R0, 31 (max positive 6-bit offset)
        ldr(&mut vm.registers, &vm.memory, 0b0110_001_000_011111);

        assert_eq!(vm.registers[Register::R1 as usize], 255);
    }

    #[test]
    fn test_ldr_max_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R2, 0x3020);
        vm.mem_write(0x3000, 77);

        // LDR R3, R2, -32 (max negative 6-bit offset)
        ldr(&mut vm.registers, &vm.memory, 0b0110_011_010_100000);

        assert_eq!(vm.registers[Register::R3 as usize], 77);
    }

    // ========== Load to Different Destination Registers ==========

    #[test]
    fn test_ldr_to_r0() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x3000);
        vm.mem_write(0x3001, 111);

        // LDR R0, R1, 1
        ldr(&mut vm.registers, &vm.memory, 0b0110_000_001_000001);

        assert_eq!(vm.registers[Register::R0 as usize], 111);
    }

    #[test]
    fn test_ldr_to_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R6, 0x3000);
        vm.mem_write(0x3002, 222);

        // LDR R7, R6, 2
        ldr(&mut vm.registers, &vm.memory, 0b0110_111_110_000010);

        assert_eq!(vm.registers[Register::R7 as usize], 222);
    }

    // ========== Load from Different Base Registers ==========

    #[test]
    fn test_ldr_from_r0() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0x4000);
        vm.mem_write(0x4005, 333);

        // LDR R1, R0, 5
        ldr(&mut vm.registers, &vm.memory, 0b0110_001_000_000101);

        assert_eq!(vm.registers[Register::R1 as usize], 333);
    }

    #[test]
    fn test_ldr_from_r1() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x5000);
        vm.mem_write(0x5003, 444);

        // LDR R2, R1, 3
        ldr(&mut vm.registers, &vm.memory, 0b0110_010_001_000011);

        assert_eq!(vm.registers[Register::R2 as usize], 444);
    }

    #[test]
    fn test_ldr_from_r2() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R2, 0x6000);
        vm.mem_write(0x6007, 555);

        // LDR R3, R2, 7
        ldr(&mut vm.registers, &vm.memory, 0b0110_011_010_000111);

        assert_eq!(vm.registers[Register::R3 as usize], 555);
    }

    #[test]
    fn test_ldr_from_r3() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 0x7000);
        vm.mem_write(0x7002, 666);

        // LDR R4, R3, 2
        ldr(&mut vm.registers, &vm.memory, 0b0110_100_011_000010);

        assert_eq!(vm.registers[Register::R4 as usize], 666);
    }

    #[test]
    fn test_ldr_from_r4() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R4, 0x8000);
        vm.mem_write(0x8004, 777);

        // LDR R5, R4, 4
        ldr(&mut vm.registers, &vm.memory, 0b0110_101_100_000100);

        assert_eq!(vm.registers[Register::R5 as usize], 777);
    }

    #[test]
    fn test_ldr_from_r5() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R5, 0x9000);
        vm.mem_write(0x9001, 888);

        // LDR R6, R5, 1
        ldr(&mut vm.registers, &vm.memory, 0b0110_110_101_000001);

        assert_eq!(vm.registers[Register::R6 as usize], 888);
    }

    #[test]
    fn test_ldr_from_r6() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R6, 0xA000);
        vm.mem_write(0xA006, 999);

        // LDR R7, R6, 6
        ldr(&mut vm.registers, &vm.memory, 0b0110_111_110_000110);

        assert_eq!(vm.registers[Register::R7 as usize], 999);
    }

    #[test]
    fn test_ldr_from_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R7, 0xB000);
        vm.mem_write(0xB003, 1111);

        // LDR R0, R7, 3
        ldr(&mut vm.registers, &vm.memory, 0b0110_000_111_000011);

        assert_eq!(vm.registers[Register::R0 as usize], 1111);
    }

    // ========== Same Register as Destination and Base ==========

    #[test]
    fn test_ldr_destination_same_as_base() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 0x3000);
        vm.mem_write(0x3005, 42);

        // LDR R3, R3, 5 (load into same register used as base)
        ldr(&mut vm.registers, &vm.memory, 0b0110_011_011_000101);

        assert_eq!(vm.registers[Register::R3 as usize], 42);
    }

    // ========== Different Value Types ==========

    #[test]
    fn test_ldr_zero_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x3000);
        vm.mem_write(0x3001, 0);

        // LDR R2, R1, 1
        ldr(&mut vm.registers, &vm.memory, 0b0110_010_001_000001);

        assert_eq!(vm.registers[Register::R2 as usize], 0);
    }

    #[test]
    fn test_ldr_max_positive_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 0x3000);
        vm.mem_write(0x3001, 0x7FFF);

        // LDR R4, R3, 1
        ldr(&mut vm.registers, &vm.memory, 0b0110_100_011_000001);

        assert_eq!(vm.registers[Register::R4 as usize], 0x7FFF);
    }

    #[test]
    fn test_ldr_negative_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R5, 0x3000);
        vm.mem_write(0x3001, 0xFFFF);

        // LDR R6, R5, 1
        ldr(&mut vm.registers, &vm.memory, 0b0110_110_101_000001);

        assert_eq!(vm.registers[Register::R6 as usize], 0xFFFF);
    }

    #[test]
    fn test_ldr_various_hex_values() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0x3000);

        let test_values = [0x1234, 0xABCD, 0x5678, 0xDEAD, 0xBEEF];

        for (i, &value) in test_values.iter().enumerate() {
            vm.mem_write(0x3000 + i as u16, value);

            let instruction = 0b0110_001_000_000000 | (i as u16);
            ldr(&mut vm.registers, &vm.memory, instruction);

            assert_eq!(vm.registers[Register::R1 as usize], value);
        }
    }

    // ========== Base Register Tests ==========

    #[test]
    fn test_ldr_from_different_base_addresses() {
        let base_addresses = [0x0100, 0x1000, 0x3000, 0x5000, 0x7000, 0xA000];

        for &base in &base_addresses {
            let mut vm = Vm::new();
            vm.write_to_register(Register::R1, base);
            vm.mem_write(base + 10, 42);

            // LDR R2, R1, 10
            ldr(&mut vm.registers, &vm.memory, 0b0110_010_001_001010);

            assert_eq!(vm.registers[Register::R2 as usize], 42);
        }
    }

    #[test]
    fn test_ldr_from_low_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R2, 0x0010);
        vm.mem_write(0x0015, 99);

        // LDR R3, R2, 5
        ldr(&mut vm.registers, &vm.memory, 0b0110_011_010_000101);

        assert_eq!(vm.registers[Register::R3 as usize], 99);
    }

    #[test]
    fn test_ldr_from_high_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R4, 0xFE00);
        vm.mem_write(0xFE10, 88);

        // LDR R5, R4, 16
        ldr(&mut vm.registers, &vm.memory, 0b0110_101_100_010000);

        assert_eq!(vm.registers[Register::R5 as usize], 88);
    }

    // ========== Preserving Registers ==========

    #[test]
    fn test_ldr_does_not_modify_base_register() {
        let mut vm = Vm::new();
        let base_address = 0x3000;
        vm.write_to_register(Register::R1, base_address);
        vm.mem_write(0x3005, 42);

        // LDR R2, R1, 5
        ldr(&mut vm.registers, &vm.memory, 0b0110_010_001_000101);

        // Base register should be unchanged
        assert_eq!(vm.registers[Register::R1 as usize], base_address);
    }

    #[test]
    fn test_ldr_preserves_other_registers() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0x1111);
        vm.write_to_register(Register::R1, 0x3000);
        vm.write_to_register(Register::R3, 0x3333);
        vm.mem_write(0x3005, 42);

        // LDR R2, R1, 5
        ldr(&mut vm.registers, &vm.memory, 0b0110_010_001_000101);

        // Check other registers unchanged
        assert_eq!(vm.registers[Register::R0 as usize], 0x1111);
        assert_eq!(vm.registers[Register::R1 as usize], 0x3000);
        assert_eq!(vm.registers[Register::R3 as usize], 0x3333);
        assert_eq!(vm.registers[Register::R2 as usize], 42);
    }

    #[test]
    fn test_ldr_overwrites_existing_destination_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x3000);
        vm.write_to_register(Register::R2, 9999); // Pre-existing value
        vm.mem_write(0x3005, 42);

        // LDR R2, R1, 5
        ldr(&mut vm.registers, &vm.memory, 0b0110_010_001_000101);

        assert_eq!(vm.registers[Register::R2 as usize], 42);
    }

    // ========== Sequential Loads ==========

    #[test]
    fn test_ldr_multiple_values_sequentially() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R0, 0x3000);
        vm.mem_write(0x3001, 10);
        vm.mem_write(0x3002, 20);
        vm.mem_write(0x3003, 30);

        // LDR R1, R0, 1
        ldr(&mut vm.registers, &vm.memory, 0b0110_001_000_000001);
        assert_eq!(vm.registers[Register::R1 as usize], 10);

        // LDR R2, R0, 2
        ldr(&mut vm.registers, &vm.memory, 0b0110_010_000_000010);
        assert_eq!(vm.registers[Register::R2 as usize], 20);

        // LDR R3, R0, 3
        ldr(&mut vm.registers, &vm.memory, 0b0110_011_000_000011);
        assert_eq!(vm.registers[Register::R3 as usize], 30);
    }

    #[test]
    fn test_ldr_array_traversal() {
        let mut vm = Vm::new();
        let array_base = 0x3000;
        vm.write_to_register(Register::R0, array_base);

        // Setup array
        for i in 0..10 {
            vm.mem_write(array_base + i, i * 10);
        }

        // Load array elements
        for i in 0..10 {
            let instruction = 0b0110_001_000_000000 | (i as u16);
            ldr(&mut vm.registers, &vm.memory, instruction);
            assert_eq!(vm.registers[Register::R1 as usize], i * 10);
        }
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_ldr_offset_minus_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R4, 0x3001);
        vm.mem_write(0x3000, 55);

        // LDR R5, R4, -1 (0x3F in 6-bit two's complement)
        ldr(&mut vm.registers, &vm.memory, 0b0110_101_100_111111);

        assert_eq!(vm.registers[Register::R5 as usize], 55);
    }

    #[test]
    fn test_ldr_offset_plus_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R6, 0x3000);
        vm.mem_write(0x3001, 66);

        // LDR R7, R6, 1
        ldr(&mut vm.registers, &vm.memory, 0b0110_111_110_000001);

        assert_eq!(vm.registers[Register::R7 as usize], 66);
    }

    // ========== Pointer-Based Access ==========

    #[test]
    fn test_ldr_pointer_dereference() {
        let mut vm = Vm::new();
        let pointer = 0x4000;
        vm.write_to_register(Register::R1, pointer);
        vm.mem_write(pointer, 123);

        // LDR R2, R1, 0 (dereference pointer)
        ldr(&mut vm.registers, &vm.memory, 0b0110_010_001_000000);

        assert_eq!(vm.registers[Register::R2 as usize], 123);
    }

    #[test]
    fn test_ldr_struct_field_access() {
        let mut vm = Vm::new();
        let struct_base = 0x3000;
        vm.write_to_register(Register::R0, struct_base);

        // Simulate struct with fields at offsets
        vm.mem_write(struct_base + 0, 100); // field 0
        vm.mem_write(struct_base + 1, 200); // field 1
        vm.mem_write(struct_base + 2, 300); // field 2

        // Load field 1
        ldr(&mut vm.registers, &vm.memory, 0b0110_001_000_000001);
        assert_eq!(vm.registers[Register::R1 as usize], 200);

        // Load field 2
        ldr(&mut vm.registers, &vm.memory, 0b0110_010_000_000010);
        assert_eq!(vm.registers[Register::R2 as usize], 300);
    }

    // ========== Boundary Conditions ==========

    #[test]
    fn test_ldr_at_memory_boundary() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R7, 0xFFE0);
        vm.mem_write(0xFFFF, 0xDEAD);

        // LDR R0, R7, 31
        ldr(&mut vm.registers, &vm.memory, 0b0110_000_111_011111);

        assert_eq!(vm.registers[Register::R0 as usize], 0xDEAD);
    }

    #[test]
    fn test_ldr_from_address_zero() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x0005);
        vm.mem_write(0x0000, 0xBEEF);

        // LDR R2, R1, -5 (0x3B in 6-bit two's complement)
        ldr(&mut vm.registers, &vm.memory, 0b0110_010_001_111011);

        assert_eq!(vm.registers[Register::R2 as usize], 0xBEEF);
    }

    // ========== Pattern Tests ==========

    #[test]
    fn test_ldr_alternating_pattern() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R1, 0x3000);
        vm.mem_write(0x3001, 0xAAAA);

        // LDR R2, R1, 1
        ldr(&mut vm.registers, &vm.memory, 0b0110_010_001_000001);

        assert_eq!(vm.registers[Register::R2 as usize], 0xAAAA);
    }

    #[test]
    fn test_ldr_all_bits_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R3, 0x3000);
        vm.mem_write(0x3001, 0xFFFF);

        // LDR R4, R3, 1
        ldr(&mut vm.registers, &vm.memory, 0b0110_100_011_000001);

        assert_eq!(vm.registers[Register::R4 as usize], 0xFFFF);
    }

    // ========== Comparison with LD ==========

    #[test]
    fn test_ldr_vs_ld_difference() {
        let mut vm = Vm::new();
        // LDR uses base register + offset
        vm.write_to_register(Register::R1, 0x3000);
        vm.mem_write(0x3005, 42);

        // LDR R2, R1, 5
        ldr(&mut vm.registers, &vm.memory, 0b0110_010_001_000101);

        assert_eq!(vm.registers[Register::R2 as usize], 42);

        // LD would use PC + offset instead
        // This test just confirms LDR uses the base register
    }

    // ========== ASCII and Character Data ==========

    #[test]
    fn test_ldr_ascii_string() {
        let mut vm = Vm::new();
        let string_base = 0x3000;
        vm.write_to_register(Register::R0, string_base);

        // Store "ABC"
        vm.mem_write(string_base + 0, 0x0041); // 'A'
        vm.mem_write(string_base + 1, 0x0042); // 'B'
        vm.mem_write(string_base + 2, 0x0043); // 'C'

        // Load characters
        ldr(&mut vm.registers, &vm.memory, 0b0110_001_000_000000);
        assert_eq!(vm.registers[Register::R1 as usize], 0x0041);

        ldr(&mut vm.registers, &vm.memory, 0b0110_001_000_000001);
        assert_eq!(vm.registers[Register::R1 as usize], 0x0042);

        ldr(&mut vm.registers, &vm.memory, 0b0110_001_000_000010);
        assert_eq!(vm.registers[Register::R1 as usize], 0x0043);
    }
}
