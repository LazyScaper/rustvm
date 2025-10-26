use crate::instructions::{sign_extend, update_flags};
use crate::registers::register::Register::{Count, Pc};

pub fn lea(registers: &mut [u16; (Count as u16) as usize], instruction: u16) {
    let destination_register = (instruction >> 9) & 0x7;
    let pc_offset_9 = sign_extend(instruction & 0x1FF, 9);

    registers[destination_register as usize] = registers[Pc as usize].wrapping_add(pc_offset_9);

    update_flags(registers, destination_register);
}

#[cfg(test)]
mod tests {
    use crate::instructions::load_effective::lea;
    use crate::registers::register::Register;
    use crate::Vm;

    // ========== Basic LEA Operations ==========

    #[test]
    fn test_lea_basic_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R2, 5 (R2 = PC + 5)
        lea(&mut vm.registers, 0b1110_010_000000101);

        assert_eq!(vm.registers[Register::R2 as usize], 0x3005);
    }

    #[test]
    fn test_lea_zero_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R3, 0 (R3 = PC)
        lea(&mut vm.registers, 0b1110_011_000000000);

        assert_eq!(vm.registers[Register::R3 as usize], 0x3000);
    }

    #[test]
    fn test_lea_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3010);

        // LEA R1, -8 (0x1F8 in 9-bit two's complement)
        lea(&mut vm.registers, 0b1110_001_111111000);

        assert_eq!(vm.registers[Register::R1 as usize], 0x3008);
    }

    #[test]
    fn test_lea_max_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R4, 255 (max positive 9-bit offset)
        lea(&mut vm.registers, 0b1110_100_011111111);

        assert_eq!(vm.registers[Register::R4 as usize], 0x30FF);
    }

    #[test]
    fn test_lea_max_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3100);

        // LEA R5, -256 (max negative 9-bit offset)
        lea(&mut vm.registers, 0b1110_101_100000000);

        assert_eq!(vm.registers[Register::R5 as usize], 0x3000);
    }

    // ========== Load Address to Different Registers ==========

    #[test]
    fn test_lea_to_r0() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R0, 10
        lea(&mut vm.registers, 0b1110_000_000001010);

        assert_eq!(vm.registers[Register::R0 as usize], 0x300A);
    }

    #[test]
    fn test_lea_to_r1() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R1, 20
        lea(&mut vm.registers, 0b1110_001_000010100);

        assert_eq!(vm.registers[Register::R1 as usize], 0x3014);
    }

    #[test]
    fn test_lea_to_r2() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R2, 30
        lea(&mut vm.registers, 0b1110_010_000011110);

        assert_eq!(vm.registers[Register::R2 as usize], 0x301E);
    }

    #[test]
    fn test_lea_to_r3() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R3, 40
        lea(&mut vm.registers, 0b1110_011_000101000);

        assert_eq!(vm.registers[Register::R3 as usize], 0x3028);
    }

    #[test]
    fn test_lea_to_r4() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R4, 50
        lea(&mut vm.registers, 0b1110_100_000110010);

        assert_eq!(vm.registers[Register::R4 as usize], 0x3032);
    }

    #[test]
    fn test_lea_to_r5() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R5, 60
        lea(&mut vm.registers, 0b1110_101_000111100);

        assert_eq!(vm.registers[Register::R5 as usize], 0x303C);
    }

    #[test]
    fn test_lea_to_r6() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R6, 70
        lea(&mut vm.registers, 0b1110_110_001000110);

        assert_eq!(vm.registers[Register::R6 as usize], 0x3046);
    }

    #[test]
    fn test_lea_to_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R7, 80
        lea(&mut vm.registers, 0b1110_111_001010000);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3050);
    }

    // ========== PC-Relative Address Calculation ==========

    #[test]
    fn test_lea_from_different_pc_values() {
        let pc_values = [0x0100, 0x1000, 0x3000, 0x5000, 0x7000, 0xA000];

        for &pc in &pc_values {
            let mut vm = Vm::new();
            vm.write_to_register(Register::Pc, pc);

            // LEA R1, 10
            lea(&mut vm.registers, 0b1110_001_000001010);

            assert_eq!(vm.registers[Register::R1 as usize], pc + 10);
        }
    }

    #[test]
    fn test_lea_from_low_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x0010);

        // LEA R2, 5
        lea(&mut vm.registers, 0b1110_010_000000101);

        assert_eq!(vm.registers[Register::R2 as usize], 0x0015);
    }

    #[test]
    fn test_lea_from_high_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0xFE00);

        // LEA R3, 16
        lea(&mut vm.registers, 0b1110_011_000010000);

        assert_eq!(vm.registers[Register::R3 as usize], 0xFE10);
    }

    // ========== Address Arithmetic ==========

    #[test]
    fn test_lea_forward_address() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R1, 100
        lea(&mut vm.registers, 0b1110_001_001100100);

        assert_eq!(vm.registers[Register::R1 as usize], 0x3064);
    }

    #[test]
    fn test_lea_backward_address() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3050);

        // LEA R2, -80 (0x1B0 in 9-bit two's complement)
        lea(&mut vm.registers, 0b1110_010_110110000);

        assert_eq!(vm.registers[Register::R2 as usize], 0x3000);
    }

    #[test]
    fn test_lea_offset_plus_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R3, 1
        lea(&mut vm.registers, 0b1110_011_000000001);

        assert_eq!(vm.registers[Register::R3 as usize], 0x3001);
    }

    #[test]
    fn test_lea_offset_minus_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3001);

        // LEA R4, -1 (0x1FF in 9-bit two's complement)
        lea(&mut vm.registers, 0b1110_100_111111111);

        assert_eq!(vm.registers[Register::R4 as usize], 0x3000);
    }

    // ========== Overwriting Previous Values ==========

    #[test]
    fn test_lea_overwrites_existing_register_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 0x9999); // Pre-existing value

        // LEA R2, 5
        lea(&mut vm.registers, 0b1110_010_000000101);

        assert_eq!(vm.registers[Register::R2 as usize], 0x3005);
    }

    #[test]
    fn test_lea_preserves_other_registers() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x1111);
        vm.write_to_register(Register::R1, 0x2222);
        vm.write_to_register(Register::R3, 0x3333);

        // LEA R2, 5
        lea(&mut vm.registers, 0b1110_010_000000101);

        // Check other registers unchanged
        assert_eq!(vm.registers[Register::R0 as usize], 0x1111);
        assert_eq!(vm.registers[Register::R1 as usize], 0x2222);
        assert_eq!(vm.registers[Register::R2 as usize], 0x3005);
        assert_eq!(vm.registers[Register::R3 as usize], 0x3333);
    }

    // ========== Sequential LEA Operations ==========

    #[test]
    fn test_lea_multiple_addresses_sequentially() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R1, 10
        lea(&mut vm.registers, 0b1110_001_000001010);
        assert_eq!(vm.registers[Register::R1 as usize], 0x300A);

        // LEA R2, 20
        lea(&mut vm.registers, 0b1110_010_000010100);
        assert_eq!(vm.registers[Register::R2 as usize], 0x3014);

        // LEA R3, 30
        lea(&mut vm.registers, 0b1110_011_000011110);
        assert_eq!(vm.registers[Register::R3 as usize], 0x301E);
    }

    #[test]
    fn test_lea_same_register_multiple_times() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R1, 5
        lea(&mut vm.registers, 0b1110_001_000000101);
        assert_eq!(vm.registers[Register::R1 as usize], 0x3005);

        // Update PC and LEA R1 again
        vm.write_to_register(Register::Pc, 0x4000);
        lea(&mut vm.registers, 0b1110_001_000000101);
        assert_eq!(vm.registers[Register::R1 as usize], 0x4005);
    }

    // ========== Use Cases ==========

    #[test]
    fn test_lea_for_data_address() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R1, 100 (load address of data section)
        lea(&mut vm.registers, 0b1110_001_001100100);

        // R1 now contains address 0x3064, could be used for LD/ST
        assert_eq!(vm.registers[Register::R1 as usize], 0x3064);
    }

    #[test]
    fn test_lea_for_string_address() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R2, 50 (load address of string)
        lea(&mut vm.registers, 0b1110_010_000110010);

        // R2 now contains address 0x3032, useful for string operations
        assert_eq!(vm.registers[Register::R2 as usize], 0x3032);
    }

    #[test]
    fn test_lea_for_array_base() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R3, 200 (load base address of array)
        lea(&mut vm.registers, 0b1110_011_011001000);

        // R3 now contains 0x30C8, can be used for array indexing
        assert_eq!(vm.registers[Register::R3 as usize], 0x30C8);
    }

    #[test]
    fn test_lea_for_subroutine_address() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R4, 150 (load address of subroutine)
        lea(&mut vm.registers, 0b1110_100_010010110);

        // R4 now contains 0x3096, could be used with JSRR
        assert_eq!(vm.registers[Register::R4 as usize], 0x3096);
    }

    // ========== Boundary Conditions ==========

    #[test]
    fn test_lea_to_address_zero() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x0005);

        // LEA R5, -5 (0x1FB in 9-bit two's complement)
        lea(&mut vm.registers, 0b1110_101_111111011);

        assert_eq!(vm.registers[Register::R5 as usize], 0x0000);
    }

    #[test]
    fn test_lea_near_memory_boundary() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0xFFF0);

        // LEA R6, 15
        lea(&mut vm.registers, 0b1110_110_000001111);

        assert_eq!(vm.registers[Register::R6 as usize], 0xFFFF);
    }

    #[test]
    fn test_lea_wraparound() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0xFFFF);

        // LEA R7, 1 (should wrap around)
        lea(&mut vm.registers, 0b1110_111_000000001);

        assert_eq!(vm.registers[Register::R7 as usize], 0x0000);
    }

    // ========== LEA vs LD Difference ==========

    #[test]
    fn test_lea_loads_address_not_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.mem_write(0x3005, 0xABCD); // Value in memory

        // LEA R1, 5 (loads ADDRESS 0x3005, not value 0xABCD)
        lea(&mut vm.registers, 0b1110_001_000000101);

        assert_eq!(vm.registers[Register::R1 as usize], 0x3005);
        // NOT 0xABCD - LEA doesn't access memory
    }

    // ========== Pointer Setup Pattern ==========

    #[test]
    fn test_lea_for_pointer_setup() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R1, 10 (get address)
        lea(&mut vm.registers, 0b1110_001_000001010);

        // R1 now contains 0x300A, can be used as base for LDR/STR
        assert_eq!(vm.registers[Register::R1 as usize], 0x300A);
    }

    #[test]
    fn test_lea_multiple_pointers() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // Setup multiple pointers
        lea(&mut vm.registers, 0b1110_001_000001010); // R1 = 0x300A
        lea(&mut vm.registers, 0b1110_010_000010100); // R2 = 0x3014
        lea(&mut vm.registers, 0b1110_011_000011110); // R3 = 0x301E

        assert_eq!(vm.registers[Register::R1 as usize], 0x300A);
        assert_eq!(vm.registers[Register::R2 as usize], 0x3014);
        assert_eq!(vm.registers[Register::R3 as usize], 0x301E);
    }

    // ========== Address Table Setup ==========

    #[test]
    fn test_lea_address_table() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // Create address table
        let offsets = [10, 20, 30, 40, 50];

        for (i, &offset) in offsets.iter().enumerate() {
            let instruction = 0b1110_000_000000000 | (offset as u16);
            lea(&mut vm.registers, instruction);

            assert_eq!(vm.registers[Register::R0 as usize], 0x3000 + offset);
        }
    }

    // ========== No Memory Access ==========

    #[test]
    fn test_lea_does_not_access_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // Memory at target address doesn't matter
        vm.mem_write(0x3010, 0xDEAD);

        // LEA R1, 16
        lea(&mut vm.registers, 0b1110_001_000010000);

        // R1 contains address, not memory value
        assert_eq!(vm.registers[Register::R1 as usize], 0x3010);

        // Memory unchanged
        assert_eq!(vm.memory[0x3010], 0xDEAD);
    }

    // ========== Large Offsets ==========

    #[test]
    fn test_lea_large_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R1, 250
        lea(&mut vm.registers, 0b1110_001_011111010);

        assert_eq!(vm.registers[Register::R1 as usize], 0x30FA);
    }

    #[test]
    fn test_lea_large_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3100);

        // LEA R2, -250 (0x106 in 9-bit two's complement)
        lea(&mut vm.registers, 0b1110_010_100000110);

        assert_eq!(vm.registers[Register::R2 as usize], 0x3006);
    }

    // ========== Practical Scenarios ==========

    #[test]
    fn test_lea_before_loop() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R1, 100 (load base address before loop)
        lea(&mut vm.registers, 0b1110_001_001100100);

        // Now R1 can be used with LDR/STR in loop with different offsets
        assert_eq!(vm.registers[Register::R1 as usize], 0x3064);
    }

    #[test]
    fn test_lea_for_relative_addressing() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // LEA R2, 5 (relative to current PC)
        lea(&mut vm.registers, 0b1110_010_000000101);
        assert_eq!(vm.registers[Register::R2 as usize], 0x3005);

        // Change PC, LEA gives different result
        vm.write_to_register(Register::Pc, 0x4000);
        lea(&mut vm.registers, 0b1110_010_000000101);
        assert_eq!(vm.registers[Register::R2 as usize], 0x4005);
    }

    // ========== Sign Extension Test ==========

    #[test]
    fn test_lea_sign_extension() {
        let mut vm = Vm::new();

        // Test various offsets to ensure proper sign extension
        let test_cases = [
            (0b000000001, 0x3001), // +1
            (0b011111111, 0x30FF), // +255
            (0b111111111, 0x2FFF), // -1
            (0b100000000, 0x2F00), // -256 (was 0x3000, should be 0x2F00)
        ];

        for (offset, expected) in test_cases.iter() {
            vm.write_to_register(Register::Pc, 0x3000);
            let instruction = 0b1110_001_000000000 | offset;
            lea(&mut vm.registers, instruction);
            assert_eq!(vm.registers[Register::R1 as usize], *expected);
        }
    }

    // ========== PC Independence ==========

    #[test]
    fn test_lea_result_depends_on_pc() {
        let mut vm = Vm::new();

        // Same instruction, different PC values
        let instruction = 0b1110_001_000001010; // LEA R1, 10

        vm.write_to_register(Register::Pc, 0x1000);
        lea(&mut vm.registers, instruction);
        assert_eq!(vm.registers[Register::R1 as usize], 0x100A);

        vm.write_to_register(Register::Pc, 0x2000);
        lea(&mut vm.registers, instruction);
        assert_eq!(vm.registers[Register::R1 as usize], 0x200A);

        vm.write_to_register(Register::Pc, 0x3000);
        lea(&mut vm.registers, instruction);
        assert_eq!(vm.registers[Register::R1 as usize], 0x300A);
    }
}
