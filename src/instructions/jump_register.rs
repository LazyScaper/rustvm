use crate::instructions::sign_extend;
use crate::registers::register::Register::{Count, Pc, R7};

pub fn jsr(registers: &mut [u16; (Count as u16) as usize], instruction: u16) {
    let long_flag = instruction >> 11 & 0x1;

    registers[R7 as usize] = registers[Pc as usize];

    if long_flag == 0 {
        let base_register = (instruction >> 6) & 0x7;
        registers[Pc as usize] = registers[base_register as usize];
    } else {
        let long_pc_offset = sign_extend(instruction & 0x7FF, 11);
        registers[Pc as usize] = registers[Pc as usize].wrapping_add(long_pc_offset);
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::jump_register::jsr;
    use crate::registers::register::Register;
    use crate::Vm;

    // ========== JSR (Jump to Subroutine - PC-relative) ==========

    #[test]
    fn test_jsr_basic_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // JSR with offset +10
        jsr(&mut vm.registers, 0b0100_1_00000001010);

        // R7 should contain return address (original PC)
        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        // PC should be at PC + offset
        assert_eq!(vm.registers[Register::Pc as usize], 0x300A);
    }

    #[test]
    fn test_jsr_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3020);

        // JSR with offset -10 (0x7F6 in 11-bit two's complement)
        jsr(&mut vm.registers, 0b0100_1_11111110110);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3020);
        assert_eq!(vm.registers[Register::Pc as usize], 0x3016);
    }

    #[test]
    fn test_jsr_zero_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // JSR with offset 0 (infinite subroutine loop)
        jsr(&mut vm.registers, 0b0100_1_00000000000);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x3000);
    }

    #[test]
    fn test_jsr_max_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // JSR with offset +1023 (max positive 11-bit)
        jsr(&mut vm.registers, 0b0100_1_01111111111);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x33FF);
    }

    #[test]
    fn test_jsr_max_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3400);

        // JSR with offset -1024 (max negative 11-bit)
        jsr(&mut vm.registers, 0b0100_1_10000000000);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3400);
        assert_eq!(vm.registers[Register::Pc as usize], 0x3000);
    }

    #[test]
    fn test_jsr_saves_return_address() {
        let mut vm = Vm::new();
        let original_pc = 0x3050;
        vm.write_to_register(Register::Pc, original_pc);

        // JSR with offset +100
        jsr(&mut vm.registers, 0b0100_1_00001100100);

        // R7 should contain the return address (original PC)
        assert_eq!(vm.registers[Register::R7 as usize], original_pc);
    }

    #[test]
    fn test_jsr_overwrites_previous_r7() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R7, 0xDEAD); // Previous value

        // JSR with offset +5
        jsr(&mut vm.registers, 0b0100_1_00000000101);

        // R7 should be overwritten with new return address
        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x3005);
    }

    // ========== JSRR (Jump to Subroutine Register) ==========

    #[test]
    fn test_jsrr_basic() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 0x5000);

        // JSRR R2
        jsr(&mut vm.registers, 0b0100_0_00_010_000000);

        // R7 should contain return address
        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        // PC should be set to R2's value
        assert_eq!(vm.registers[Register::Pc as usize], 0x5000);
    }

    #[test]
    fn test_jsrr_from_r0() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x4000);

        // JSRR R0
        jsr(&mut vm.registers, 0b0100_0_00_000_000000);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x4000);
    }

    #[test]
    fn test_jsrr_from_r1() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 0x4100);

        // JSRR R1
        jsr(&mut vm.registers, 0b0100_0_00_001_000000);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x4100);
    }

    #[test]
    fn test_jsrr_from_r3() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R3, 0x5500);

        // JSRR R3
        jsr(&mut vm.registers, 0b0100_0_00_011_000000);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x5500);
    }

    #[test]
    fn test_jsrr_from_r4() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R4, 0x6000);

        // JSRR R4
        jsr(&mut vm.registers, 0b0100_0_00_100_000000);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x6000);
    }

    #[test]
    fn test_jsrr_from_r5() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R5, 0x7000);

        // JSRR R5
        jsr(&mut vm.registers, 0b0100_0_00_101_000000);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x7000);
    }

    #[test]
    fn test_jsrr_from_r6() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R6, 0x8000);

        // JSRR R6
        jsr(&mut vm.registers, 0b0100_0_00_110_000000);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x8000);
    }

    #[test]
    fn test_jsrr_does_not_modify_base_register() {
        let mut vm = Vm::new();
        let target_address = 0x5000;
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R3, target_address);

        // JSRR R3
        jsr(&mut vm.registers, 0b0100_0_00_011_000000);

        // R3 should still contain the original value (unless it's R7)
        assert_eq!(vm.registers[Register::R3 as usize], target_address);
    }

    // ========== JSR/JSRR Return Pattern ==========

    #[test]
    fn test_jsr_and_ret_pattern() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // JSR to subroutine
        jsr(&mut vm.registers, 0b0100_1_00000001010); // JSR +10

        let return_address = vm.registers[Register::R7 as usize];
        let subroutine_address = vm.registers[Register::Pc as usize];

        assert_eq!(return_address, 0x3000);
        assert_eq!(subroutine_address, 0x300A);

        // Simulate RET (JMP R7) would return to 0x3000
        assert_eq!(return_address, 0x3000);
    }

    #[test]
    fn test_nested_jsr_calls() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // First JSR
        jsr(&mut vm.registers, 0b0100_1_00000000101); // JSR +5
        let first_return = vm.registers[Register::R7 as usize];
        assert_eq!(first_return, 0x3000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x3005);

        // Second JSR (nested call - would overwrite R7)
        jsr(&mut vm.registers, 0b0100_1_00000001010); // JSR +10
        let second_return = vm.registers[Register::R7 as usize];
        assert_eq!(second_return, 0x3005); // Return to first subroutine
        assert_eq!(vm.registers[Register::Pc as usize], 0x300F);
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_jsr_from_low_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x0100);

        // JSR with offset +50
        jsr(&mut vm.registers, 0b0100_1_00000110010);

        assert_eq!(vm.registers[Register::R7 as usize], 0x0100);
        assert_eq!(vm.registers[Register::Pc as usize], 0x0132);
    }

    #[test]
    fn test_jsr_from_high_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0xF000);

        // JSR with offset +100
        jsr(&mut vm.registers, 0b0100_1_00001100100);

        assert_eq!(vm.registers[Register::R7 as usize], 0xF000);
        assert_eq!(vm.registers[Register::Pc as usize], 0xF064);
    }

    #[test]
    fn test_jsrr_to_low_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R1, 0x0050);

        // JSRR R1
        jsr(&mut vm.registers, 0b0100_0_00_001_000000);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x0050);
    }

    #[test]
    fn test_jsrr_to_high_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R2, 0xFE00);

        // JSRR R2
        jsr(&mut vm.registers, 0b0100_0_00_010_000000);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        assert_eq!(vm.registers[Register::Pc as usize], 0xFE00);
    }

    #[test]
    fn test_jsrr_to_zero() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R3, 0x0000);

        // JSRR R3
        jsr(&mut vm.registers, 0b0100_0_00_011_000000);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x0000);
    }

    // ========== Preserving Other Registers ==========

    #[test]
    fn test_jsr_preserves_other_registers() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x1111);
        vm.write_to_register(Register::R1, 0x2222);
        vm.write_to_register(Register::R2, 0x3333);

        // JSR with offset +10
        jsr(&mut vm.registers, 0b0100_1_00000001010);

        // Check other registers are unchanged
        assert_eq!(vm.registers[Register::R0 as usize], 0x1111);
        assert_eq!(vm.registers[Register::R1 as usize], 0x2222);
        assert_eq!(vm.registers[Register::R2 as usize], 0x3333);
    }

    #[test]
    fn test_jsrr_preserves_other_registers() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x1111);
        vm.write_to_register(Register::R1, 0x5000);
        vm.write_to_register(Register::R2, 0x2222);
        vm.write_to_register(Register::R3, 0x3333);

        // JSRR R1
        jsr(&mut vm.registers, 0b0100_0_00_001_000000);

        // Check other registers (except R7) are unchanged
        assert_eq!(vm.registers[Register::R0 as usize], 0x1111);
        assert_eq!(vm.registers[Register::R1 as usize], 0x5000);
        assert_eq!(vm.registers[Register::R2 as usize], 0x2222);
        assert_eq!(vm.registers[Register::R3 as usize], 0x3333);
    }

    // ========== Recursive Call Simulation ==========

    #[test]
    fn test_jsr_for_recursive_calls() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // First call
        jsr(&mut vm.registers, 0b0100_1_00000000101); // JSR +5 to 0x3005
        let first_return = vm.registers[Register::R7 as usize];

        // Second call (from within first subroutine)
        jsr(&mut vm.registers, 0b0100_1_00000000101); // JSR +5 to 0x300A
        let second_return = vm.registers[Register::R7 as usize];

        // Note: In real LC-3, you'd save first_return to stack before second call
        assert_eq!(first_return, 0x3000);
        assert_eq!(second_return, 0x3005);
    }

    // ========== Different PC Values ==========

    #[test]
    fn test_jsr_with_various_pc_values() {
        let pc_values = [0x0100, 0x1000, 0x3000, 0x5000, 0x7000, 0xA000];

        for &pc in &pc_values {
            let mut vm = Vm::new();
            vm.write_to_register(Register::Pc, pc);

            // JSR with offset +10
            jsr(&mut vm.registers, 0b0100_1_00000001010);

            assert_eq!(vm.registers[Register::R7 as usize], pc);
            assert_eq!(vm.registers[Register::Pc as usize], pc + 10);
        }
    }

    #[test]
    fn test_jsrr_with_various_addresses() {
        let addresses = [0x0100, 0x1000, 0x3000, 0x5000, 0x7000, 0xA000, 0xFE00];

        for &addr in &addresses {
            let mut vm = Vm::new();
            vm.write_to_register(Register::Pc, 0x3000);
            vm.write_to_register(Register::R1, addr);

            // JSRR R1
            jsr(&mut vm.registers, 0b0100_0_00_001_000000);

            assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
            assert_eq!(vm.registers[Register::Pc as usize], addr);
        }
    }

    // ========== Offset Boundary Tests ==========

    #[test]
    fn test_jsr_offset_plus_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // JSR with offset +1
        jsr(&mut vm.registers, 0b0100_1_00000000001);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x3001);
    }

    #[test]
    fn test_jsr_offset_minus_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // JSR with offset -1 (0x7FF in 11-bit two's complement)
        jsr(&mut vm.registers, 0b0100_1_11111111111);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x2FFF);
    }

    // ========== Subroutine Entry Points ==========

    #[test]
    fn test_jsr_to_trap_routine() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        vm.write_to_register(Register::R0, 0x0400); // Trap routine address

        // JSRR R0 (call trap routine)
        jsr(&mut vm.registers, 0b0100_0_00_000_000000);

        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x0400);
    }

    #[test]
    fn test_jsr_typical_subroutine_pattern() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);

        // JSR to subroutine at offset +50
        jsr(&mut vm.registers, 0b0100_1_00000110010);

        // Verify subroutine entry
        assert_eq!(vm.registers[Register::R7 as usize], 0x3000);
        assert_eq!(vm.registers[Register::Pc as usize], 0x3032);

        // After subroutine completes, RET (JMP R7) would return to 0x3000
    }
}
