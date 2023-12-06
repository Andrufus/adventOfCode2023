pub mod day6 {

    pub fn do_it() {
        part1();
        part2();
    }

    fn part1() {
        let races = [(49, 263), (97, 1532), (94, 1378), (94, 1851)];

        let ways: usize = races.iter().map(|&race| how_many_ways(race.0, race.1)).product();

        println!("{ways}");
    }

    fn part2() {
        let time = 49_979_494_u64;
        let distance = 263_153_213_781_851_u64;

        println!("{}", how_many_ways(time, distance));
    }

    fn how_many_ways(time: u64, distance: u64) -> usize {
        (1..time).filter(|&speed| speed * (time - speed) > distance).count()
    }
}
