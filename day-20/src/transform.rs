#[derive(Eq, Ord, PartialEq, PartialOrd, Hash, Copy, Clone, Debug)]
pub enum Transform {
    R0,
    R1,
    R2,
    R3,
    MR0,
    MR1,
    MR2,
    MR3,
}

impl Transform {
    pub fn all() -> Vec<Self> {
        use Transform::*;
        vec![R0, R1, R2, R3, MR0, MR1, MR2, MR3]
    }
}
