use crate::registers::register::Register::{Count, Pc, R7};

enum TrapCode {
    TRAP_GETC = 0x20,  /* get character from keyboard, not echoed onto the terminal */
    TRAP_OUT = 0x21,   /* output a character */
    TRAP_PUTS = 0x22,  /* output a word string */
    TRAP_IN = 0x23,    /* get character from keyboard, echoed onto the terminal */
    TRAP_PUTSP = 0x24, /* output a byte string */
    TRAP_HALT = 0x25,  /* halt the program */
}

pub fn trap(registers: &mut [u16; (Count as u16) as usize], instruction: u16) {
    let trap_vect_8 = instruction & 0xFF;

    registers[R7 as usize] = registers[Pc as usize];

    match instruction & 0xFF {
        TRAP_GETC => {}
        TRAP_OUT => {}
        TRAP_PUTS => {}
        TRAP_IN => {}
        TRAP_PUTSP => {}
        TRAP_HALT => {}
    }
}
