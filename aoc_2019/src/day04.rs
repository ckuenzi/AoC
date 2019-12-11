use itertools::Itertools;

const LOWER: u32 = 240920;
const UPPER: u32 = 789857;

#[aoc(day4, part1, part2)]
fn part12(_: &str) -> String {
    let mut total_part1 = 0;
    let mut total_part2 = 0;
    for pin in LOWER..UPPER {
        let digits = num_to_digits(pin);
        if !digits.windows(2).all(|d| d[0] <= d[1]) {
            continue;
        }
        let doubles = digits.windows(2).filter(|d| d[0] == d[1]).unique().count();
        let triples = digits
            .windows(3)
            .filter(|d| d[0] == d[1] && d[1] == d[2])
            .unique()
            .count();
        total_part1 += (doubles >= 1) as usize;
        total_part2 += (doubles > triples) as usize;
    }
    format!("{:?}", (total_part1, total_part2))
}

fn num_to_digits(mut num: u32) -> Vec<u8> {
    let mut n = 6;
    let mut digits: Vec<u8> = vec![0; 6];
    while num != 0 {
        n -= 1;
        digits[n] = (num % 10) as u8;
        num /= 10;
    }
    digits
}
