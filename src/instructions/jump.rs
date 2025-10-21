use crate::registers::register::Register::{Count, Pc};

pub fn jmp(registers: &mut [u16; (Count as u16) as usize], instruction: u16) {
    let base_register = (instruction >> 6) & 0x7;

    registers[Pc as usize] = registers[base_register as usize];
}

#[cfg(test)]
mod tests {
    use crate::instructions::jump::jmp;
    use crate::registers::register::Register;
    use crate::Vm;

    // ========== Basic JMP Operations ==========

    #[test]
    fn test_jmp_basic() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 0x4000);

        // JMP R1 (PC = R1)
        jmp(&mut vm.registers, 0b1100_000_001_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x4000);
    }

    #[test]
    fn test_jmp_to_low_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x5000);
        vm.write_to_register(Register::R2, 0x0100);

        // JMP R2
        jmp(&mut vm.registers, 0b1100_000_010_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x0100);
    }

    #[test]
    fn test_jmp_to_high_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R3, 0xFE00);

        // JMP R3
        jmp(&mut vm.registers, 0b1100_000_011_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0xFE00);
    }

    #[test]
    fn test_jmp_to_zero() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R4, 0x0000);

        // JMP R4
        jmp(&mut vm.registers, 0b1100_000_100_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x0000);
    }

    #[test]
    fn test_jmp_to_max_address() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R5, 0xFFFF);

        // JMP R5
        jmp(&mut vm.registers, 0b1100_000_101_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0xFFFF);
    }

    // ========== JMP from All Registers ==========

    #[test]
    fn test_jmp_from_r0() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x4000);

        // JMP R0
        jmp(&mut vm.registers, 0b1100_000_000_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x4000);
    }

    #[test]
    fn test_jmp_from_r1() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 0x5000);

        // JMP R1
        jmp(&mut vm.registers, 0b1100_000_001_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x5000);
    }

    #[test]
    fn test_jmp_from_r2() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 0x6000);

        // JMP R2
        jmp(&mut vm.registers, 0b1100_000_010_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x6000);
    }

    #[test]
    fn test_jmp_from_r3() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R3, 0x7000);

        // JMP R3
        jmp(&mut vm.registers, 0b1100_000_011_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x7000);
    }

    #[test]
    fn test_jmp_from_r4() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R4, 0x8000);

        // JMP R4
        jmp(&mut vm.registers, 0b1100_000_100_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x8000);
    }

    #[test]
    fn test_jmp_from_r5() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R5, 0x9000);

        // JMP R5
        jmp(&mut vm.registers, 0b1100_000_101_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x9000);
    }

    #[test]
    fn test_jmp_from_r6() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R6, 0xA000);

        // JMP R6
        jmp(&mut vm.registers, 0b1100_000_110_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0xA000);
    }

    #[test]
    fn test_jmp_from_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R7, 0xB000);

        // JMP R7 (this is also RET when R7 is used)
        jmp(&mut vm.registers, 0b1100_000_111_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0xB000);
    }

    // ========== RET (Special case: JMP R7) ==========

    #[test]
    fn test_ret_is_jmp_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R7, 0x3050); // Return address

        // RET is encoded as JMP R7
        jmp(&mut vm.registers, 0b1100_000_111_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3050);
    }

    #[test]
    fn test_ret_after_jsr() {
        let mut vm = Vm::new();
        let return_address = 0x3005;
        vm.write_to_register(Register::Pc, 0x4000); // Currently in subroutine
        vm.write_to_register(Register::R7, return_address); // Saved by JSR

        // RET
        jmp(&mut vm.registers, 0b1100_000_111_000000);

        assert_eq!(vm.registers[Register::Pc as usize], return_address);
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_jmp_infinite_loop() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x3000); // Jump to self

        // JMP R0 (infinite loop)
        jmp(&mut vm.registers, 0b1100_000_000_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3000);
    }

    #[test]
    fn test_jmp_preserves_other_registers() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x1111);
        vm.write_to_register(Register::R1, 0x4000);
        vm.write_to_register(Register::R2, 0x2222);

        // JMP R1
        jmp(&mut vm.registers, 0b1100_000_001_000000);

        // Check PC changed
        assert_eq!(vm.registers[Register::Pc as usize], 0x4000);

        // Check other registers unchanged
        assert_eq!(vm.registers[Register::R0 as usize], 0x1111);
        assert_eq!(vm.registers[Register::R1 as usize], 0x4000);
        assert_eq!(vm.registers[Register::R2 as usize], 0x2222);
    }

    #[test]
    fn test_jmp_does_not_modify_source_register() {
        let mut vm = Vm::new();
        let target_address = 0x5000;
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R3, target_address);

        // JMP R3
        jmp(&mut vm.registers, 0b1100_000_011_000000);

        // Check R3 still contains the original value
        assert_eq!(vm.registers[Register::R3 as usize], target_address);
        assert_eq!(vm.registers[Register::Pc as usize], target_address);
    }

    // ========== Forward and Backward Jumps ==========

    #[test]
    fn test_jmp_forward() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 0x4000);

        // JMP R1 (jump forward)
        jmp(&mut vm.registers, 0b1100_000_001_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x4000);
    }

    #[test]
    fn test_jmp_backward() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x5000);
        vm.write_to_register(Register::R2, 0x3000);

        // JMP R2 (jump backward)
        jmp(&mut vm.registers, 0b1100_000_010_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3000);
    }

    #[test]
    fn test_jmp_large_forward_jump() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x0100);
        vm.write_to_register(Register::R4, 0xF000);

        // JMP R4
        jmp(&mut vm.registers, 0b1100_000_100_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0xF000);
    }

    #[test]
    fn test_jmp_large_backward_jump() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0xF000);
        vm.write_to_register(Register::R5, 0x0100);

        // JMP R5
        jmp(&mut vm.registers, 0b1100_000_101_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x0100);
    }

    // ========== Sequential Jumps ==========

    #[test]
    fn test_sequential_jumps() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x4000);
        vm.write_to_register(Register::R1, 0x5000);
        vm.write_to_register(Register::R2, 0x6000);

        // First jump
        jmp(&mut vm.registers, 0b1100_000_000_000000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x4000);

        // Second jump
        jmp(&mut vm.registers, 0b1100_000_001_000000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x5000);

        // Third jump
        jmp(&mut vm.registers, 0b1100_000_010_000000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x6000);
    }

    #[test]
    fn test_jmp_chain() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x3100);

        // Jump to 0x3100
        jmp(&mut vm.registers, 0b1100_000_000_000000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x3100);

        // Update R0 and jump again
        vm.write_to_register(Register::R0, 0x3200);
        jmp(&mut vm.registers, 0b1100_000_000_000000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x3200);
    }

    // ========== Specific Memory Regions ==========

    #[test]
    fn test_jmp_to_trap_vector_table() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 0x0025); // HALT trap vector

        // JMP R1
        jmp(&mut vm.registers, 0b1100_000_001_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x0025);
    }

    #[test]
    fn test_jmp_to_user_program_space() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x0100);
        vm.write_to_register(Register::R2, 0x3000); // Typical user program start

        // JMP R2
        jmp(&mut vm.registers, 0b1100_000_010_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3000);
    }

    #[test]
    fn test_jmp_to_device_registers() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R3, 0xFE00); // Device register area

        // JMP R3
        jmp(&mut vm.registers, 0b1100_000_011_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0xFE00);
    }

    // ========== Odd Address Tests ==========

    #[test]
    fn test_jmp_to_odd_address() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R4, 0x3001); // Odd address

        // JMP R4 (technically valid in LC-3, though unusual)
        jmp(&mut vm.registers, 0b1100_000_100_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3001);
    }

    #[test]
    fn test_jmp_to_even_address() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R5, 0x4000); // Even address

        // JMP R5
        jmp(&mut vm.registers, 0b1100_000_101_000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x4000);
    }

    // ========== Value Pattern Tests ==========

    #[test]
    fn test_jmp_with_different_bit_patterns() {
        let mut vm = Vm::new();

        let addresses = [
            0x0001, 0x00FF, 0x0100, 0x0FFF, 0x1000, 0x7FFF, 0x8000, 0xFFFF,
        ];

        for (i, &addr) in addresses.iter().enumerate() {
            let register = match i % 8 {
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

            vm.write_to_register(Register::Pc, 0x3000);
            vm.write_to_register(register, addr);

            let instruction = 0b1100_000_000_000000 | ((i as u16 % 8) << 6);
            jmp(&mut vm.registers, instruction);

            assert_eq!(vm.registers[Register::Pc as usize], addr);
        }
    }

    // ========== PC Update Only ==========

    #[test]
    fn test_jmp_only_updates_pc() {
        let mut vm = Vm::new();

        // Set all registers to known values
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
            vm.write_to_register(register, 0x1000 + (i as u16 * 0x100));
        }

        vm.write_to_register(Register::Pc, 0x3000);
        let original_r2 = vm.registers[Register::R2 as usize];

        // JMP R2
        jmp(&mut vm.registers, 0b1100_000_010_000000);

        // PC should change
        assert_eq!(vm.registers[Register::Pc as usize], original_r2);

        // All other registers should be unchanged
        for i in 0..8 {
            if i != Register::Pc as usize {
                assert_eq!(vm.registers[i], 0x1000 + (i as u16 * 0x100));
            }
        }
    }
}
