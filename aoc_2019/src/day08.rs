use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Add;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[aoc(day8, part1)]
fn part1(input: &str) -> u32{
    input
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

}

#[aoc(day8, part2)]
fn part2(input: &str) -> String{
    let mut picture = [['2'; HEIGHT]; WIDTH];
    input
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

    let mut out = String::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            out = out.add(&format!("{}", picture[x][y]).to_string());
        }
        out = out.add("\n");
    }
    out.to_string()
}

