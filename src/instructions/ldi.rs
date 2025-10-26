use crate::instructions::{sign_extend, update_flags};
use crate::registers::register::Register::{Count, Pc};
use crate::MEMORY_MAX;

pub fn ldi(
    registers: &mut [u16; (Count as u16) as usize],
    memory: &[u16; MEMORY_MAX],
    instruction: u16,
) {
    let destination_register = (instruction >> 9) & 0x7;
    let pc_offset_9 = sign_extend(instruction & 0x1FF, 9);

    let address_of_value_to_load =
        memory[registers[(Pc as u16) as usize].wrapping_add(pc_offset_9) as usize];
    registers[destination_register as usize] = memory[address_of_value_to_load as usize];
    update_flags(registers, destination_register)
}

#[cfg(test)]
mod tests {
    use crate::instructions::ldi::ldi;
    use crate::registers::register::Register;
    use crate::Vm;

    // ========== Basic Functionality Tests ==========

    #[test]
    fn should_load_value_with_zero_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.mem_write(0x3000, 0x4000); // Pointer at PC
        vm.mem_write(0x4000, 42); // Actual value

        // LDI R2, 0
        ldi(&mut vm.registers, &vm.memory, 0b1010_010_000000000);

        assert_eq!(vm.registers[Register::R2 as usize], 42);
    }

    #[test]
    fn should_load_value_with_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.mem_write(0x3005, 0x4000); // Pointer at PC + 5
        vm.mem_write(0x4000, 123); // Actual value

        // LDI R3, 5
        ldi(&mut vm.registers, &vm.memory, 0b1010_011_000000101);

        assert_eq!(vm.registers[Register::R3 as usize], 123);
    }

    #[test]
    fn should_load_value_with_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3010);
        vm.mem_write(0x3008, 0x5000); // Pointer at PC - 8
        vm.mem_write(0x5000, 99); // Actual value

        // LDI R1, -8 (offset = 0x1F8 in 9-bit two's complement)
        ldi(&mut vm.registers, &vm.memory, 0b1010_001_111111000);

        assert_eq!(vm.registers[Register::R1 as usize], 99);
    }

    #[test]
    fn should_load_value_with_max_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.mem_write(0x30FF, 0x4000); // Pointer at PC + 255
        vm.mem_write(0x4000, 77); // Actual value

        // LDI R4, 255 (max positive 9-bit offset)
        ldi(&mut vm.registers, &vm.memory, 0b1010_100_011111111);

        assert_eq!(vm.registers[Register::R4 as usize], 77);
    }

    #[test]
    fn should_load_value_with_max_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3100);
        vm.mem_write(0x3000, 0x4000); // Pointer at PC - 256
        vm.mem_write(0x4000, 88); // Actual value

        // LDI R5, -256 (max negative 9-bit offset)
        ldi(&mut vm.registers, &vm.memory, 0b1010_101_100000000);

        assert_eq!(vm.registers[Register::R5 as usize], 88);
    }

    // ========== Different Destination Registers ==========

    #[test]
    fn should_load_into_r0() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.mem_write(0x3000, 0x4000);
        vm.mem_write(0x4000, 11);

        // LDI R0, 0
        ldi(&mut vm.registers, &vm.memory, 0b1010_000_000000000);

        assert_eq!(vm.registers[Register::R0 as usize], 11);
    }

    #[test]
    fn should_load_into_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.mem_write(0x3000, 0x4000);
        vm.mem_write(0x4000, 22);

        // LDI R7, 0
        ldi(&mut vm.registers, &vm.memory, 0b1010_111_000000000);

        assert_eq!(vm.registers[Register::R7 as usize], 22);
    }

    // ========== Different Value Types ==========

    #[test]
    fn should_load_zero_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.mem_write(0x3000, 0x4000);
        vm.mem_write(0x4000, 0);

        // LDI R2, 0
        ldi(&mut vm.registers, &vm.memory, 0b1010_010_000000000);

        assert_eq!(vm.registers[Register::R2 as usize], 0);
    }

    #[test]
    fn should_load_max_positive_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.mem_write(0x3000, 0x4000);
        vm.mem_write(0x4000, 0x7FFF); // Max positive 16-bit signed value

        // LDI R3, 0
        ldi(&mut vm.registers, &vm.memory, 0b1010_011_000000000);

        assert_eq!(vm.registers[Register::R3 as usize], 0x7FFF);
    }

    #[test]
    fn should_load_negative_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.mem_write(0x3000, 0x4000);
        vm.mem_write(0x4000, 0xFFFF); // -1 in two's complement

        // LDI R4, 0
        ldi(&mut vm.registers, &vm.memory, 0b1010_100_000000000);

        assert_eq!(vm.registers[Register::R4 as usize], 0xFFFF);
    }

    #[test]
    fn should_load_max_unsigned_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.mem_write(0x3000, 0x4000);
        vm.mem_write(0x4000, 0xFFFF);

        // LDI R5, 0
        ldi(&mut vm.registers, &vm.memory, 0b1010_101_000000000);

        assert_eq!(vm.registers[Register::R5 as usize], 0xFFFF);
    }

    // ========== Chain of Indirection Tests ==========

    #[test]
    fn should_handle_pointer_to_low_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.mem_write(0x3000, 0x0010); // Pointer to low memory
        vm.mem_write(0x0010, 55); // Value in low memory

        // LDI R6, 0
        ldi(&mut vm.registers, &vm.memory, 0b1010_110_000000000);

        assert_eq!(vm.registers[Register::R6 as usize], 55);
    }

    #[test]
    fn should_handle_pointer_to_high_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.mem_write(0x3000, 0xFE00); // Pointer to high memory
        vm.mem_write(0xFE00, 66); // Value in high memory

        // LDI R1, 0
        ldi(&mut vm.registers, &vm.memory, 0b1010_001_000000000);

        assert_eq!(vm.registers[Register::R1 as usize], 66);
    }

    // ========== PC-Relative Tests ==========

    #[test]
    fn should_work_with_different_pc_values() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x5000);
        vm.mem_write(0x5003, 0x6000);
        vm.mem_write(0x6000, 111);

        // LDI R2, 3
        ldi(&mut vm.registers, &vm.memory, 0b1010_010_000000011);

        assert_eq!(vm.registers[Register::R2 as usize], 111);
    }

    #[test]
    fn should_handle_pc_at_zero() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0);
        vm.mem_write(0x0001, 0x1000);
        vm.mem_write(0x1000, 222);

        // LDI R3, 1
        ldi(&mut vm.registers, &vm.memory, 0b1010_011_000000001);

        assert_eq!(vm.registers[Register::R3 as usize], 222);
    }

    // ========== Overwrite Previous Value Tests ==========

    #[test]
    fn should_overwrite_existing_register_value() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::R2, 9999); // Pre-existing value
        vm.write_to_register(Register::Pc, 0x3000);
        vm.mem_write(0x3000, 0x4000);
        vm.mem_write(0x4000, 42);

        // LDI R2, 0
        ldi(&mut vm.registers, &vm.memory, 0b1010_010_000000000);

        assert_eq!(vm.registers[Register::R2 as usize], 42);
    }

    // ========== Complex Scenarios ==========

    #[test]
    fn should_load_multiple_values_sequentially() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // Setup first load
        vm.mem_write(0x3000, 0x4000);
        vm.mem_write(0x4000, 10);

        // Setup second load
        vm.mem_write(0x3001, 0x4100);
        vm.mem_write(0x4100, 20);

        // First LDI R1, 0
        ldi(&mut vm.registers, &vm.memory, 0b1010_001_000000000);
        assert_eq!(vm.registers[Register::R1 as usize], 10);

        // Second LDI R2, 1
        ldi(&mut vm.registers, &vm.memory, 0b1010_010_000000001);
        assert_eq!(vm.registers[Register::R2 as usize], 20);
    }

    #[test]
    fn should_handle_pointer_chain_at_same_location() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.mem_write(0x3000, 0x3000); // Pointer points to itself
        vm.mem_write(0x3000, 0x4000); // But actually contains a different pointer
        vm.mem_write(0x4000, 33);

        // This is weird but valid - the pointer value is what matters
        // LDI R4, 0
        ldi(&mut vm.registers, &vm.memory, 0b1010_100_000000000);

        assert_eq!(vm.registers[Register::R4 as usize], 33);
    }

    // ========== Negative Offset Edge Cases ==========

    #[test]
    fn should_handle_negative_offset_wrapping() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x0005);
        vm.mem_write(0x0000, 0x4000); // PC + (-5) = 0
        vm.mem_write(0x4000, 44);

        // LDI R5, -5 (0x1FB in 9-bit two's complement)
        ldi(&mut vm.registers, &vm.memory, 0b1010_101_111111011);

        assert_eq!(vm.registers[Register::R5 as usize], 44);
    }

    #[test]
    fn should_handle_offset_minus_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.mem_write(0x2FFF, 0x4000); // PC - 1
        vm.mem_write(0x4000, 55);

        // LDI R6, -1 (0x1FF in 9-bit two's complement)
        ldi(&mut vm.registers, &vm.memory, 0b1010_110_111111111);

        assert_eq!(vm.registers[Register::R6 as usize], 55);
    }
}
