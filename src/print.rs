

pub fn result(
    mut depth: u32, 
    name: &str, 
    amount: f32
    ) -> u32 {
    
    for _i in 0..depth {print!("- ");}
    println!("{}: {}", name, amount);

    depth += 1;

    depth
}