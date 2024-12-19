use super::{
    assembler::{AssemblerType, AssemblerT1, AssemblerT2, AssemblerT3},
    furnace::*,
};
use crate::{calculate, print};

/* furnace */
pub struct IronPlate {
    pub furnace: FurnaceType,
}
pub struct CopperPlate {
    pub furnace: FurnaceType,
}
pub struct StoneBrick {
    pub furnace: FurnaceType,
}
pub struct SteelPlate {
    pub furnace: FurnaceType,
}
 pub fn iron_plate(recipe: IronPlate, group_output: f32, mut depth: u32) {
    let name: &str = "iron plate";
    const CRAFTING_TIME: f32    = 3.2;
    const OUTPUT: f32           = 1.0;
    const INPUT_IRON_ORE: f32   = 1.0;

    let (total_speed, total_productivity): (f32, f32) 
        = recipe.furnace.calc_total();

    let (total_time, amount) : (f32, f32) 
        = calculate::calc_amount(
            total_speed, 
            total_productivity, 
            group_output, 
            CRAFTING_TIME, 
            OUTPUT
    );

    for i in 0..depth {print!("- ");}
    println!("iron plate: {}", amount);
    
    depth += 1;
    let total_input_iron_ore: f32 
        = calculate::calc_input(total_time, amount, INPUT_IRON_ORE);
}
pub fn copper_plate(recipe: CopperPlate, group_output: f32, mut depth: u32) {
    let name: &str = "copper plate";
    const CRAFTING_TIME: f32    = 3.2;
    const OUTPUT: f32           = 1.0;
    const INPUT_COPPER_ORE: f32 = 1.0;

    let (total_speed, total_productivity): (f32, f32) 
        = recipe.furnace.calc_total();

    let (total_time, amount) : (f32, f32) 
        = calculate::calc_amount(
            total_speed, 
            total_productivity, 
            group_output, 
            CRAFTING_TIME, 
            OUTPUT
    );

    for i in 0..depth {print!("- ");}
    println!("copper plate: {}", amount);

    depth += 1;
    let total_input_copper_ore: f32 
        = calculate::calc_input(total_time, amount, INPUT_COPPER_ORE);
}
pub fn stone_brick(recipe: StoneBrick, group_output: f32, mut depth: u32) {
    let name: &str = "stone brick";
    const CRAFTING_TIME: f32    = 3.2;
    const OUTPUT: f32           = 1.0;
    const INPUT_STONE: f32      = 2.0;

    let (total_speed, total_productivity): (f32, f32) 
        = recipe.furnace.calc_total();

    let (total_time, amount) : (f32, f32) 
        = calculate::calc_amount(
            total_speed, 
            total_productivity, 
            group_output, 
            CRAFTING_TIME, 
            OUTPUT
    );
    
    for i in 0..depth {print!("- ");}
    println!("stone brick: {}", amount);

    depth += 1;
    let total_input_stone: f32 
        = calculate::calc_input(total_time, amount, INPUT_STONE);
}
pub fn steel_plate(recipe: SteelPlate, group_output: f32, mut depth: u32) {
    let name: &str = "steel plate";
    const CRAFTING_TIME: f32    = 16.0;
    const OUTPUT: f32           = 1.0;
    const INPUT_IRON_PLATE: f32 = 5.0;

    let (total_speed, total_productivity): (f32, f32) 
        = recipe.furnace.calc_total();

    let (total_time, amount) : (f32, f32) 
        = calculate::calc_amount(
            total_speed, 
            total_productivity, 
            group_output, 
            CRAFTING_TIME, 
            OUTPUT
    );

    for i in 0..depth {print!("- ");}
    println!("steel plate: {}", amount);

    depth += 1;
    let total_input_iron_plate: f32 
        = calculate::calc_input(total_time, amount, INPUT_IRON_PLATE);
}

/* assembler */
pub struct AutomationScience {
    pub assembler: AssemblerType,
    pub iron_gear: IronGear,
    pub copper_plate: CopperPlate,
}
pub struct IronGear {
    pub assembler: AssemblerType,
    pub iron_plate: IronPlate,
}
pub fn automation_science (recipe: AutomationScience, group_output: f32, mut depth: u32) {
    let name: &str = "red science";
    const CRAFTING_TIME: f32    = 5.0;
    const OUTPUT: f32           = 1.0;
    const INPUT_IRON_GEAR: f32  = 1.0;
    const INPUT_COPPER: f32     = 1.0;

    let (total_speed, total_productivity): (f32, f32) 
        = recipe.assembler.calc_total();

    let (total_time, amount): (f32, f32) 
        = calculate::calc_amount(
            total_speed, 
            total_productivity, 
            group_output, 
            CRAFTING_TIME, 
            OUTPUT
        );
    
    let total_input_iron_gear: f32  
        = calculate::calc_input(total_time, amount, INPUT_IRON_GEAR);
    let total_input_copper: f32     
        = calculate::calc_input(total_time, amount, INPUT_COPPER);

    for _i in 0..depth {print!("- ");}
    println!("red science: {}", amount);

    depth += 1;
    
    // depth = print::result(depth, name, amount);

    iron_gear       (recipe.iron_gear, total_input_iron_gear, depth);
    copper_plate    (recipe.copper_plate, total_input_copper, depth);
}
fn iron_gear(recipe: IronGear, group_output: f32, mut depth: u32) {
    let name: &str = "iron gear";
    const CRAFTING_TIME: f32    = 0.5;
    const OUTPUT: f32           = 1.0;
    const INPUT_IRON_PLATE: f32 = 2.0;

    let (total_speed, total_productivity): (f32, f32) 
        = recipe.assembler.calc_total();

    let (total_time, amount) : (f32, f32) 
        = calculate::calc_amount(
            total_speed, 
            total_productivity, 
            group_output, 
            CRAFTING_TIME, 
            OUTPUT
        );
    
    let total_input_iron_plate: f32 
        = calculate::calc_input(total_time, amount, INPUT_IRON_PLATE);

    for i in 0..depth {print!("- ");}
    println!("iron gear: {}", amount);

    depth += 1;
    iron_plate(recipe.iron_plate, total_input_iron_plate, depth);
}
