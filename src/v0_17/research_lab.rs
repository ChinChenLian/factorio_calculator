use super::{recipe, recipe_init};
use crate::{parameter, print};
pub struct ResearchLab{
    power: u32,
}

impl ResearchLab {
    pub fn research() {
        let science_per_sec: f32 
            = parameter::SCIENCE_PER_MINUTE 
            / parameter::SECONDS_IN_MINUTE;
        
        let mut depth: u32 = 1;

        recipe::automation_science(
            recipe_init::automation_science(),
            science_per_sec,
            depth,
        );
        // logistic_science     (science_per_sec);
        // military_science     (science_per_sec);
        // chemical_science     (science_per_sec);


    }

}

