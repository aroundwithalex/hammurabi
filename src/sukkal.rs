use std::io;

pub fn ask_to_buy_land(bushels: i32, cost: i32) -> i32 {
    loop {
        println!("\n\tGreat Hammurabi, how much land should we buy?\n\t");
        let mut acres = String::new();

        io::stdin()
            .read_line(&mut acres)
            .expect("\n\tGreat Hammurabi, I did not understand.");

        let acres: i32 = acres
            .trim()
            .parse()
            .expect("\n\tGreat Hammurabi, that is not a number.");

        if acres * cost <= bushels {
            return acres;
        }

        println!("\n\tGreat Hammurabi, we have only {} bushels.", bushels);
    }
}

pub fn ask_to_sell_land(current_acres: i32) -> i32 {
    loop {
        println!("\n\tHow many acres shall we sell?");
        let mut acres = String::new();

        io::stdin()
            .read_line(&mut acres)
            .expect("\n\tGreat Hammurabi, I did not understand.");

        let acres: i32 = acres
            .trim()
            .parse()
            .expect("\n\tGreat Hammubari, that is not a number.");

        if acres <= current_acres {
            return acres;
        }

        println!("\n\tGreat Hammurabi, we only have {} acres.", current_acres);
    }
}

pub fn ask_to_feed(current_bushels: i32) -> i32 {
    loop {
        println!("\n\tHow many bushels shall we use to feed the populus?");

        let mut bushels = String::new();

        io::stdin()
            .read_line(&mut bushels)
            .expect("\n\tGreat Hammurabi, I did not understand.");

        let bushels: i32 = bushels
            .trim()
            .parse()
            .expect("\n\tGreat Hammurabi, that is not a number.");

        if bushels <= current_bushels {
            return bushels;
        }

        println!(
            "\n\tGreat Hammurabi, we only have {} bushels.",
            current_bushels
        );
    }
}

pub fn ask_to_cultivate(bushels: i32, population: i32, land: i32) -> i32 {
    loop {
        println!("\n\tHow much land should we plant seed in?");

        let mut planting_land = String::new();

        io::stdin()
            .read_line(&mut planting_land)
            .expect("\n\tGreat Hammurabi, I did not understand.");

        let planting_land: i32 = planting_land
            .trim()
            .parse()
            .expect("\n\tGreat Hammurabi, that is not a number.");

        if planting_land > land {
            println!("\n\tGreat Hammurabi, we only have {} acres.", land);
            continue;
        }

        let pop_to_acres = population * 10;

        if planting_land > pop_to_acres {
            println!(
                "\n\tGreat Hammurabi, we have {} to farm {} acres.",
                population, pop_to_acres
            );
            continue;
        }

        let land_to_bushels = planting_land * 2;

        if land_to_bushels > bushels {
            println!(
                "\n\tGreat Hammurabi, we have {} bushels to cultivate {} acres.",
                bushels,
                bushels / 2
            );
            continue;
        }

        return planting_land;
    }
}
