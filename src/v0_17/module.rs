pub enum ModuleType {
    Empty,
    Efficiency  (ModuleEfficiency),
    Speed       (ModuleSpeed),
    Productivity(ModuleProductivity),
}

impl ModuleType {
    pub fn get_speed(self: &Self) -> f32 {
        match self {
            ModuleType::Empty => 0.0,
            ModuleType::Efficiency(a) 
                => ModuleEfficiency::get_speed(a),
            ModuleType::Speed(b) 
                => ModuleSpeed::get_speed(b),
            ModuleType::Productivity(c) 
                => ModuleProductivity::get_speed(c),
        }
    }
    pub fn get_productivity (self: &Self) -> f32 {
        match self {
            ModuleType::Empty => 0.0,
            ModuleType::Efficiency(a) 
                => ModuleEfficiency::get_productivity(a),
            ModuleType::Speed(b) 
                => ModuleSpeed::get_productivity(b),
            ModuleType::Productivity(c) 
                => ModuleProductivity::get_productivity(c),
        }
    }
}

pub trait Module {
    fn init(tier: u8) -> ModuleType {ModuleType::Empty}
    /* pub fn init(module_type: ModuleType, module_tier: u8) -> ModuleType {
        match (module_type, module_tier) {
            (ModuleType::Efficiency(_), 1) 
                => ModuleType::Efficiency (ModuleEfficiency {power: 1}),
            (ModuleType::Efficiency(_), 2) 
                => ModuleType::Efficiency (ModuleEfficiency {power: 1}),
            (ModuleType::Efficiency(_), 3) 
                => ModuleType::Efficiency (ModuleEfficiency {power: 1}),
            (ModuleType::Speed(_), 1) 
                => ModuleType::Speed (ModuleSpeed { 
                    power: 1, 
                    speed: 0.20, 
                }),
            (ModuleType::Speed(_), 2) 
                => ModuleType::Speed (ModuleSpeed { 
                    power: 1, 
                    speed: 0.30, 
                }),
            (ModuleType::Speed(_), 3) 
                => ModuleType::Speed (ModuleSpeed { 
                    power: 1, 
                    speed: 0.50, 
                }),
            (ModuleType::Productivity(_), 1) 
                => ModuleType::Productivity (ModuleProductivity { 
                    power: 1, 
                    speed: -0.05, 
                    productivity: 0.04, 
                }),
            (ModuleType::Productivity(_), 2) 
                => ModuleType::Productivity (ModuleProductivity { 
                    power: 1, 
                    speed: -0.10, 
                    productivity: 0.06, 
                }),
            (ModuleType::Productivity(_), 3) 
                => ModuleType::Productivity (ModuleProductivity { 
                    power: 1, 
                    speed: -0.15, 
                    productivity: 0.10, 
                }),
            _   => ModuleType::Empty,
        }
    } */

}

pub struct ModuleEmpty{}
impl ModuleEmpty{
    pub fn init(_tier: u8) -> ModuleType{ModuleType::Empty}
    fn get_speed(self: &Self)           -> f32 {0.0}
    fn get_productivity(self: &Self)    -> f32 {0.0}
}

pub struct ModuleEfficiency {
    power: u32
}
impl ModuleEfficiency {
    pub fn init(tier: u8) -> ModuleType {
        match tier {
            1 => ModuleType::Efficiency(ModuleEfficiency{power: 1}),
            2 => ModuleType::Efficiency(ModuleEfficiency{power: 1}),
            3 => ModuleType::Efficiency(ModuleEfficiency{power: 1}),
            _ => {
                println!("Error for module input"); 
                ModuleType::Efficiency(ModuleEfficiency{power: 1})
            }
        }
    }
    fn get_speed(self: &Self)           -> f32 {0.0}
    fn get_productivity(self: &Self)    -> f32 {0.0}
}

pub struct ModuleSpeed {
    power: u32, 
    speed: f32
}
impl ModuleSpeed {
    pub fn init(tier: u8) -> ModuleType {
        match tier {
            1 => ModuleType::Speed(ModuleSpeed{
                power: 1, 
                speed: 0.20,
            }),
            2 => ModuleType::Speed(ModuleSpeed{
                power: 1, 
                speed: 0.30,
            }),
            3 => ModuleType::Speed(ModuleSpeed{
                power: 1, 
                speed: 0.50,
            }),
            _ => {
                println!("Error for module input");
                ModuleType::Speed(ModuleSpeed{
                    power: 1, 
                    speed: 0.20,
                })
            }
        }
    }
    fn get_speed(self: &Self)           -> f32 {self.speed}
    fn get_productivity(self: &Self)    -> f32 {0.0}
}

pub struct ModuleProductivity {
    power: u32, 
    speed: f32, 
    productivity: f32
}
impl ModuleProductivity {
    pub fn init(tier: u8) -> ModuleType {
        match tier {
            1 => ModuleType::Productivity(ModuleProductivity{
                power: 1, 
                speed: -0.05, 
                productivity: 0.04,
            }),
            2 => ModuleType::Productivity(ModuleProductivity{
                power: 1, 
                speed: -0.10, 
                productivity: 9.06,
            }),
            3 => ModuleType::Productivity(ModuleProductivity{
                power: 1, 
                speed: -0.15, 
                productivity: 0.10,
            }),
            _ => {
                println!("Error for module input");
                ModuleType::Productivity(ModuleProductivity{
                    power: 1, 
                    speed: -0.05, 
                    productivity: 0.04,
                })
            }
        }
    }
    fn get_speed(self: &Self)           -> f32 {self.speed}
    fn get_productivity(self: &Self)    -> f32 {self.productivity}
}