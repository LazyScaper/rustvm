#[repr(u16)]
pub enum Opcode {
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
    pub fn get(op_code: u16) -> Option<Opcode> {
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
