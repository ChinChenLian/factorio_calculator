pub mod research_lab;

// pub mod mining_drill;
pub mod furnace;
pub mod assembler;
// pub mod rocket_launcher;

// pub mod pumpjack;
// pub mod oil_refinery;
// pub mod chemical_plant;

pub mod module;
pub mod calculate;


mod parameter {
    pub const SECONDS_IN_MINUTE: f32    = 60.0;
    pub static SCIENCE_PER_MINUTE: f32  = 60.0;

    pub static ASSEMBLER_TIER: u8   = 3;
    pub static FURNACE_TIER: u8     = 1;

}