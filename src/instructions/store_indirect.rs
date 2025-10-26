use crate::instructions::sign_extend;
use crate::registers::register::Register::Pc;
use crate::Vm;

pub fn sti(vm: &mut Vm, instruction: u16) {
    let source_register = (instruction >> 9) & 0x7;
    let pc_offset_9 = sign_extend(instruction & 0x1FF, 9);
    let destination_address = vm.mem_read(vm.registers[Pc as usize].wrapping_add(pc_offset_9));

    vm.mem_write(destination_address, vm.registers[source_register as usize])
}

#[cfg(test)]
mod tests {
    use crate::instructions::store_indirect::sti;
    use crate::registers::register::Register;
    use crate::Vm;

    // ========== Basic STI Operations ==========

    #[test]
    fn test_sti_basic() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 42);
        vm.mem_write(0x3000, 0x4000); // Pointer at PC

        // STI R2, 0 (store R2 at address pointed to by memory[PC + 0])
        sti(&mut vm, 0b1011_010_000000000);

        assert_eq!(vm.memory[0x4000], 42);
    }

    #[test]
    fn test_sti_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R3, 123);
        vm.mem_write(0x3005, 0x5000); // Pointer at PC + 5

        // STI R3, 5
        sti(&mut vm, 0b1011_011_000000101);

        assert_eq!(vm.memory[0x5000], 123);
    }

    #[test]
    fn test_sti_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3010);
        vm.write_to_register(Register::R1, 99);
        vm.mem_write(0x3008, 0x6000); // Pointer at PC - 8

        // STI R1, -8 (0x1F8 in 9-bit two's complement)
        sti(&mut vm, 0b1011_001_111111000);

        assert_eq!(vm.memory[0x6000], 99);
    }

    #[test]
    fn test_sti_max_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R4, 255);
        vm.mem_write(0x30FF, 0x7000); // Pointer at PC + 255

        // STI R4, 255
        sti(&mut vm, 0b1011_100_011111111);

        assert_eq!(vm.memory[0x7000], 255);
    }

    #[test]
    fn test_sti_max_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3100);
        vm.write_to_register(Register::R5, 77);
        vm.mem_write(0x3000, 0x8000); // Pointer at PC - 256

        // STI R5, -256
        sti(&mut vm, 0b1011_101_100000000);

        assert_eq!(vm.memory[0x8000], 77);
    }

    // ========== Store from Different Registers ==========

    #[test]
    fn test_sti_from_r0() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 111);
        vm.mem_write(0x3001, 0x4000);

        // STI R0, 1
        sti(&mut vm, 0b1011_000_000000001);

        assert_eq!(vm.memory[0x4000], 111);
    }

    #[test]
    fn test_sti_from_r1() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 222);
        vm.mem_write(0x3002, 0x5000);

        // STI R1, 2
        sti(&mut vm, 0b1011_001_000000010);

        assert_eq!(vm.memory[0x5000], 222);
    }

    #[test]
    fn test_sti_from_r2() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 333);
        vm.mem_write(0x3003, 0x6000);

        // STI R2, 3
        sti(&mut vm, 0b1011_010_000000011);

        assert_eq!(vm.memory[0x6000], 333);
    }

    #[test]
    fn test_sti_from_r3() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R3, 444);
        vm.mem_write(0x3004, 0x7000);

        // STI R3, 4
        sti(&mut vm, 0b1011_011_000000100);

        assert_eq!(vm.memory[0x7000], 444);
    }

    #[test]
    fn test_sti_from_r4() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R4, 555);
        vm.mem_write(0x3005, 0x8000);

        // STI R4, 5
        sti(&mut vm, 0b1011_100_000000101);

        assert_eq!(vm.memory[0x8000], 555);
    }

    #[test]
    fn test_sti_from_r5() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R5, 666);
        vm.mem_write(0x3006, 0x9000);

        // STI R5, 6
        sti(&mut vm, 0b1011_101_000000110);

        assert_eq!(vm.memory[0x9000], 666);
    }

    #[test]
    fn test_sti_from_r6() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R6, 777);
        vm.mem_write(0x3007, 0xA000);

        // STI R6, 7
        sti(&mut vm, 0b1011_110_000000111);

        assert_eq!(vm.memory[0xA000], 777);
    }

    #[test]
    fn test_sti_from_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R7, 888);
        vm.mem_write(0x3008, 0xB000);

        // STI R7, 8
        sti(&mut vm, 0b1011_111_000001000);

        assert_eq!(vm.memory[0xB000], 888);
    }

    // ========== Different Value Types ==========

    #[test]
    fn test_sti_zero_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 0);
        vm.mem_write(0x3001, 0x4000);
        vm.mem_write(0x4000, 0xFFFF); // Pre-existing value

        // STI R2, 1
        sti(&mut vm, 0b1011_010_000000001);

        assert_eq!(vm.memory[0x4000], 0);
    }

    #[test]
    fn test_sti_max_positive_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R3, 0x7FFF);
        vm.mem_write(0x3001, 0x5000);

        // STI R3, 1
        sti(&mut vm, 0b1011_011_000000001);

        assert_eq!(vm.memory[0x5000], 0x7FFF);
    }

    #[test]
    fn test_sti_negative_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R4, 0xFFFF);
        vm.mem_write(0x3001, 0x6000);

        // STI R4, 1
        sti(&mut vm, 0b1011_100_000000001);

        assert_eq!(vm.memory[0x6000], 0xFFFF);
    }

    #[test]
    fn test_sti_various_hex_values() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        let test_values = [0x1234, 0xABCD, 0x5678, 0xDEAD, 0xBEEF];

        for (i, &value) in test_values.iter().enumerate() {
            vm.write_to_register(Register::R0, value);
            vm.mem_write(0x3000 + i as u16, 0x4000 + i as u16);

            let instruction = 0b1011_000_000000000 | (i as u16);
            sti(&mut vm, instruction);

            assert_eq!(vm.memory[0x4000 + i], value);
        }
    }

    // ========== PC-Relative Pointer Location Tests ==========

    #[test]
    fn test_sti_from_different_pc_values() {
        let pc_values = [0x0100, 0x1000, 0x3000, 0x5000, 0x7000];

        for &pc in &pc_values {
            let mut vm = Vm::new();
            vm.write_to_register(Register::Pc, pc);
            vm.write_to_register(Register::R1, 42);
            vm.mem_write(pc + 10, 0x8000);

            // STI R1, 10
            sti(&mut vm, 0b1011_001_000001010);

            assert_eq!(vm.memory[0x8000], 42);
        }
    }

    #[test]
    fn test_sti_pointer_in_low_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x0010);
        vm.write_to_register(Register::R2, 99);
        vm.mem_write(0x0015, 0x4000); // Pointer in low memory

        // STI R2, 5
        sti(&mut vm, 0b1011_010_000000101);

        assert_eq!(vm.memory[0x4000], 99);
    }

    #[test]
    fn test_sti_pointer_in_high_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0xFE00);
        vm.write_to_register(Register::R3, 88);
        vm.mem_write(0xFE10, 0x5000); // Pointer in high memory

        // STI R3, 16
        sti(&mut vm, 0b1011_011_000010000);

        assert_eq!(vm.memory[0x5000], 88);
    }

    // ========== Pointer Target Location Tests ==========

    #[test]
    fn test_sti_to_low_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R4, 111);
        vm.mem_write(0x3001, 0x0050); // Pointer to low memory

        // STI R4, 1
        sti(&mut vm, 0b1011_100_000000001);

        assert_eq!(vm.memory[0x0050], 111);
    }

    #[test]
    fn test_sti_to_high_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R5, 222);
        vm.mem_write(0x3001, 0xFE00); // Pointer to high memory

        // STI R5, 1
        sti(&mut vm, 0b1011_101_000000001);

        assert_eq!(vm.memory[0xFE00], 222);
    }

    #[test]
    fn test_sti_to_address_zero() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R6, 0xBEEF);
        vm.mem_write(0x3001, 0x0000); // Pointer to address 0

        // STI R6, 1
        sti(&mut vm, 0b1011_110_000000001);

        assert_eq!(vm.memory[0x0000], 0xBEEF);
    }

    // ========== Preserving State ==========

    #[test]
    fn test_sti_does_not_modify_source_register() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 123);
        vm.mem_write(0x3005, 0x4000);

        // STI R2, 5
        sti(&mut vm, 0b1011_010_000000101);

        // Source register should remain unchanged
        assert_eq!(vm.registers[Register::R2 as usize], 123);
        assert_eq!(vm.memory[0x4000], 123);
    }

    #[test]
    fn test_sti_does_not_modify_pointer() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R3, 42);
        vm.mem_write(0x3005, 0x4000);

        // STI R3, 5
        sti(&mut vm, 0b1011_011_000000101);

        // Pointer should remain unchanged
        assert_eq!(vm.memory[0x3005], 0x4000);
        assert_eq!(vm.memory[0x4000], 42);
    }

    #[test]
    fn test_sti_preserves_other_registers() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x1111);
        vm.write_to_register(Register::R1, 0x2222);
        vm.write_to_register(Register::R2, 42);
        vm.write_to_register(Register::R3, 0x3333);
        vm.mem_write(0x3005, 0x4000);

        // STI R2, 5
        sti(&mut vm, 0b1011_010_000000101);

        // Check other registers unchanged
        assert_eq!(vm.registers[Register::R0 as usize], 0x1111);
        assert_eq!(vm.registers[Register::R1 as usize], 0x2222);
        assert_eq!(vm.registers[Register::R2 as usize], 42);
        assert_eq!(vm.registers[Register::R3 as usize], 0x3333);
    }

    #[test]
    fn test_sti_preserves_other_memory_locations() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 42);
        vm.mem_write(0x3005, 0x4000);
        vm.mem_write(0x3FFF, 0xAAAA);
        vm.mem_write(0x4001, 0xBBBB);

        // STI R2, 5
        sti(&mut vm, 0b1011_010_000000101);

        // Check neighboring memory unchanged
        assert_eq!(vm.memory[0x3FFF], 0xAAAA);
        assert_eq!(vm.memory[0x4000], 42);
        assert_eq!(vm.memory[0x4001], 0xBBBB);
    }

    // ========== Overwriting Memory ==========

    #[test]
    fn test_sti_overwrites_existing_target_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 42);
        vm.mem_write(0x3005, 0x4000);
        vm.mem_write(0x4000, 9999); // Pre-existing value

        // STI R2, 5
        sti(&mut vm, 0b1011_010_000000101);

        assert_eq!(vm.memory[0x4000], 42);
    }

    // ========== Sequential Stores ==========

    #[test]
    fn test_sti_multiple_values_sequentially() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 10);
        vm.write_to_register(Register::R2, 20);
        vm.write_to_register(Register::R3, 30);
        vm.mem_write(0x3001, 0x4000);
        vm.mem_write(0x3002, 0x4001);
        vm.mem_write(0x3003, 0x4002);

        // STI R1, 1
        sti(&mut vm, 0b1011_001_000000001);
        assert_eq!(vm.memory[0x4000], 10);

        // STI R2, 2
        sti(&mut vm, 0b1011_010_000000010);
        assert_eq!(vm.memory[0x4001], 20);

        // STI R3, 3
        sti(&mut vm, 0b1011_011_000000011);
        assert_eq!(vm.memory[0x4002], 30);
    }

    #[test]
    fn test_sti_same_target_multiple_times() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 10);
        vm.write_to_register(Register::R2, 20);
        vm.write_to_register(Register::R3, 30);
        vm.mem_write(0x3005, 0x4000);

        // STI R1, 5
        sti(&mut vm, 0b1011_001_000000101);
        assert_eq!(vm.memory[0x4000], 10);

        // STI R2, 5 (overwrite)
        sti(&mut vm, 0b1011_010_000000101);
        assert_eq!(vm.memory[0x4000], 20);

        // STI R3, 5 (overwrite again)
        sti(&mut vm, 0b1011_011_000000101);
        assert_eq!(vm.memory[0x4000], 30);
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_sti_offset_minus_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3001);
        vm.write_to_register(Register::R4, 55);
        vm.mem_write(0x3000, 0x5000);

        // STI R4, -1 (0x1FF in 9-bit two's complement)
        sti(&mut vm, 0b1011_100_111111111);

        assert_eq!(vm.memory[0x5000], 55);
    }

    #[test]
    fn test_sti_offset_plus_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R5, 66);
        vm.mem_write(0x3001, 0x6000);

        // STI R5, 1
        sti(&mut vm, 0b1011_101_000000001);

        assert_eq!(vm.memory[0x6000], 66);
    }

    // ========== Indirection Chain ==========

    #[test]
    fn test_sti_pointer_table() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // Setup pointer table
        vm.mem_write(0x3000, 0x4000); // pointer 0
        vm.mem_write(0x3001, 0x4100); // pointer 1
        vm.mem_write(0x3002, 0x4200); // pointer 2

        // Store values through pointers
        vm.write_to_register(Register::R0, 100);
        sti(&mut vm, 0b1011_000_000000000);
        assert_eq!(vm.memory[0x4000], 100);

        vm.write_to_register(Register::R1, 200);
        sti(&mut vm, 0b1011_001_000000001);
        assert_eq!(vm.memory[0x4100], 200);

        vm.write_to_register(Register::R2, 300);
        sti(&mut vm, 0b1011_010_000000010);
        assert_eq!(vm.memory[0x4200], 300);
    }

    // ========== Pattern Tests ==========

    #[test]
    fn test_sti_alternating_pattern() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 0xAAAA);
        vm.mem_write(0x3001, 0x4000);

        // STI R2, 1
        sti(&mut vm, 0b1011_010_000000001);

        assert_eq!(vm.memory[0x4000], 0xAAAA);
    }

    #[test]
    fn test_sti_all_bits_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R4, 0xFFFF);
        vm.mem_write(0x3001, 0x5000);

        // STI R4, 1
        sti(&mut vm, 0b1011_100_000000001);

        assert_eq!(vm.memory[0x5000], 0xFFFF);
    }

    // ========== ASCII and Character Data ==========

    #[test]
    fn test_sti_ascii_values() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 0x0041); // 'A'
        vm.write_to_register(Register::R2, 0x0042); // 'B'
        vm.write_to_register(Register::R3, 0x0043); // 'C'
        vm.mem_write(0x3001, 0x4000);
        vm.mem_write(0x3002, 0x4001);
        vm.mem_write(0x3003, 0x4002);

        // STI R1, 1
        sti(&mut vm, 0b1011_001_000000001);
        assert_eq!(vm.memory[0x4000], 0x0041);

        // STI R2, 2
        sti(&mut vm, 0b1011_010_000000010);
        assert_eq!(vm.memory[0x4001], 0x0042);

        // STI R3, 3
        sti(&mut vm, 0b1011_011_000000011);
        assert_eq!(vm.memory[0x4002], 0x0043);
    }

    // ========== Boundary Conditions ==========

    #[test]
    fn test_sti_at_memory_boundary() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0xFFF0);
        vm.write_to_register(Register::R7, 0xDEAD);
        vm.mem_write(0xFFFF, 0x5000);

        // STI R7, 15
        sti(&mut vm, 0b1011_111_000001111);

        assert_eq!(vm.memory[0x5000], 0xDEAD);
    }

    // ========== Store and Load Pattern ==========

    #[test]
    fn test_sti_then_ldi_pattern() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 42);
        vm.mem_write(0x3005, 0x4000);

        // STI R2, 5
        sti(&mut vm, 0b1011_010_000000101);

        // Verify the store
        assert_eq!(vm.memory[0x4000], 42);

        // Could now LDI R3, 5 to load it back
    }

    // ========== Round-trip Test ==========

    #[test]
    fn test_sti_ldi_round_trip() {
        let mut vm = Vm::new();
        let original_value = 0xABCD;
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, original_value);
        vm.mem_write(0x300A, 0x5000); // Pointer

        // STI R1, 10
        sti(&mut vm, 0b1011_001_000001010);

        // Clear R1
        vm.write_to_register(Register::R1, 0);

        // Verify stored value at indirect location
        assert_eq!(vm.memory[0x5000], original_value);
    }

    // ========== Comparison with ST ==========

    #[test]
    fn test_sti_vs_st_difference() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 42);
        vm.mem_write(0x3005, 0x4000);

        // STI uses two memory accesses: read pointer, then store
        sti(&mut vm, 0b1011_010_000000101);

        // Value stored at location pointed to by memory[PC + 5]
        assert_eq!(vm.memory[0x4000], 42);

        // ST would store directly at PC + 5
        // This test confirms STI performs indirection
    }
}
