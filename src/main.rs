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
use crate::registers::register::{MemoryMappedRegister, Register};
use crate::registers::ConditionFlag;
use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;
use std::sync::OnceLock;
use std::{env, io};
use windows::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE, WAIT_OBJECT_0};
use windows::Win32::System::Console::{
    FlushConsoleInputBuffer, GetConsoleMode, GetStdHandle, SetConsoleMode, ENABLE_ECHO_INPUT,
    ENABLE_LINE_INPUT, STD_INPUT_HANDLE,
};
use windows::Win32::System::Console::{GetNumberOfConsoleInputEvents, CONSOLE_MODE};
use windows::Win32::System::Threading::WaitForSingleObject;

static H_STDIN_RAW: OnceLock<isize> = OnceLock::new();
static mut FDW_OLD_MODE: CONSOLE_MODE = CONSOLE_MODE(0);

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

    fn read_file(&mut self, file_name: &str) -> bool {
        let mut file_reader = BufReader::new(File::open(file_name).unwrap());
        let mut address = match file_reader.read_u16::<BigEndian>() {
            Ok(addr) => addr,
            Err(e) => {
                eprintln!("Error reading start address: {}", e);
                return false;
            }
        };

        loop {
            match file_reader.read_u16::<BigEndian>() {
                Ok(instruction) => {
                    if address as usize >= self.memory.len() {
                        eprintln!(
                            "Error: memory overflow at address 0x{:04X} (memory size {})",
                            address,
                            self.memory.len()
                        );
                        return false;
                    }

                    self.memory[address as usize] = instruction;
                    address = address.wrapping_add(1); // Prevent panic, wraps safely
                }
                Err(e) => {
                    return if e.kind() == io::ErrorKind::UnexpectedEof {
                        true // finished reading normally
                    } else {
                        eprintln!("Error reading from file: {}", e);
                        false
                    };
                }
            }
        }
    }

    fn mem_read(&mut self, address: u16) -> u16 {
        if address == MemoryMappedRegister::MR_KBSR as u16 {
            if self.check_key() {
                self.memory[MemoryMappedRegister::MR_KBSR as usize] = 1 << 15;
                self.memory[MemoryMappedRegister::MR_KBDR as usize] = self.get_char();
            } else {
                self.memory[MemoryMappedRegister::MR_KBSR as usize] = 0
            }
        }

        self.memory[address as usize]
    }

    fn check_key(&self) -> bool {
        unsafe {
            if let Some(h_stdin) = self.get_handle() {
                let wait_result = WaitForSingleObject(h_stdin, 1000);
                if wait_result == WAIT_OBJECT_0 {
                    let mut events: u32 = 0;
                    if GetNumberOfConsoleInputEvents(h_stdin, &mut events).is_ok() && events > 0 {
                        return true;
                    }
                }
            }
            false
        }
    }

    fn get_handle(&self) -> Option<HANDLE> {
        H_STDIN_RAW.get().map(|&raw| HANDLE(raw as *mut _))
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

    fn get_char(&self) -> u16 {
        let mut buffer = [0; 1];
        match io::stdin().read_exact(&mut buffer) {
            Ok(_) => buffer[0] as u16,
            _ => panic!("Error reading from stdin"),
        }
    }

    pub fn disable_input_buffering(&self) -> windows::core::Result<()> {
        unsafe {
            let h_stdin = GetStdHandle(STD_INPUT_HANDLE)?;
            if h_stdin == INVALID_HANDLE_VALUE {
                panic!("Invalid handle to STD_INPUT_HANDLE");
            }

            H_STDIN_RAW.set(h_stdin.0 as isize).ok();

            let mut old_mode = CONSOLE_MODE(0);
            GetConsoleMode(h_stdin, &mut old_mode)?;
            FDW_OLD_MODE = old_mode;

            // Disable echo and line input (same behavior as C code)
            let new_mode = old_mode.0 ^ ENABLE_ECHO_INPUT.0 ^ ENABLE_LINE_INPUT.0;
            let new_mode = CONSOLE_MODE(new_mode);

            SetConsoleMode(h_stdin, new_mode)?;
            FlushConsoleInputBuffer(h_stdin)?;
            Ok(())
        }
    }

    pub fn restore_input_buffering(&self) -> windows::core::Result<()> {
        unsafe {
            if let Some(h_stdin) = self.get_handle() {
                SetConsoleMode(h_stdin, FDW_OLD_MODE)?;
            }
            Ok(())
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut vm = Vm::new();

    match args.len() {
        // 1 => {
        //     println!("Usage: {} [image-file1]...", args[0]);
        //     exit(2)
        // }
        1 => {
           vm.read_file("C:\\Users\\Jake\\workspace\\rustvm\\2048.obj");
        }
        _ => {
            for image_file in args {
                if !vm.read_file(&image_file) {
                    println!("{} is not a valid image.", &image_file);
                    exit(1);
                }
            }
        }
    }

    vm.registers[Register::Cond as usize] = ConditionFlag::Zro as u16;
    vm.registers[Register::Pc as usize] = PC_START as u16;

    vm.run();

    vm.restore_input_buffering().unwrap();
}
