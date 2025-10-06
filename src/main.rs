use std::env;
use std::process::exit;

const MEMORY_MAX: usize = 1 << 16;
const PC_START: usize = 0x3000;

#[repr(u16)]
enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    Pc = 8, /* program counter */
    Cond = 9,
    Count = 10,
}

#[repr(u16)]
enum ConditionFlag {
    Pos = 1 << 0, /* P */
    Zro = 1 << 1, /* Z */
    Neg = 1 << 2, /* N */
}

#[repr(u16)]
enum Opcode {
    Br = 0,    /* branch */
    Add = 1,   /* add  */
    Ld = 2,    /* load */
    St = 3,    /* store */
    Jsr = 4,   /* jump register */
    And = 5,   /* bitwise and */
    Ldr = 6,   /* load register */
    Str = 7,   /* store register */
    Rti = 8,   /* unused */
    Not = 9,   /* bitwise not */
    Ldi = 10,  /* load indirect */
    Sti = 11,  /* store indirect */
    Jmp = 12,  /* jump */
    Res = 13,  /* reserved (unused) */
    Lea = 14,  /* load effective address */
    Trap = 15, /* execute trap */
}

impl Opcode {
    fn get(op_code: u16) -> Option<Opcode> {
        match op_code {
            0 => Some(Opcode::Br),
            1 => Some(Opcode::Add),
            2 => Some(Opcode::Ld),
            3 => Some(Opcode::St),
            4 => Some(Opcode::Jsr),
            5 => Some(Opcode::And),
            6 => Some(Opcode::Ldr),
            7 => Some(Opcode::Str),
            8 => Some(Opcode::Rti),
            9 => Some(Opcode::Not),
            10 => Some(Opcode::Ldi),
            11 => Some(Opcode::Sti),
            12 => Some(Opcode::Jmp),
            13 => Some(Opcode::Res),
            14 => Some(Opcode::Lea),
            15 => Some(Opcode::Trap),
            _ => None,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            print!("Usage: {} [image-file1]...\n", args[0]);
            exit(2)
        }
        _ => {
            for image_file in args {
                if (!read_image(&image_file)) {
                    println!("{} is not a valid image.", &image_file);
                    exit(1);
                }
            }
        }
    }

    let mut registers = [0u16; (Register::Count as u16) as usize];
    let mut memory = [0u16; MEMORY_MAX];

    registers[Register::Cond as usize] = ConditionFlag::Zro as u16;
    registers[Register::Pc as usize] = PC_START as u16;

    let mut running = true;

    while running {
        let address_of_instruction = registers[Register::Pc as usize];
        let instruction: u16 = memory[address_of_instruction as usize];
        let opcode: Opcode = Opcode::get(instruction).unwrap();

        match opcode {
            Opcode::Br => {}
            Opcode::Add => {
                let destination_register = (instruction >> 9) & 0x7;
                let first_argument = (instruction >> 6) & 0x7;
                let immediate_mode_flag = (instruction >> 5) & 0x1;

                if immediate_mode_flag == 1 {
                    let imm5 = sign_extend(instruction & 0x1F, 5);
                    registers[destination_register as usize] = first_argument + imm5;
                } else {
                    let address_of_second_argument = instruction & 0x7;
                    registers[destination_register as usize] =
                        first_argument + registers[address_of_second_argument as usize];
                }

                update_flags(registers, destination_register)
            }
            Opcode::Ld => {}
            Opcode::St => {}
            Opcode::Jsr => {}
            Opcode::And => {}
            Opcode::Ldr => {}
            Opcode::Str => {}
            Opcode::Rti => {}
            Opcode::Not => {}
            Opcode::Ldi => {}
            Opcode::Sti => {}
            Opcode::Jmp => {}
            Opcode::Res => {}
            Opcode::Lea => {}
            Opcode::Trap => {}
            _ => {}
        }
    }
}

fn update_flags(mut registers: [u16; (Register::Count as u16) as usize], r: u16) {
    match registers[r as usize] {
        0 => registers[Register::Cond as usize] = ConditionFlag::Zro as u16,
        1 => registers[Register::Cond as usize] = ConditionFlag::Neg as u16,
        _ => registers[Register::Cond as usize] = ConditionFlag::Pos as u16,
    }
}

fn sign_extend(input: u16, bit_count: u16) -> u16 {
    let sign_bit = input >> bit_count - 1;

    if sign_bit & 1 == 1 {
        return input | (0xFFFF << bit_count);
    }

    input
}

fn read_image(image_file: &String) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::sign_extend;

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
