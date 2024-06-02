use rand::Rng;

pub fn percent_destroyed() -> f32 {
    rand::thread_rng().gen_range(0.1..=0.4)
}

pub fn immigration(population: i32, land: i32, stored_grain: i32) -> i32 {
    let immigrants: i32 = (20 * land + stored_grain) / (100 * population) + 1;
    return immigrants;
}

pub fn reap_harvest() -> i32 {
    rand::thread_rng().gen_range(1..=10)
}

pub fn get_land_worth() -> i32 {
    rand::thread_rng().gen_range(16..=26)
}
