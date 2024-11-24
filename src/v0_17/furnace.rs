use super::{
    calculate, 
    module::ModuleType, 
    parameter::FURNACE_TIER
};

pub enum Furnace {
    _Stone(FurnaceStone),
    _Steel(FurnaceSteel),
    _Electric(FurnaceElectric)
}
struct FurnaceStone {
    _power: u32,
    speed: f32,
}
struct FurnaceSteel {
    _power: u32,
    speed: f32,
}
struct FurnaceElectric {
    _power: u32,
    _power_drain: u32,
    speed: f32,
    module_slot_1: ModuleType,
    module_slot_2: ModuleType
}

impl Furnace {
    fn get_amount(
        demand_per_sec: f32,
        time_to_smelt: f32,
        output: f32
        ) -> (f32, f32) {
        
        let (total_speed, total_productivity) : (f32, f32) = 
            match FURNACE_TIER {
                1 => FurnaceStone::assign(),
                2 => FurnaceSteel::assign(),
                3 => FurnaceElectric::assign(),
                _ => todo!()
            };

        return calculate::calc_amount(
            total_speed, 
            total_productivity, 
            demand_per_sec, 
            time_to_smelt, 
            output
        )
    }

    pub fn iron_plate(demand_per_sec: f32, mut depth: u32) {
        const TIME_TO_SMELT: f32    = 3.2;
        const OUTPUT: f32           = 1.0;
        const INPUT_IRON_ORE: f32   = 1.0;

        let (total_time, amount) : (f32, f32) 
            = Furnace::get_amount(demand_per_sec, TIME_TO_SMELT, OUTPUT);

        for i in 0..depth {print!("- ");}
        println!("iron plate: {}", amount);
        
        depth += 1;
        let total_input_copper_ore: f32 
            = calculate::calc_input(total_time, amount, INPUT_IRON_ORE);
    }
    pub fn copper_plate(demand_per_sec: f32, mut depth: u32) {
        const TIME_TO_SMELT: f32    = 3.2;
        const OUTPUT: f32           = 1.0;
        const INPUT_COPPER_ORE: f32 = 1.0;

        let (total_time, amount) : (f32, f32) 
            = Furnace::get_amount(demand_per_sec, TIME_TO_SMELT, OUTPUT);

        for i in 0..depth {print!("- ");}
        println!("copper plate: {}", amount);

        depth += 1;
        let total_input_copper_ore: f32 
            = calculate::calc_input(total_time, amount, INPUT_COPPER_ORE);
    }
    pub fn stone_brick(demand_per_sec: f32, mut depth: u32) {
        const TIME_TO_SMELT: f32    = 3.2;
        const OUTPUT: f32           = 1.0;
        const INPUT_STONE: f32      = 2.0;

        let (total_time, amount) : (f32, f32) 
            = Furnace::get_amount(demand_per_sec, TIME_TO_SMELT, OUTPUT);
        
        for i in 0..depth {print!("- ");}
        println!("stone brick: {}", amount);

        depth += 1;
        let total_input_stone: f32 
            = calculate::calc_input(total_time, amount, INPUT_STONE);
    }
    pub fn steel_plate(demand_per_sec: f32, mut depth: u32) {
        const TIME_TO_SMELT: f32    = 16.0;
        const OUTPUT: f32           = 1.0;
        const INPUT_IRON_PLATE: f32 = 5.0;

        let (total_time, amount) : (f32, f32) 
            = Furnace::get_amount(demand_per_sec, TIME_TO_SMELT, OUTPUT);

        for i in 0..depth {print!("- ");}
        println!("steel plate: {}", amount);

        depth += 1;
        let total_input_iron_plate: f32 
            = calculate::calc_input(total_time, amount, INPUT_IRON_PLATE);
    }
}

impl FurnaceStone {
    fn assign() -> (f32, f32) {
        let furnace: FurnaceStone = FurnaceStone {
            _power : 90_000,
            speed : 1.0,
        };
        let total_speed: f32 = furnace.speed;
        let total_productivity: f32 = 1.0;

        (total_speed, total_productivity)
    }
}

impl FurnaceSteel {
    fn assign() -> (f32, f32) {
        let furnace:FurnaceSteel = FurnaceSteel {
            _power : 90_000,
            speed : 2.0,
        };
        let total_speed: f32 = furnace.speed;
        let total_productivity: f32 = 1.0;

        (total_speed, total_productivity)
    }
}

impl FurnaceElectric {
    fn assign() -> (f32, f32) {
        let furnace: FurnaceElectric = FurnaceElectric {
            _power           : 180_000,
            _power_drain     : 6_000,
            speed           : 2.0,
            module_slot_1   : ModuleType::module_init(1, 1),
            module_slot_2   : ModuleType::module_init(1, 1),
        };
        let total_speed: f32 
            = furnace.speed
            + furnace.module_slot_1.get_speed()
            + furnace.module_slot_2.get_speed()
            ;
        let total_productivity: f32 
            = 1.0
            + furnace.module_slot_1.get_productivity()
            + furnace.module_slot_2.get_productivity()
            ;

        (total_speed, total_productivity)
    }
}
