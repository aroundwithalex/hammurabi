use hammurabi::abacus::{get_land_worth, immigration, percent_destroyed, reap_harvest};
use hammurabi::bushels::Bushels;
use hammurabi::events::{invasion_by_medes, is_famine, is_plague, is_rat_infested};
use hammurabi::pop::Population;
use hammurabi::reports::{
    game_summary, report_bad_result, report_bankrupt, report_famine, report_good_result,
    report_mede_invasion, report_neutral_result, report_no_bushels, report_no_population,
    report_plague,
};
use hammurabi::results::Results;
use hammurabi::sukkal::{ask_to_buy_land, ask_to_cultivate, ask_to_feed, ask_to_sell_land};
use hammurabi::utils::get_current_year;

fn main() {
    game_summary();

    let mut pop = Population::new();

    let mut bushs = Bushels::new();

    let mut results = Results::new();

    for round in 0..=10 {
        pop.status();
        bushs.status();

        get_current_year(round);

        let is_last_round = round == 10;

        let land_to_buy = ask_to_buy_land(bushs.bushels_in_storage, bushs.land_worth);
        bushs.increase_owned_land(land_to_buy);
        bushs.decrease_bushels_in_storage(bushs.land_worth * land_to_buy);

        let land_to_sell = ask_to_sell_land(bushs.owned_land);
        bushs.decrease_owned_land(land_to_sell);
        bushs.increase_bushels_in_storage(bushs.land_worth * land_to_sell);

        let bushels_for_feeding = ask_to_feed(bushs.bushels_in_storage);
        bushs.decrease_bushels_in_storage(bushels_for_feeding);

        let land_to_cultivate = ask_to_cultivate(
            bushs.bushels_in_storage,
            pop.total_population,
            bushs.owned_land,
        );
        bushs.decrease_bushels_in_storage(2 * land_to_cultivate);

        let plague = is_plague();

        if plague {
            let damage_caused = pop.total_population / 2;
            pop.decrease_population(damage_caused);
            pop.increase_plague_deaths(damage_caused);
            report_plague(damage_caused);
        }

        let in_famine = is_famine(pop.total_population, bushs.bushels_in_storage);

        if in_famine {
            let damage_caused = pop.total_population / 2;
            pop.decrease_population(damage_caused);
            pop.increase_starvation_deaths(damage_caused);
            report_famine(damage_caused);
        }

        let infested = is_rat_infested();

        if infested {
            let amount_destroyed = percent_destroyed() * bushs.bushels_in_storage as f32;
            bushs.decrease_bushels_in_storage(amount_destroyed as i32);
        }

        let invaded = invasion_by_medes();

        if invaded {
            let lost_land = bushs.owned_land / 2;
            let land_cost = bushs.land_worth * 2;

            bushs.decrease_owned_land(lost_land);
            bushs.set_land_worth(land_cost);

            report_mede_invasion();
        }

        let harvest = reap_harvest();
        bushs.increase_bushels_harvested(harvest * land_to_cultivate);
        bushs.increase_bushels_in_storage(harvest * land_to_cultivate);

        if !in_famine {
            let immigrants = immigration(
                pop.total_population,
                bushs.owned_land,
                bushs.bushels_in_storage,
            );
            pop.increase_population(immigrants);
            pop.increase_immigration(immigrants);
        }

        bushs.set_land_worth(get_land_worth());

        if pop.total_population < 1 {
            report_no_population();
            return;
        }

        if bushs.bushels_in_storage < 1 {
            report_no_bushels();
            return;
        }

        let bankrupt = bushs.bushels_in_storage < 1 && bushs.owned_land < 1;

        if bankrupt {
            report_bankrupt();
            return;
        }

        if is_last_round {
            results.set_end_population(pop.total_population);
            results.set_end_land_owned(bushs.owned_land);
            results.set_end_bushels(bushs.bushels_in_storage);
            results.set_is_famine(in_famine);
            results.set_is_plague(plague);
            results.set_is_infested(infested);
            results.set_is_invaded(invaded);
        }
    }

    let has_higher_pop = results.higher_pop();
    let has_more_land = results.more_land();
    let has_more_bushels = results.more_bushels();
    let has_events = results.has_events();

    let growth = has_higher_pop && has_more_land && has_more_bushels;

    if growth && !has_events {
        report_good_result();
    } else if growth && has_events {
        report_neutral_result();
    } else {
        report_bad_result();
    }
}
