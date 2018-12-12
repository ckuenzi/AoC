use std::collections::HashMap;
fn main() {
    let mut input = include_str!("input.txt").lines();
    let initial_state = input
        .next()
        .unwrap()
        .split(' ')
        .nth(2)
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>();
    let padding = initial_state.len() * 5;

    input.next();
    let rules: HashMap<_, _> = input
        .map(|line| {
            let mut e = line.split(' ').step_by(2);
            (
                e.next()
                    .unwrap()
                    .chars()
                    .map(|c| c == '#')
                    .collect::<Vec<_>>(),
                e.next().unwrap() == "#",
            )
        })
        .collect();

    let mut pots = vec![false; padding * 2];

    initial_state
        .iter()
        .enumerate()
        .for_each(|plant| pots[plant.0 + padding] = *plant.1);

    let mut last = 0;

    for i in 1..=200 {
        let mut changes = vec![];
        for pos in 0..pots.len() - 5 {
            let mut res;
            if let Some(slice) =
                rules.get(&pots.iter().skip(pos).take(5).fold(vec![], |mut out, pot| {
                    out.push(*pot);
                    out
                })) {
                res = *slice;
            } else {
                res = false;
            }
            changes.push((pos + 2, res));
        }
        changes.iter().for_each(|change| pots[change.0] = change.1);
        let num = pots.iter().enumerate().fold(0, |mut out, pot| {
            if *pot.1 {
                out += pot.0 as i32 - padding as i32;
            }
            out
        });
        println!("{} -> {} -> {}", i, num, num - last);
        last = num;
    }
}
