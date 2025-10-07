mod instructions;
mod registers;

use crate::instructions::add::add;
use crate::instructions::opcodes::Opcode;
use crate::registers::register::Register;
use crate::registers::ConditionFlag;
use std::env;
use std::process::exit;

const MEMORY_MAX: usize = 1 << 16;
const PC_START: usize = 0x3000;

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
            Opcode::Add => add(registers, instruction),
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

fn read_image(image_file: &String) -> bool {
    todo!()
}
