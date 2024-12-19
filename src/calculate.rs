pub fn calc_amount(
    total_speed: f32,
    total_productivity: f32,
    group_output: f32, 
    crafting_time: f32, 
    output: f32
    ) -> (f32, f32) {
        
    let total_time: f32     = crafting_time / total_speed;
    let total_output: f32   = (output * total_productivity) / total_time;
    let amount: f32         = group_output / total_output;
    // dbg!(total_speed, total_productivity, demand_per_sec, time_to_craft, output);
    // dbg!(total_time, total_output, amount);

    (total_time, amount)
}

pub fn calc_input(total_time: f32, amount: f32, input: f32) -> f32 {
    let total_input: f32 = amount * (input / total_time);
    // dbg!(input, total_input);

    total_input
}