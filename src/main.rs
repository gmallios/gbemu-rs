use strum::IntoEnumIterator;

use crate::instruction::{GBInstruction, GBInstructions};

mod cpu;
mod instruction;
mod reg;

fn main() {
    let enumA = GBInstruction::iter().count();
    let enumB = GBInstructions::iter().count();
    println!("Hello, world!, {} {}", enumA, enumB);
}
