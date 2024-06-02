pub struct Population {
    pub total_population: i32,
    pub starvation_deaths: i32,
    pub immigration: i32,
    pub plague_deaths: i32,
}

impl Population {
    pub fn new() -> Self {
        Self {
            total_population: 100,
            starvation_deaths: 0,
            immigration: 5,
            plague_deaths: 0,
        }
    }
    pub fn status(&self) {
        println!("\n\tOur population is currently {}.", self.total_population);
        println!(
            "\n\tThere were {} deaths caused by starvation.",
            self.starvation_deaths
        );
        println!("\n\tThere were {} immigrants.", self.immigration);
        println!("\n\tThere were {} plague deaths.", self.plague_deaths);
    }
    pub fn increase_population(&mut self, increase: i32) {
        self.total_population += increase;
    }
    pub fn decrease_population(&mut self, decrease: i32) {
        self.total_population -= decrease;
    }
    pub fn increase_starvation_deaths(&mut self, increase: i32) {
        self.starvation_deaths += increase;
    }
    pub fn decrease_starvation_deaths(&mut self, decrease: i32) {
        self.starvation_deaths -= decrease;
    }
    pub fn increase_immigration(&mut self, increase: i32) {
        self.immigration += increase;
    }
    pub fn decrease_immigration(&mut self, decrease: i32) {
        self.immigration -= decrease;
    }
    pub fn increase_plague_deaths(&mut self, increase: i32) {
        self.plague_deaths += increase;
    }
    pub fn decrease_plague_deaths(&mut self, decrease: i32) {
        self.plague_deaths -= decrease;
    }
}
