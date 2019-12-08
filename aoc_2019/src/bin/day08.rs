use itertools::Itertools;
use std::collections::HashMap;
const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut picture = [['2'; HEIGHT]; WIDTH];
    include_str!("inputs\\day08a.txt")
        .chars()
        .chunks(WIDTH * HEIGHT)
        .into_iter()
        .for_each(|layer| {
            for (i, c) in layer.enumerate() {
                let x = i % WIDTH;
                let y = (i - x) / 25;
                if picture[x][y] == '2' {
                    picture[x][y] = match c {
                        '1' => 'X',
                        '2' => '2',
                        _ => ' ',
                    };
                }
            }
        });

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", picture[x][y]);
        }
        print!("\n");
    }
}

fn part1() {
    println!(
        "{:?}",
        include_str!("inputs\\day08a.txt")
            .chars()
            .chunks(WIDTH * HEIGHT)
            .into_iter()
            .map(|layer| {
                let mut cnt = HashMap::<char, u32>::new();
                for c in layer {
                    *cnt.entry(c).or_insert(0) += 1;
                }
                cnt
            })
            .min_by_key(|cnt| *cnt.get(&'0').unwrap())
            .map(|cnt| *cnt.get(&'1').unwrap() * *cnt.get(&'2').unwrap())
            .unwrap()
    );
}
