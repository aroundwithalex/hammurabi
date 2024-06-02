use rand::Rng;

fn calculate_chance() -> i32 {
    rand::thread_rng().gen_range(1..=100)
}

pub fn is_plague() -> bool {
    let chance = calculate_chance();

    return chance >= 50;
}

pub fn is_famine(population: i32, bushels: i32) -> bool {
    let starving: i32 = population - (bushels / 20);

    return starving >= 1;
}

pub fn is_rat_infested() -> bool {
    let chance = calculate_chance();

    return chance >= 50;
}

pub fn invasion_by_medes() -> bool {
    let chance = calculate_chance();

    return chance >= 50;
}
