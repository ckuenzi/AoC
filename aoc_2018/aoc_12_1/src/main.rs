fn main() {
    let padding = 3;
    let iterations = 20;

    let mut input = include_str!("input.txt").lines();
    let mut initial_state = input
        .next()
        .unwrap()
        .split(' ')
        .nth(2)
        .unwrap()
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect::<Vec<_>>();
    input.next();

    let mut input = input.collect::<Vec<_>>();
    input.sort();

    let rules: Vec<_> = input
        .iter()
        .rev()
        .map(|rule| match rule.chars().last().unwrap() {
            '#' => 1,
            _ => 0,
        })
        .collect();

    let mut pots = vec![0; padding];
    pots.append(&mut initial_state);
    pots.append(&mut vec![0; iterations]);
    pots.append(&mut vec![0; padding]);

    for i in 1..=iterations {
        let mut rolling = 0;
        for pos in 0..pots.len() - 5 {
            rolling = ((rolling << 1) & 31) + pots[pos + 4]; //<----
            pots[pos + 2] = rules[rolling];
        }

        let num = pots.iter().enumerate().fold(0, |mut out, pot| {
            if *pot.1 == 1 {
                out += pot.0 as i32 - padding as i32;
            }
            out
        });
        println!("{} -> {}", i, num)
    }
}
