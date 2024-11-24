pub enum ModuleType {
    Efficiency  (ModuleEfficiency),
    Speed       (ModuleSpeed),
    Productivity(ModuleProductivity),
}
struct ModuleEfficiency {
    power: u32
}
struct ModuleSpeed {
    power: u32, 
    speed: f32
}
struct ModuleProductivity {
    power: u32, 
    speed: f32, 
    productivity: f32
}

impl ModuleType {
    pub fn module_init(module_type: u8, module_tier: u8) -> ModuleType {
        match (module_type, module_tier) {
            (1, 1) => ModuleType::Efficiency (ModuleEfficiency {power: 1}),
            (1, 2) => ModuleType::Efficiency (ModuleEfficiency {power: 1}),
            (1, 3) => ModuleType::Efficiency (ModuleEfficiency {power: 1}),
            (2, 1) => ModuleType::Speed (ModuleSpeed { 
                        power: 1, 
                        speed: 0.20, 
                    }),
            (2, 2) => ModuleType::Speed (ModuleSpeed { 
                        power: 1, 
                        speed: 0.30, 
                    }),
            (2, 3) => ModuleType::Speed (ModuleSpeed { 
                        power: 1, 
                        speed: 0.50, 
                    }),
            (3, 1) => ModuleType::Productivity (ModuleProductivity { 
                        power: 1, 
                        speed: -0.05, 
                        productivity: 0.04, 
                    }),
            (3, 2) => ModuleType::Productivity (ModuleProductivity { 
                        power: 1, 
                        speed: -0.10, 
                        productivity: 0.06, 
                    }),
            (3, 3) => ModuleType::Productivity (ModuleProductivity { 
                        power: 1, 
                        speed: -0.15, 
                        productivity: 0.10, 
                    }),
            _ => todo!()
        }
    }
    pub fn get_speed(self: &Self) -> f32 {
        match self {
            ModuleType::Efficiency(a) 
                => ModuleEfficiency::get_speed(a),
            ModuleType::Speed(b) 
                => ModuleSpeed::get_speed(b),
            ModuleType::Productivity(c) 
                => ModuleProductivity::get_speed(c),
        }
    }
    pub fn get_productivity(self: &Self) -> f32 {
        match self {
            ModuleType::Efficiency(a) 
                => ModuleEfficiency::get_productivity(a),
            ModuleType::Speed(b) 
                => ModuleSpeed::get_productivity(b),
            ModuleType::Productivity(c) 
                => ModuleProductivity::get_productivity(c),
        }
    }
}

impl ModuleEfficiency {
    fn get_speed(self: &Self)           -> f32 {0.0}
    fn get_productivity(self: &Self)    -> f32 {0.0}
}
impl ModuleSpeed {
    fn get_speed(self: &Self)           -> f32 {self.speed}
    fn get_productivity(self: &Self)    -> f32 {0.0}
}
impl ModuleProductivity {
    fn get_speed(self: &Self)           -> f32 {self.speed}
    fn get_productivity(self: &Self)    -> f32 {self.productivity}
}