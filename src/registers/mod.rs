pub mod register;

#[repr(u16)]
pub enum ConditionFlag {
    Pos = 1 << 0, /* P */
    Zro = 1 << 1, /* Z */
    Neg = 1 << 2, /* N */
}
