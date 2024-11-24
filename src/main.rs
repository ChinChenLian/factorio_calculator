mod message;
// mod research_lab;
// mod assembler;
// use std::io;

mod v0_17;
use v0_17::research_lab::ResearchLab;



fn main() {
    ResearchLab::research();
    /* request the spm needed
     */

    /* spm
     * 
     * individual output    = (individual output / craft time) * speed * productivity
     * number of assembler  = total output needed / individual output
     * total request        = number of assembler * individual request
     * call(total request);
     */

    // assembler speed
    // module slot
    // speed mod
    // productivity mod
    // research mod
}