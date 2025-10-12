mod instructions;
mod registers;

use crate::instructions::add::add;
use crate::instructions::and::and;
use crate::instructions::branch::br;
use crate::instructions::ldi::load_indirect;
use crate::instructions::opcodes::Opcode;
use crate::registers::register::Register;
use crate::registers::ConditionFlag;
use std::env;
use std::process::exit;

const MEMORY_MAX: usize = 1 << 16;
const PC_START: usize = 0x3000;

struct Vm {
    memory: [u16; MEMORY_MAX],
    registers: [u16; (Register::Count as u16) as usize],
}

impl Vm {
    fn new() -> Vm {
        Self {
            registers: [0; (Register::Count as u16) as usize],
            memory: [0; MEMORY_MAX],
        }
    }

    fn write_to_register(&mut self, register: Register, value: u16) {
        self.registers[register as usize] = value;
    }

    fn write_to_memory(&mut self, offset: u16, value: u16) {
        self.memory[offset as usize] = value;
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
            Opcode::Br => br(&mut self.registers, instruction),
            Opcode::Add => add(&mut self.registers, instruction),
            Opcode::Ld => {}
            Opcode::St => {}
            Opcode::Jsr => {}
            Opcode::And => and(&mut self.registers, instruction),
            Opcode::Ldr => {}
            Opcode::Str => {}
            Opcode::Rti => {}
            Opcode::Not => {}
            Opcode::Ldi => load_indirect(&mut self.registers, self.memory, instruction),
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

    let mut vm = Vm::new();

    vm.registers[Register::Cond as usize] = ConditionFlag::Zro as u16;
    vm.registers[Register::Pc as usize] = PC_START as u16;

    vm.run();
}

fn read_image(image_file: &String) -> bool {
    todo!()
}
