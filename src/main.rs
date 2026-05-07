use rosu_pp::{Beatmap, Difficulty, Performance};
use metron_rs::calculator::Calculator;
use metron_rs::etterna::minacalc515::{MinaCalc515, MinaCalcDifficultyContext};
use metron_rs::clock_rate::ClockRate;
use metron_rs::quaver::quaver2025::difficulty::{Quaver2025, QuaverDifficultyContext};
use metron_rs::interlude::interlude2025::{Interlude2025, Interlude2025DifficultyContext};
use rhythm_open_exchange::auto_decode;

fn main() {
    // rosu-pp
    let map = Beatmap::from_path("./assets/test/taiko_beatmap.osu").unwrap();

    if let Err(sus) = map.check_suspicion() {
        panic!("{sus:?}");
    }

    let diff_attrs = Difficulty::new().calculate(&map);

    let _stars = diff_attrs.stars();

    let perf_attrs = Performance::new(diff_attrs)
        .combo(545)
        .accuracy(99.5)
        .calculate();

    let pp = perf_attrs.pp();

    println!("pp: {pp}");

    // metron-rs
    let chart = auto_decode("./assets/test/mania_beatmap.osu").unwrap();
    let calc = MinaCalc515;
    let context = MinaCalcDifficultyContext::default();

    let difficulty = calc.calculate_difficulty(&chart, &context).unwrap();
    println!("MSD: {:.2}", difficulty.overall);

    let quaver_context = QuaverDifficultyContext {
        clock_rate: ClockRate::from_percentage(100).unwrap(),
    };
    let difficulty = Quaver2025.calculate_difficulty(&chart, &quaver_context).unwrap();
    println!("Quaver: {:.2}", difficulty.stars);

    let inter_context = Interlude2025DifficultyContext {
        clock_rate: Some(ClockRate::from_percentage(100).unwrap()),
    };
    let difficulty = Interlude2025.calculate_difficulty(&chart, &inter_context).unwrap();
    println!("Interlude: {:.2}", difficulty.stars);

    // ssrrr
    let ssrrr_chart = ssrrr::preprocess_file("./assets/test/mania_beatmap.osu", "None", 100).unwrap();
    let ssrrr_data = ssrrr::algorithm::process::process::calculate(&ssrrr_chart).unwrap();

    println!("SSRRR: {:.2}", ssrrr_data.rating);

}
