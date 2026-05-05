fn main() {
    let map = rosu_pp::Beatmap::from_path("./assets/test/beatmap.osu").unwrap();

    if let Err(sus) = map.check_suspicion() {
        panic!("{sus:?}");
    }

    let diff_attrs = rosu_pp::Difficulty::new().calculate(&map);

    let _stars = diff_attrs.stars();

    let perf_attrs = rosu_pp::Performance::new(diff_attrs)
        .combo(545)
        .accuracy(99.5)
        .calculate();

    let pp = perf_attrs.pp();

    println!("pp: {pp}");
}
