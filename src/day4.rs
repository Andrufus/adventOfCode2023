pub mod day4 {
    use crate::input_to_vec;

    pub fn do_it() {
        let mut sum = 0;
        for line in input_to_vec(4) {
            let numbers: Vec<u32> = line
                .split(':')
                .last()
                .unwrap()
                .split(' ')
                .filter_map(|num| num.parse().ok())
                .collect();

            let wins = numbers[10..].iter().filter(|num| numbers[..10].contains(num)).count() as u32;
            if wins > 0 {
                sum += 2_u32.pow(wins - 1);
            }
        }

        println!("{sum}");
    }
}
