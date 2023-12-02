pub mod day2 {
    use crate::input_to_vec;
    use std::collections::HashMap;

    pub fn do_it() {
        part1();
        part2();
    }

    fn part1() {
        let mut sum = 0;
        'line: for (id, line) in input_to_vec(2).iter().enumerate() {
            let mut game: Vec<&str> = line.split(&[':', ';'][..]).collect();
            game.remove(0);

            for hand in game {
                for cube in hand.split(',') {
                    let parts: Vec<&str> = cube.trim().split(' ').collect();
                    let number = parts.get(0).unwrap().parse::<u8>().unwrap();
                    let color = *parts.get(1).unwrap();

                    if (color == "red" && number > 12)
                        || (color == "green" && number > 13)
                        || (color == "blue" && number > 14)
                    {
                        continue 'line;
                    }
                }
            }
            sum += id + 1;
        }

        println!("{sum}");
    }

    fn part2() {
        let mut sum = 0;
        for line in input_to_vec(2) {
            let mut game: Vec<&str> = line.split(&[':', ';'][..]).collect();
            game.remove(0);

            let mut cubes = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

            for hand in game {
                for cube in hand.split(',') {
                    let parts: Vec<&str> = cube.trim().split(' ').collect();
                    let number = parts.get(0).unwrap().parse::<u16>().unwrap();
                    let color = *parts.get(1).unwrap();

                    cubes.entry(color).and_modify(|nb| {
                        if number > *nb {
                            *nb = number;
                        }
                    });
                }
            }

            sum += cubes.values().product::<u16>();
        }

        println!("{sum}");
    }
}
