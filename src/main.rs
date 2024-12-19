mod message;
// use std::io;

mod v0_17;
mod calculate;
mod print;

fn main() {
    v0_17::research_lab::ResearchLab::research();

    /* spm
     * 
     * individual output    = (individual output / craft time) * speed * productivity
     * number of assembler  = total output needed / individual output
     * total request        = number of assembler * individual request
     * call(total request);
     */

    /* structure
     * research lab
     *      - recipe
     *          - assembler / furnace / mining drill 
     *            / oil refinery / chemical plant / pumpjack 
     *            / rocket launcher
     *              - module
     *          - next recipe
     */
}
mod parameter {
    pub const SECONDS_IN_MINUTE: f32    = 60.0;
    pub static SCIENCE_PER_MINUTE: f32  = 60.0;

    pub static DISPLAY_POWER: bool        = false;
    pub static DISPLAY_POWER_TOTAL: bool  = false;
    pub static DISPLAY_AMOUNT: bool       = true;

    pub static AUTOMATION_SCIENCE: bool = true;
}