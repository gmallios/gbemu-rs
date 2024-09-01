#[allow(clippy::upper_case_acronyms)]
enum Instruction {
    NOP,
    ADD(ArithmeticTarget),
    ADC,
    SUB(ArithmeticTarget),
    SBC,
}

enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}
