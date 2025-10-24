mod instructions;
mod registers;

use crate::instructions::add::add;
use crate::instructions::and::and;
use crate::instructions::branch::br;
use crate::instructions::jump::jmp;
use crate::instructions::jump_register::jsr;
use crate::instructions::ldi::ldi;
use crate::instructions::load::ld;
use crate::instructions::load_effective::lea;
use crate::instructions::load_register::ldr;
use crate::instructions::not::not;
use crate::instructions::opcodes::Opcode;
use crate::instructions::store::st;
use crate::instructions::store_indirect::sti;
use crate::instructions::store_register::str;
use crate::instructions::trap::trap;
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
            Opcode::Ld => ld(&mut self.registers, &self.memory, instruction),
            Opcode::St => st(&mut self.registers, &mut self.memory, instruction),
            Opcode::Jsr => jsr(&mut self.registers, instruction),
            Opcode::And => and(&mut self.registers, instruction),
            Opcode::Ldr => ldr(&mut self.registers, &self.memory, instruction),
            Opcode::Str => str(&mut self.registers, &mut self.memory, instruction),
            Opcode::Not => not(&mut self.registers, instruction),
            Opcode::Ldi => ldi(&mut self.registers, &self.memory, instruction),
            Opcode::Sti => sti(&mut self.registers, &mut self.memory, instruction),
            Opcode::Jmp => jmp(&mut self.registers, instruction),
            Opcode::Lea => lea(&mut self.registers, instruction),
            Opcode::Trap => trap(&mut self.registers, &self.memory, instruction),
            Opcode::Res | Opcode::Rti => {
                return false;
            }
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
            println!("Usage: {} [image-file1]...", args[0]);
            exit(2)
        }
        _ => {
            for image_file in args {
                if !read_image(&image_file) {
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

fn read_image(image_file: &str) -> bool {
    todo!()
}
