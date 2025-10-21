use crate::registers::register::Register;
use crate::registers::ConditionFlag;

pub mod add;
pub mod and;
pub mod branch;
pub mod ldi;
pub mod not;
pub mod opcodes;

fn sign_extend(input: u16, bit_count: u16) -> u16 {
    let sign_bit = input >> (bit_count - 1);

    if sign_bit & 1 == 1 {
        return input | (0xFFFF << bit_count);
    }

    input
}

fn update_flags(registers: &mut [u16; (Register::Count as u16) as usize], r: u16) {
    match registers[r as usize] {
        0 => registers[Register::Cond as usize] = ConditionFlag::Zro as u16,
        1 => registers[Register::Cond as usize] = ConditionFlag::Neg as u16,
        _ => registers[Register::Cond as usize] = ConditionFlag::Pos as u16,
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::sign_extend;

    #[test]
    fn should_sign_extend_5bit_positive_number() {
        let result = sign_extend(0b01010, 5);
        assert_eq!(result, 10)
    }

    #[test]
    fn test_sign_extend_5bit_negative_number() {
        // 5-bit: 0b11111 = -1 in two's complement
        let result = sign_extend(0b11101, 5);
        assert_eq!(result as i16, -3);
    }

    #[test]
    fn test_sign_extend_9bit_positive_number() {
        let result = sign_extend(0b000001111, 9);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_sign_extend_9bit_negative_number() {
        let result = sign_extend(0b101001111, 9);
        assert_eq!(result as i16, -177);
    }

    #[test]
    fn test_sign_extend_10bit_positive_max_number() {
        let result = sign_extend(0b0111111111, 10);
        assert_eq!(result, 511);
    }

    #[test]
    fn test_sign_extend_zero() {
        // Zero should remain zero regardless of bit count
        assert_eq!(sign_extend(0, 5), 0);
        assert_eq!(sign_extend(0, 9), 0);
        assert_eq!(sign_extend(0, 11), 0);
    }
}
