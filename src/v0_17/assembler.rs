use super::module::ModuleType;

pub enum AssemblerType {
    T1  (AssemblerT1),
    T2  (AssemblerT2),
    T3  (AssemblerT3)
}
impl AssemblerType {
    pub fn calc_total(self: &Self) -> (f32, f32) {
        match self {
            AssemblerType::T1(a) => AssemblerT1::calc_total(a),
            AssemblerType::T2(a) => AssemblerT2::calc_total(a),
            AssemblerType::T3(a) => AssemblerT3::calc_total(a),
        }
    }
}

pub struct AssemblerT1 {
    _power: u32, 
    speed: f32
}
impl AssemblerT1 {
    pub fn init() -> AssemblerType {
        return AssemblerType::T1(AssemblerT1 {
            _power: 10,
            speed: 0.5,
        });
    }
    fn calc_total(self: &Self) -> (f32, f32) {
        let total_speed: f32 = self.speed;
        let total_productivity: f32 = 1.0;

        (total_speed, total_productivity)
    }
}

pub struct AssemblerT2 {
    _power: u32, 
    speed: f32, 
    module_slot_1: ModuleType, 
    module_slot_2: ModuleType
}
impl AssemblerT2 {
    pub fn init (module_slot_1: ModuleType, module_slot_2: ModuleType) -> AssemblerType {
        return AssemblerType::T2(AssemblerT2 {
            _power: 10,
            speed: 0.75,
            module_slot_1,
            module_slot_2,
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

pub struct AssemblerT3 {
    _power: u32, 
    speed: f32, 
    module_slot_1: ModuleType, 
    module_slot_2: ModuleType, 
    module_slot_3: ModuleType, 
    module_slot_4: ModuleType
}
impl AssemblerT3 {
    pub fn init (
        module_slot_1: ModuleType, module_slot_2: ModuleType, 
        module_slot_3: ModuleType, module_slot_4: ModuleType) -> AssemblerType {
        return AssemblerType::T3(AssemblerT3 {
            _power: 10,
            speed: 1.25,
            module_slot_1,
            module_slot_2,
            module_slot_3,
            module_slot_4,
        });
    }
    fn calc_total(self: &Self) -> (f32, f32) {
        let total_speed: f32 
            = self.speed 
            + self.module_slot_1.get_speed()
            + self.module_slot_2.get_speed()
            + self.module_slot_3.get_speed()
            + self.module_slot_4.get_speed()
            ;
        let total_productivity: f32 
            = 1.0
            + self.module_slot_1.get_productivity()
            + self.module_slot_2.get_productivity()
            + self.module_slot_3.get_productivity()
            + self.module_slot_4.get_productivity()
            ;

        (total_speed, total_productivity)
    }
}
