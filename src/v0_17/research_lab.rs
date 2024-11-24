use super::{assembler::Assembler, parameter};

pub struct ResearchLab{
    power: u32,
}

impl ResearchLab {
    pub fn research() {
        let science_per_sec: f32 
            = parameter::SCIENCE_PER_MINUTE 
            / parameter::SECONDS_IN_MINUTE;
        
        Assembler::automation_science   (science_per_sec);
        // Assembler::logistic_science     (science_per_sec);
        // Assembler::military_science     (science_per_sec);
        // Assembler::chemical_science     (science_per_sec);
    }
}