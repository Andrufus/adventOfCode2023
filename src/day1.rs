pub mod day1 {
    use crate::input_to_vec;
    use fancy_regex::Regex;

    pub fn do_it() {
        println!("Part 1 : {}", compute(r"(?=(\d{1}))"));
        println!("Part 2 : {}", compute(r"(?=(\d{1}|one|two|three|four|five|six|seven|eight|nine))"));
    }

    fn compute(regex: &str) -> u16 {
        let mut sum = 0;
        let regex = Regex::new(regex).unwrap();

        for line in input_to_vec(1) {
            let numbers: Vec<&str> = regex
                .captures_iter(&line)
                .map(|cap| cap.unwrap().get(1).unwrap().as_str())
                .collect();

            sum += format!("{}{}", convert(*numbers.first().unwrap()), convert(*numbers.last().unwrap()))
                .parse::<u16>()
                .unwrap();
        }

        sum
    }

    fn convert(string: &str) -> u8 {
        match string.parse::<u8>() {
            Ok(digit) => digit,
            Err(_) => match string {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                &_ => panic!(),
            },
        }
    }
}
