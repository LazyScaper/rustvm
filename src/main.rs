mod instructions;
mod registers;

use crate::instructions::add::add;
use crate::instructions::ldi::load_indirect;
use crate::instructions::opcodes::Opcode;
use crate::registers::register::Register;
use crate::registers::ConditionFlag;
use std::env;
use std::process::exit;

const MEMORY_MAX: usize = 1 << 16;
const PC_START: usize = 0x3000;

struct vm {
    memory: [u16; MEMORY_MAX],
    registers: [u16; (Register::Count as u16) as usize],
}

impl vm {
    fn new() -> vm {
        Self {
            registers: [0; (Register::Count as u16) as usize],
            memory: [0; MEMORY_MAX],
        }
    }

    fn load_program(&mut self) {
        // Load the program into memory
    }

    fn run(&mut self) {
        while self.fetch_decode_execute() {}
    }

    fn fetch_decode_execute(&mut self) -> bool {
        let instruction = self.fetch();
        let opcode = Self::decode(instruction);
        self.execute(instruction, opcode)
    }

    fn execute(&mut self, instruction: u16, opcode: Opcode) -> bool {
        match opcode {
            Opcode::Br => {}
            Opcode::Add => add(self.registers, instruction),
            Opcode::Ld => {}
            Opcode::St => {}
            Opcode::Jsr => {}
            Opcode::And => {}
            Opcode::Ldr => {}
            Opcode::Str => {}
            Opcode::Rti => {}
            Opcode::Not => {}
            Opcode::Ldi => load_indirect(self.registers, instruction),
            Opcode::Sti => {}
            Opcode::Jmp => {}
            Opcode::Res => {}
            Opcode::Lea => {}
            Opcode::Trap => {}
        }

        true
    }

    fn decode(instruction: u16) -> Opcode {
        let opcode: Opcode = match Opcode::get(instruction) {
            Some(opcode) => opcode,
            None => panic!("Invalid opcode {:X}", instruction),
        };
        opcode
    }

    fn fetch(&mut self) -> u16 {
        let address_of_instruction = self.registers[Register::Pc as usize];
        let instruction: u16 = self.memory[address_of_instruction as usize];
        instruction
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

    let mut vm = vm::new();

    vm.registers[Register::Cond as usize] = ConditionFlag::Zro as u16;
    vm.registers[Register::Pc as usize] = PC_START as u16;

    vm.run();
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
