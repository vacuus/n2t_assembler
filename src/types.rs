pub enum Pars {
    Instruction(Instruction),
    Label(Label),
}

pub enum Instruction {
    A(u16),
    C(u16),
}

// not sure of the representation for now
pub struct Label;
