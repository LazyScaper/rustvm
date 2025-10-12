use crate::instructions::sign_extend;
use crate::registers::register::Register::{Cond, Count, Pc};

pub fn br(registers: &mut [u16; (Count as u16) as usize], instruction: u16) {
    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    let cond_flag = (instruction >> 9) & 0x7;

    if cond_flag & registers[Cond as usize] != 0 {
        registers[Pc as usize] = registers[Pc as usize].wrapping_add(pc_offset);
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::branch::br;
    use crate::registers::register::Register;
    use crate::Vm;

    // Helper function to set condition flags
    // N = negative (bit 2), Z = zero (bit 1), P = positive (bit 0)
    fn set_condition_flags(vm: &mut Vm, n: bool, z: bool, p: bool) {
        let mut flags = 0u16;
        if n {
            flags |= 0b100;
        }
        if z {
            flags |= 0b010;
        }
        if p {
            flags |= 0b001;
        }
        vm.write_to_register(Register::Cond, flags);
    }

    // ========== BR (Unconditional Branch - all flags set) ==========

    #[test]
    fn test_br_unconditional_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, true, false); // Any flags

        // BR (nzp all set) with offset +10
        br(&mut vm.registers, 0b0000_111_000001010);

        assert_eq!(vm.registers[Register::Pc as usize], 0x300A);
    }

    #[test]
    fn test_br_unconditional_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3010);
        set_condition_flags(&mut vm, true, false, false);

        // BR with offset -5 (0x1FB in 9-bit two's complement)
        br(&mut vm.registers, 0b0000_111_111111011);

        assert_eq!(vm.registers[Register::Pc as usize], 0x300B);
    }

    #[test]
    fn test_br_unconditional_zero_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, false, true);

        // BR with offset 0 (infinite loop)
        br(&mut vm.registers, 0b0000_111_000000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3000);
    }

    #[test]
    fn test_br_unconditional_max_positive_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, true, false);

        // BR with offset +255
        br(&mut vm.registers, 0b0000_111_011111111);

        assert_eq!(vm.registers[Register::Pc as usize], 0x30FF);
    }

    #[test]
    fn test_br_unconditional_max_negative_offset() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3100);
        set_condition_flags(&mut vm, true, false, false);

        // BR with offset -256
        br(&mut vm.registers, 0b0000_111_100000000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3000);
    }

    // ========== BRn (Branch if Negative) ==========

    #[test]
    fn test_brn_when_negative_flag_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, true, false, false); // N flag set

        // BRn with offset +5
        br(&mut vm.registers, 0b0000_100_000000101);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3005);
    }

    #[test]
    fn test_brn_when_negative_flag_not_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, true, false); // Z flag set, not N

        // BRn with offset +5 (should not branch)
        br(&mut vm.registers, 0b0000_100_000000101);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3000); // No change
    }

    #[test]
    fn test_brn_when_positive_flag_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, false, true); // P flag set

        // BRn with offset +5 (should not branch)
        br(&mut vm.registers, 0b0000_100_000000101);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3000); // No change
    }

    // ========== BRz (Branch if Zero) ==========

    #[test]
    fn test_brz_when_zero_flag_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, true, false); // Z flag set

        // BRz with offset +10
        br(&mut vm.registers, 0b0000_010_000001010);

        assert_eq!(vm.registers[Register::Pc as usize], 0x300A);
    }

    #[test]
    fn test_brz_when_zero_flag_not_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, true, false, false); // N flag set, not Z

        // BRz with offset +10 (should not branch)
        br(&mut vm.registers, 0b0000_010_000001010);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3000); // No change
    }

    // ========== BRp (Branch if Positive) ==========

    #[test]
    fn test_brp_when_positive_flag_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, false, true); // P flag set

        // BRp with offset +15
        br(&mut vm.registers, 0b0000_001_000001111);

        assert_eq!(vm.registers[Register::Pc as usize], 0x300F);
    }

    #[test]
    fn test_brp_when_positive_flag_not_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, true, false); // Z flag set, not P

        // BRp with offset +15 (should not branch)
        br(&mut vm.registers, 0b0000_001_000001111);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3000); // No change
    }

    // ========== BRnz (Branch if Negative or Zero) ==========

    #[test]
    fn test_brnz_when_negative_flag_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, true, false, false); // N flag set

        // BRnz with offset +20
        br(&mut vm.registers, 0b0000_110_000010100);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3014);
    }

    #[test]
    fn test_brnz_when_zero_flag_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, true, false); // Z flag set

        // BRnz with offset +20
        br(&mut vm.registers, 0b0000_110_000010100);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3014);
    }

    #[test]
    fn test_brnz_when_positive_flag_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, false, true); // P flag set

        // BRnz with offset +20 (should not branch)
        br(&mut vm.registers, 0b0000_110_000010100);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3000); // No change
    }

    // ========== BRzp (Branch if Zero or Positive) ==========

    #[test]
    fn test_brzp_when_zero_flag_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, true, false); // Z flag set

        // BRzp with offset +7
        br(&mut vm.registers, 0b0000_011_000000111);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3007);
    }

    #[test]
    fn test_brzp_when_positive_flag_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, false, true); // P flag set

        // BRzp with offset +7
        br(&mut vm.registers, 0b0000_011_000000111);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3007);
    }

    #[test]
    fn test_brzp_when_negative_flag_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, true, false, false); // N flag set

        // BRzp with offset +7 (should not branch)
        br(&mut vm.registers, 0b0000_011_000000111);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3000); // No change
    }

    // ========== BRnp (Branch if Negative or Positive - not zero) ==========

    #[test]
    fn test_brnp_when_negative_flag_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, true, false, false); // N flag set

        // BRnp with offset +3
        br(&mut vm.registers, 0b0000_101_000000011);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3003);
    }

    #[test]
    fn test_brnp_when_positive_flag_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, false, true); // P flag set

        // BRnp with offset +3
        br(&mut vm.registers, 0b0000_101_000000011);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3003);
    }

    #[test]
    fn test_brnp_when_zero_flag_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, true, false); // Z flag set

        // BRnp with offset +3 (should not branch)
        br(&mut vm.registers, 0b0000_101_000000011);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3000); // No change
    }

    // ========== NOP (No Operation - no flags set) ==========

    #[test]
    fn test_nop_never_branches() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, true, false, false); // N flag set

        // NOP (no condition bits set) with offset +100
        br(&mut vm.registers, 0b0000_000_001100100);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3000); // No change
    }

    #[test]
    fn test_nop_with_all_flags_set() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, true, true, true); // All flags (invalid state)

        // NOP with offset +50
        br(&mut vm.registers, 0b0000_000_000110010);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3000); // No change
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_branch_from_low_memory() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x0010);
        set_condition_flags(&mut vm, false, false, true);

        // BRp with offset +5
        br(&mut vm.registers, 0b0000_001_000000101);

        assert_eq!(vm.registers[Register::Pc as usize], 0x0015);
    }

    #[test]
    fn test_branch_backward_to_zero() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 10);
        set_condition_flags(&mut vm, true, false, false);

        // BRn with offset -10
        br(&mut vm.registers, 0b0000_100_111110110);

        assert_eq!(vm.registers[Register::Pc as usize], 0);
    }

    #[test]
    fn test_branch_large_forward_jump() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, true, false);

        // BRz with offset +200
        br(&mut vm.registers, 0b0000_010_011001000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x30C8);
    }

    #[test]
    fn test_branch_large_backward_jump() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3200);
        set_condition_flags(&mut vm, false, false, true);

        // BRp with offset -200 (0x138 in 9-bit two's complement)
        br(&mut vm.registers, 0b0000_001_100111000);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3138);
    }

    // ========== Multiple Condition Combinations ==========

    #[test]
    fn test_branch_multiple_conditions_match_first() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, true, false, false); // N flag

        // BRnzp (all conditions) with offset +1
        br(&mut vm.registers, 0b0000_111_000000001);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3001);
    }

    #[test]
    fn test_branch_multiple_conditions_match_middle() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, true, false); // Z flag

        // BRnzp (all conditions) with offset +1
        br(&mut vm.registers, 0b0000_111_000000001);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3001);
    }

    #[test]
    fn test_branch_multiple_conditions_match_last() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, false, true); // P flag

        // BRnzp (all conditions) with offset +1
        br(&mut vm.registers, 0b0000_111_000000001);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3001);
    }

    // ========== Offset Sign Extension Tests ==========

    #[test]
    fn test_branch_offset_minus_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, false, true);

        // BRp with offset -1 (0x1FF in 9-bit two's complement)
        br(&mut vm.registers, 0b0000_001_111111111);

        assert_eq!(vm.registers[Register::Pc as usize], 0x2FFF);
    }

    #[test]
    fn test_branch_offset_plus_one() {
        let mut vm = Vm::new();
        vm.write_to_register(Register::Pc, 0x3000);
        set_condition_flags(&mut vm, false, true, false);

        // BRz with offset +1
        br(&mut vm.registers, 0b0000_010_000000001);

        assert_eq!(vm.registers[Register::Pc as usize], 0x3001);
    }
}
