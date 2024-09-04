#[derive(Debug)]
pub struct Registers {
    /// Accumulator and Flags
    pub a: u8,
    f: u8,
    /// Register pairs
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    // Stack pointer
    pub sp: u16,
    // Program counter/pointer
    pub pc: u16,
}

// pub struct FlagRegister {
//     zero_flag: bool,
//     sub_flag: bool,
//     half_carry_flag: bool,
//     carry_flag: bool,
// }
//
// impl Registers {}
//
// impl FlagRegister {
//     pub const ZERO_FLAG_POS: u8 = 7;
//     pub const SUB_FLAG_POS: u8 = 6;
//     pub const HALF_CARRY_FLAG_POS: u8 = 5;
//     pub const CARRY_FLAG_POS: u8 = 4;
// }
