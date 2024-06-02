pub struct Bushels {
    pub bushels_harvested: i32,
    pub bushels_per_acre: i32,
    pub destroyed_by_rats: i32,
    pub bushels_in_storage: i32,
    pub owned_land: i32,
    pub land_worth: i32,
}

impl Bushels {
    pub fn new() -> Self {
        Self {
            bushels_harvested: 3000,
            bushels_per_acre: 3,
            destroyed_by_rats: 200,
            bushels_in_storage: 2800,
            owned_land: 1000,
            land_worth: 19,
        }
    }
    pub fn status(&self) {
        println!(
            "\n\tWe harvested {} bushels at {} per acre.",
            self.bushels_harvested, self.bushels_per_acre
        );
        println!(
            "\n\tRats destroyed {} bushels leaving {} in storage.",
            self.destroyed_by_rats,
            self.bushels_harvested - self.destroyed_by_rats
        );
        println!(
            "\n\tThe city owns {} of land, worth {} bushels.",
            self.owned_land,
            self.owned_land * self.land_worth
        );
    }
    pub fn increase_bushels_harvested(&mut self, increase: i32) {
        self.bushels_harvested += increase;
    }
    pub fn decrease_bushels_harvested(&mut self, decrease: i32) {
        self.bushels_harvested -= decrease;
    }
    pub fn increase_bushels_per_acre(&mut self, increase: i32) {
        self.bushels_per_acre += increase;
    }
    pub fn decrease_bushels_per_acre(&mut self, decrease: i32) {
        self.bushels_per_acre -= decrease;
    }
    pub fn increase_destroyed_by_rats(&mut self, increase: i32) {
        self.destroyed_by_rats += increase;
    }
    pub fn decrease_destroyed_by_rats(&mut self, decrease: i32) {
        self.destroyed_by_rats -= decrease;
    }
    pub fn increase_bushels_in_storage(&mut self, increase: i32) {
        self.bushels_in_storage += increase;
    }
    pub fn decrease_bushels_in_storage(&mut self, decrease: i32) {
        self.bushels_in_storage -= decrease;
    }
    pub fn increase_owned_land(&mut self, increase: i32) {
        self.owned_land += increase;
    }
    pub fn decrease_owned_land(&mut self, decrease: i32) {
        self.owned_land -= decrease;
    }
    pub fn set_land_worth(&mut self, land_worth: i32) {
        self.land_worth = land_worth;
    }
}
