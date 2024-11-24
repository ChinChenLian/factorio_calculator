use super::{
    calculate, 
    furnace::Furnace, 
    module::ModuleType, 
    parameter::ASSEMBLER_TIER
};

pub enum Assembler {
    _T1  (AssemblerT1),
    _T2  (AssemblerT2),
    _T3  (AssemblerT3)
}
struct AssemblerT1 {
    _power: u32, 
    speed: f32
}
struct AssemblerT2 {
    _power: u32, 
    speed: f32, 
    module_slot_1: ModuleType, 
    module_slot_2: ModuleType
}
struct AssemblerT3 {
    _power: u32, 
    speed: f32, 
    module_slot_1: ModuleType, 
    module_slot_2: ModuleType, 
    module_slot_3: ModuleType, 
    module_slot_4: ModuleType
}
// main
impl Assembler {
    fn get_amount(                 
        demand_per_sec: f32, 
        time_to_craft: f32, 
        output: f32
        ) -> (f32, f32) {
        
        let (total_speed, total_productivity)
            : (f32, f32) 
            = match ASSEMBLER_TIER {
                1 => AssemblerT1::assign(),
                2 => AssemblerT2::assign(),
                3 => AssemblerT3::assign(),
                _ => {
                    print!("error assembler tier. default to tier 1");
                    AssemblerT1::assign()
                }
        };
        
        return calculate::calc_amount(
            total_speed, 
            total_productivity, 
            demand_per_sec, 
            time_to_craft, 
            output
        );
    }

    pub fn automation_science(demand_per_sec: f32) {
        // craft product
        const TIME_TO_CRAFT: f32    = 5.0;
        const OUTPUT: f32           = 1.0;
        const INPUT_IRON_GEAR: f32  = 1.0;
        const INPUT_COPPER: f32     = 1.0;

        let (total_time, amount) : (f32, f32) 
            = Assembler::get_amount(demand_per_sec, TIME_TO_CRAFT, OUTPUT);
        
        let total_input_iron_gear: f32  
            = calculate::calc_input(total_time, amount, INPUT_IRON_GEAR);
        let total_input_copper: f32     
            = calculate::calc_input(total_time, amount, INPUT_COPPER);

        let mut depth: u32 = 1;
        for i in 0..depth {print!("- ");}
        println!("red science: {}", amount);

        depth += 1;
        Assembler::iron_gear    (total_input_iron_gear, depth);
        Furnace::copper_plate   (total_input_copper, depth);
    }
    fn iron_gear(demand_per_sec: f32, mut depth: u32) {
        const TIME_TO_CRAFT: f32    = 0.5;
        const OUTPUT: f32           = 1.0;
        const INPUT_IRON_PLATE: f32 = 2.0;

        let (total_time, amount) : (f32, f32) 
            = Assembler::get_amount(demand_per_sec, TIME_TO_CRAFT, OUTPUT);
        
        let total_input_iron_plate: f32 
            = calculate::calc_input(total_time, amount, INPUT_IRON_PLATE);

        for i in 0..depth {print!("- ");}
        println!("iron gear: {}", amount);

        depth += 1;
        Furnace::iron_plate(total_input_iron_plate, depth);
    }
}

impl AssemblerT1 {
    fn assign() -> (f32, f32) {
        let assembler: AssemblerT1 = AssemblerT1 {
            _power: 10,
            speed: 0.5,
        };
        let total_speed: f32 = assembler.speed;
        let total_productivity: f32 = 1.0;

        (total_speed, total_productivity)
    }
}

impl AssemblerT2 {
    fn assign() -> (f32, f32) {
        let assembler: AssemblerT2 = AssemblerT2 {
            _power: 10,
            speed: 0.75,
            module_slot_1: ModuleType::module_init(1, 1),
            module_slot_2: ModuleType::module_init(1, 1),
        };
        let total_speed: f32 
            = assembler.speed
            + assembler.module_slot_1.get_speed()
            + assembler.module_slot_2.get_speed()
            ;
        let total_productivity: f32 
            = 1.0
            + assembler.module_slot_1.get_productivity()
            + assembler.module_slot_2.get_productivity()
            ;

        (total_speed, total_productivity)
    }
}

impl AssemblerT3 {
    fn assign() -> (f32, f32) {
        let assembler: AssemblerT3 = AssemblerT3 {
            _power: 10,
            speed: 1.25,
            module_slot_1: ModuleType::module_init(1, 1),
            module_slot_2: ModuleType::module_init(1, 1),
            module_slot_3: ModuleType::module_init(1, 1),
            module_slot_4: ModuleType::module_init(1, 1),
        };
        let total_speed: f32 
            = assembler.speed 
            + assembler.module_slot_1.get_speed()
            + assembler.module_slot_2.get_speed()
            + assembler.module_slot_3.get_speed()
            + assembler.module_slot_4.get_speed()
            ;
        let total_productivity: f32 
            = 1.0
            + assembler.module_slot_1.get_productivity()
            + assembler.module_slot_2.get_productivity()
            + assembler.module_slot_3.get_productivity()
            + assembler.module_slot_4.get_productivity()
            ;

        (total_speed, total_productivity)
    }
}

