struct MiningDrill {
    tier: u8,
    power: u32,
    power_drain: u32,
    mining_speed: f32,
    module_slot: u8,
    output: f32,
    output_uranium: f32,
}

impl MinerDrill {
    fn initiate(self: &mut Self) {
        match self.tier {
            1 => tier_1_init,
            2 => tier_2_init,
            _ => (),
        }
    }

    /* burner mining drill */
    fn tier_1_init(self: &mut Self) {
        self.power = 150_000;
        self.mining_speed = 0.25;
        self.module_slot = 0;
    }
    /* electric mining drill */
    fn tier_2_init() {
        self.power = 90_000;
        self.power_drain = 0;
        self.mining_speed = 0.5;
        self.module_slot = 3;
        self.output = 0.5; // per second
        self.output_uranium = 0.25; // per second
    }
    fn iron_output() {
        
    }

}