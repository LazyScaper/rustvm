use crate::registers::register::Register::{Count, Pc, R0, R7};
use crate::MEMORY_MAX;
use std::io;
use std::io::Write;

const TRAP_GETC: u16 = 0x20; /* get character from keyboard, not echoed onto the terminal */
const TRAP_OUT: u16 = 0x21; /* output a character */
const TRAP_PUTS: u16 = 0x22; /* output a word string */
const TRAP_IN: u16 = 0x23; /* get character from keyboard, echoed onto the terminal */
const TRAP_PUTSP: u16 = 0x24; /* output a byte string */
const TRAP_HALT: u16 = 0x25; /* halt the program */

pub fn trap(
    registers: &mut [u16; (Count as u16) as usize],
    memory: &[u16; MEMORY_MAX],
    instruction: u16,
) {
    registers[R7 as usize] = registers[Pc as usize];

    match instruction & 0xFF {
        TRAP_GETC => {}
        TRAP_OUT => {}
        TRAP_PUTS => {
            let mut memory_address = registers[R0 as usize];
            loop {
                let character = memory[memory_address as usize];
                if character == 0x0000 {
                    break;
                }
                let c = (character & 0xFF) as u8;
                print!("{}", c as char);
                memory_address += 1;
            }
            io::stdout().flush().expect("Could not flush stdout");
        }
        TRAP_IN => {}
        TRAP_PUTSP => {}
        TRAP_HALT => {}
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::trap::trap;
    use crate::registers::register::Register;
    use crate::Vm;

    // ========== Basic TRAP Operations ==========

    #[test]
    fn test_trap_saves_pc_to_r7() {
        let mut vm = Vm::new();
        let original_pc = 0x3000;
        vm.write_to_register(Register::Pc, original_pc);

        // TRAP x20 (GETC)
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100000);

        // R7 should contain the return address (original PC)
        assert_eq!(vm.registers[Register::R7 as usize], original_pc);
        // Note: PC doesn't change with native trap implementation
    }

    #[test]
    fn test_trap_overwrites_previous_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R7, 0xDEAD); // Pre-existing value

        // TRAP x20
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100000);

        // R7 should be overwritten with return address
        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
    }

    // ========== Common Trap Vectors ==========

    #[test]
    fn test_trap_getc() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // TRAP x20 (GETC) - native implementation
        // Note: Can't easily test stdin reading in unit tests
        // Just verify R7 is saved
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100000);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
    }

    #[test]
    fn test_trap_out() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x0041); // 'A'

        // TRAP x21 (OUT) - outputs character in R0
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100001);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        // R0 should be unchanged
        assert_eq!(vm.registers[Register::R0 as usize], 0x0041);
    }

    #[test]
    fn test_trap_puts() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x4000); // Pointer to string

        // Setup a string in memory: "HI" + null terminator
        vm.write_to_memory(0x4000, 0x0048); // 'H'
        vm.write_to_memory(0x4001, 0x0049); // 'I'
        vm.write_to_memory(0x4002, 0x0000); // null terminator

        // TRAP x22 (PUTS)
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100010);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        // R0 should still point to string
        assert_eq!(vm.registers[Register::R0 as usize], 0x4000);
    }

    #[test]
    fn test_trap_in() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // TRAP x23 (IN) - native implementation
        // Note: Can't easily test stdin reading in unit tests
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100011);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
    }

    #[test]
    fn test_trap_putsp() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x4000); // Pointer to packed string

        // Setup packed string: "AB" in one word
        vm.write_to_memory(0x4000, 0x4241); // 'A' (0x41) in low byte, 'B' (0x42) in high byte
        vm.write_to_memory(0x4001, 0x0000); // null terminator

        // TRAP x24 (PUTSP)
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100100);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
    }

    #[test]
    fn test_trap_halt() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // TRAP x25 (HALT)
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100101);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
    }

    // ========== Different Trap Vectors ==========

    #[test]
    fn test_trap_vector_x00() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // TRAP x00 (user-defined or reserved)
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00000000);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
    }

    #[test]
    fn test_trap_vector_x01() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // TRAP x01
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00000001);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
    }

    #[test]
    fn test_trap_vector_xff() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // TRAP xFF (max trap vector)
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_11111111);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
    }

    // ========== PC Values ==========

    #[test]
    fn test_trap_from_different_pc_values() {
        let pc_values = [0x0100, 0x1000, 0x3000, 0x5000, 0x7000, 0xA000];

        for &pc in &pc_values {
            let mut vm = Vm::new();
            vm.write_to_register(Register::Pc, pc);

            // TRAP x25
            trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100101);

            assert_eq!(vm.registers[Register::R7 as usize], pc);
        }
    }

    #[test]
    fn test_trap_from_low_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x0200);

        // TRAP x20
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100000);

        assert_eq!(vm.registers[Register::R7 as usize], 0x0200);
    }

    #[test]
    fn test_trap_from_high_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0xF000);

        // TRAP x20
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100000);

        assert_eq!(vm.registers[Register::R7 as usize], 0xF000);
    }

    // ========== Preserving Other Registers ==========

    #[test]
    fn test_trap_preserves_r0_through_r6() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x1111);
        vm.write_to_register(Register::R1, 0x2222);
        vm.write_to_register(Register::R2, 0x3333);
        vm.write_to_register(Register::R3, 0x4444);
        vm.write_to_register(Register::R4, 0x5555);
        vm.write_to_register(Register::R5, 0x6666);
        vm.write_to_register(Register::R6, 0x7777);

        // TRAP x25 (HALT doesn't modify registers)
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100101);

        // Check all registers R0-R6 unchanged
        assert_eq!(vm.registers[Register::R0 as usize], 0x1111);
        assert_eq!(vm.registers[Register::R1 as usize], 0x2222);
        assert_eq!(vm.registers[Register::R2 as usize], 0x3333);
        assert_eq!(vm.registers[Register::R3 as usize], 0x4444);
        assert_eq!(vm.registers[Register::R4 as usize], 0x5555);
        assert_eq!(vm.registers[Register::R5 as usize], 0x6666);
        assert_eq!(vm.registers[Register::R6 as usize], 0x7777);
    }

    // ========== Sequential Traps ==========

    #[test]
    fn test_sequential_traps() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // TRAP x20
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100000);
        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);

        // Update PC as if continuing execution
        vm.write_to_register(Register::Pc, 0x3010);

        // TRAP x21
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100001);
        assert_eq!(vm.registers[Register::R7 as usize], 0x3010);
    }

    #[test]
    fn test_nested_traps_overwrite_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // First TRAP
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100000);
        let first_return = vm.registers[Register::R7 as usize];
        assert_eq!(first_return, 0x3000);

        // Second TRAP (nested - would overwrite R7)
        vm.write_to_register(Register::Pc, 0x0410);
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100001);
        let second_return = vm.registers[Register::R7 as usize];
        assert_eq!(second_return, 0x0410);
    }

    // ========== Trap Vector Identification ==========

    #[test]
    fn test_trap_vector_extraction() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // Test that different trap vectors save R7 correctly
        let trap_vectors = [0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x30, 0xFF];

        for &vector in &trap_vectors {
            vm.write_to_register(Register::Pc, 0x3000);
            vm.write_to_register(Register::R7, 0); // Clear R7

            let instruction = 0b1111_0000_00000000 | vector;
            trap(&mut vm.registers, &vm.memory, instruction);

            assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        }
    }

    // ========== Trap and Return Pattern ==========

    #[test]
    fn test_trap_and_ret_pattern() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // TRAP x25
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100101);

        let return_address = vm.registers[Register::R7 as usize];
        assert_eq!(return_address, 0x3000);

        // Simulate RET (JMP R7) would return to 0x3000
        assert_eq!(return_address, 0x3000);
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_trap_with_all_zeros() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0);

        // TRAP x00
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00000000);

        assert_eq!(vm.registers[Register::R7 as usize], 0);
    }

    // ========== Specific Trap Functionality ==========

    #[test]
    fn test_trap_puts_with_empty_string() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x4000); // Pointer to string

        // Empty string (just null terminator)
        vm.write_to_memory(0x4000, 0x0000);

        // TRAP x22 (PUTS)
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100010);

        // R0 should still point to string
        assert_eq!(vm.registers[Register::R0 as usize], 0x4000);
    }

    #[test]
    fn test_trap_puts_with_long_string() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x4000); // Pointer to string

        // "HELLO" + null
        vm.write_to_memory(0x4000, 0x0048); // 'H'
        vm.write_to_memory(0x4001, 0x0045); // 'E'
        vm.write_to_memory(0x4002, 0x004C); // 'L'
        vm.write_to_memory(0x4003, 0x004C); // 'L'
        vm.write_to_memory(0x4004, 0x004F); // 'O'
        vm.write_to_memory(0x4005, 0x0000); // null

        // TRAP x22 (PUTS)
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100010);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
    }

    #[test]
    fn test_trap_putsp_with_packed_string() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x4000);

        // "ABCD" packed: 'BA' and 'DC'
        vm.write_to_memory(0x4000, 0x4241); // 'A' (0x41), 'B' (0x42)
        vm.write_to_memory(0x4001, 0x4443); // 'C' (0x43), 'D' (0x44)
        vm.write_to_memory(0x4002, 0x0000); // null

        // TRAP x24 (PUTSP)
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100100);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
    }

    // ========== User-Defined Traps ==========

    #[test]
    fn test_user_defined_trap() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // TRAP x30 (user-defined)
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00110000);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
    }

    #[test]
    fn test_multiple_user_traps() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // Test multiple user-defined traps
        for i in 0x30..=0x35 {
            vm.write_to_register(Register::Pc, 0x3000);
            vm.write_to_register(Register::R7, 0); // Clear R7

            let instruction = 0b1111_0000_00000000 | i;
            trap(&mut vm.registers, &vm.memory, instruction);

            assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        }
    }

    // ========== Boundary Trap Vectors ==========

    #[test]
    fn test_trap_vector_at_boundary() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // TRAP xFF (last possible trap vector)
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_11111111);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
    }

    // ========== Register Preservation Tests ==========

    #[test]
    fn test_trap_getc_preserves_other_registers() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 0x1111);
        vm.write_to_register(Register::R2, 0x2222);

        // TRAP x20 (GETC) - modifies R0 but should preserve others
        trap(&mut vm.registers, &vm.memory, 0b1111_0000_00100000);

        assert_eq!(vm.registers[Register::R1 as usize], 0x1111);
        assert_eq!(vm.registers[Register::R2 as usize], 0x2222);
    }
}
