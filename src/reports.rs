use crate::utils::calculate_rule;

pub fn game_summary() {
    let summary = format!(
        "\n\tGreetings, great Hammurabi. The year is 1760BC, and you are the ruler of Babylon.
        You have ruled for {} years. You have ten to go. You need to manage the next ten
        years wisely to ensure stability for your successor. Rule well, and you'll become a
        legend. Rule poorly, and bad things will happen.  Good luck, wise and great king.",
        calculate_rule(),
    );
    println!("{}", summary);
}

pub fn report_plague(damage: i32) {
    let report = format!(
        "\n\tGreat Hammurabi, a terrible plague is sweeping Babylonia.
        Thus far, {} have died.",
        damage
    );
    println!("{}", report);
}

pub fn report_famine(damage: i32) {
    let report = format!(
        "\n\tGreat Hammurabi, a terrible famine is sweeping Babylonia.
        Thus far, {} have died.",
        damage
    );
    println!("{}", report);
}

pub fn report_no_population() {
    let report = "\n\tGreat Hammurabi, we have no people left. Immigrants have heard about our situation and
        are avoiding our territory. We should through ourselves at the mercy of the Egyptians. Hopefully
        they will treat us kindly.";
    println!("{}", report);
}

pub fn report_no_bushels() {
    let report =
        "\n\tGreat Hammurabi, we have no bushels left. Because of this, a rebellion started in
        Eastern Babylonia. The rebels have taken Babylon and we have fled to Assyria. Let's hope the
        Assyrians treat us kindly, unlike their other client kings.";
    println!("{}", report);
}

pub fn report_bankrupt() {
    let report = "\n\tGreat Hammurabi, we have no land left. We also have no bushels left either, meaning we
        can't buy more land. There was an intrigue in the court, and you've been deposed as king. Hopefully
        your successor will treat you kindly.";
    println!("{}", report);
}

pub fn report_mede_invasion() {
    let report = "\n\tGreat Hammurabi, we have been invaded by the Medes. They have taken half of
        our land and the cost of land has doubled. We shall have to grow our population to take
        the land back.";
    println!("{}", report);
}

pub fn report_good_result() {
    let report = "\n\tGreat Hammurabi, you have decided to appoint a successor to rule Babylon. You
        retire as a much loved and respected figure, having ruled wisely. You also created a law code,
        which everyone thinks is great. We hope you have a happy retirement in the Hanging Gardens.";
    println!("{}", report);
}

pub fn report_neutral_result() {
    let report = "\n\tGreat Hammurabi, you have handed your kingdom over to a successor. You are passing on
        a good legacy and a stable kingdom. Whilst there might not be statues erected to you, you have done
        a good job.";
    println!("{}", report);
}

pub fn report_bad_result() {
    let report =
        "\n\tGreat Hammurabi, you have not ruled wisely. The empire has shrunk and we are beset by
        problems. Enjoy your exile in the land of Tyre!";
    println!("{}", report);
}
