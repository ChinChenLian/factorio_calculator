use super::module::ModuleType;

pub enum FurnaceType {
    Stone(FurnaceStone),
    Steel(FurnaceSteel),
    Electric(FurnaceElectric)
}

impl FurnaceType {
    pub fn calc_total(self: &Self) -> (f32, f32){
        match self {
            FurnaceType::Stone(f)       => FurnaceStone::calc_total(f),
            FurnaceType::Steel(f)       => FurnaceSteel::calc_total(f),
            FurnaceType::Electric(f) => FurnaceElectric::calc_total(f),
        }
    }
}

pub struct FurnaceStone {
    _power: u32,
    speed: f32,
}
impl FurnaceStone {
    pub fn init() -> FurnaceType {
        return FurnaceType::Stone(FurnaceStone {
            _power : 90_000,
            speed : 1.0,
        });
    }
    fn calc_total(self: &Self) -> (f32, f32) {
        let total_speed: f32 = self.speed;
        let total_productivity: f32 = 1.0;

        (total_speed, total_productivity)
    }
}

pub struct FurnaceSteel {
    _power: u32,
    speed: f32,
}
impl FurnaceSteel {
    pub fn init() -> FurnaceType {
        return FurnaceType::Steel(FurnaceSteel {
            _power : 90_000,
            speed : 2.0,
        });

    }
    fn calc_total(self: &Self) -> (f32, f32) {
        let total_speed: f32 = self.speed;
        let total_productivity: f32 = 1.0;

        (total_speed, total_productivity)
    }
}

pub struct FurnaceElectric {
    _power: u32,
    _power_drain: u32,
    speed: f32,
    module_slot_1: ModuleType,
    module_slot_2: ModuleType
}
impl FurnaceElectric {
    pub fn init(module_slot_1: ModuleType, module_slot_2: ModuleType) -> FurnaceType {
        return FurnaceType::Electric(FurnaceElectric {
            _power           : 180_000,
            _power_drain     : 6_000,
            speed           : 2.0,
            module_slot_1   ,
            module_slot_2   ,
        });
    }
    fn calc_total(self: &Self) -> (f32, f32) {
        let total_speed: f32 
            = self.speed
            + self.module_slot_1.get_speed()
            + self.module_slot_2.get_speed()
            ;
        let total_productivity: f32 
            = 1.0
            + self.module_slot_1.get_productivity()
            + self.module_slot_2.get_productivity()
            ;

        (total_speed, total_productivity)
    }
}

