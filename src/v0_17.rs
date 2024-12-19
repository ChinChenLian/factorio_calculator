pub mod research_lab;

// pub mod mining_drill;
pub mod furnace;
pub mod assembler;
// pub mod rocket_launcher;

// pub mod pumpjack;
// pub mod oil_refinery;
// pub mod chemical_plant;

pub mod module;
pub mod recipe;




/* This is for all the settings along the entire recipe chain.
 * It is place here because this is the file that include everything together
 */
mod recipe_init {

    use super::furnace::FurnaceStone        as F1;
    use super::furnace::FurnaceSteel        as F2;
    use super::furnace::FurnaceElectric     as F3;

    use super::assembler::AssemblerT1       as A1;
    use super::assembler::AssemblerT2       as A2;
    use super::assembler::AssemblerT3       as A3;

    use super::module::ModuleEmpty          as Emp;
    use super::module::ModuleEfficiency     as Eff;
    use super::module::ModuleSpeed          as Spd;
    use super::module::ModuleProductivity   as Prod;

    use super::recipe::*;

    pub fn automation_science () -> AutomationScience {
        return AutomationScience {
            assembler: A2::init(
                Eff::init(1), 
                Eff::init(1)
            ),
            iron_gear: IronGear {
                assembler: A1::init(),
                iron_plate: IronPlate{
                    furnace: F1::init(),
                }
            },
            copper_plate: CopperPlate{
                furnace: F1::init()
            },
        };
    }
}