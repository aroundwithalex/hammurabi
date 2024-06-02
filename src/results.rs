pub struct Results {
    pub end_pop: i32,
    pub end_land: i32,
    pub end_bushels: i32,
    pub is_famine: bool,
    pub is_plague: bool,
    pub is_infested: bool,
    pub is_invaded: bool,
}

impl Results {
    pub fn new() -> Self {
        Self {
            end_pop: 0,
            end_land: 0,
            end_bushels: 0,
            is_famine: false,
            is_plague: false,
            is_infested: false,
            is_invaded: false,
        }
    }
    pub fn set_end_population(&mut self, pop: i32) {
        self.end_pop = pop;
    }
    pub fn set_end_land_owned(&mut self, land_owned: i32) {
        self.end_land = land_owned;
    }
    pub fn set_end_bushels(&mut self, bushels: i32) {
        self.end_bushels = bushels;
    }
    pub fn set_is_famine(&mut self, in_famine: bool) {
        self.is_famine = in_famine;
    }
    pub fn set_is_plague(&mut self, is_plague: bool) {
        self.is_plague = is_plague;
    }
    pub fn set_is_infested(&mut self, is_infested: bool) {
        self.is_infested = is_infested;
    }
    pub fn set_is_invaded(&mut self, is_invaded: bool) {
        self.is_invaded = is_invaded;
    }
    pub fn higher_pop(&self) -> bool {
        self.end_pop >= 3000
    }
    pub fn more_land(&self) -> bool {
        self.end_land >= 1000
    }
    pub fn more_bushels(&self) -> bool {
        self.end_bushels >= 2800
    }
    pub fn has_events(&self) -> bool {
        self.is_famine || self.is_plague || self.is_infested || self.is_invaded
    }
}
