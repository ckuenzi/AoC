#[aoc_generator(day16)]
fn gen(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect()
}

#[aoc(day16, part1)]
fn part1(signal: &Vec<u8>) -> usize {
    range_to_num(&fft(signal.to_vec(), 100), 0, 8)
}

#[aoc(day16, part2)]
fn part2(signal: &Vec<u8>) -> usize {
    let mut big_signal = vec![];
    for _ in 0..10000 {
        big_signal.append(&mut signal.clone());
    }
    let offset = range_to_num(&signal, 0, 7);
    range_to_num(&ffft(big_signal, 100, offset), offset, offset + 8)
}

fn ffft(mut signal: Vec<u8>, cycles: u32, offset: usize) -> Vec<u8> {
    for _ in 0..cycles {
        let mut prev = signal[signal.len() - 1];
        for pos in (offset..signal.len() - 1).into_iter().rev() {
            prev = (prev + signal[pos]) % 10;
            signal[pos] = prev;
        }
    }
    signal
}

fn fft(mut signal: Vec<u8>, cycles: u32) -> Vec<u8> {
    let mut new_signal: Vec<u8> = vec![];
    let mut patterns = vec![];
    for i in 1..=signal.len() {
        patterns.push(pattern(i));
    }
    for _ in 0..cycles {
        for i in 0..signal.len() {
            new_signal.push(
                (signal
                    .iter()
                    .zip(patterns[i].iter().cycle().skip(1))
                    .map(|(&s, &p)| s as i8 * p)
                    .sum::<i8>()
                    .abs()
                    % 10) as u8,
            )
        }
        signal = new_signal;
        new_signal = vec![];
    }
    signal
}

fn pattern(reps: usize) -> Vec<i8> {
    let mut output = vec![];
    for i in vec![0, 1, 0, -1] {
        output.append(&mut vec![i; reps]);
    }
    output
}

fn range_to_num(signal: &Vec<u8>, start: usize, end: usize) -> usize {
    signal
        .iter()
        .skip(start)
        .take(end - start)
        .fold(0, |acc, &d| acc * 10 + d as usize)
}

#[test]
fn test_part1() {
    assert_eq!(part1(&gen("80871224585914546619083218645595")), 24176176);
    assert_eq!(part1(&gen("19617804207202209144916044189917")), 73745418);
    assert_eq!(part1(&gen("69317163492948606335995924319873")), 52432133);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&gen("03036732577212944063491565474664")), 84462026);
}
