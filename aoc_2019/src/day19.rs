use super::intscript::Computer;
use hashbrown::HashMap;
use itertools::Itertools;

#[aoc_generator(day19)]
fn beam_gen(input: &str) -> HashMap<(i64, i64), i64> {
    let mut computer = Computer::new_from_text(input);
    let mut out = HashMap::new();
    for y in 0..50 {
        for x in 0..50 {
            computer.reset().add_inputs(vec![x, y]).run();
            out.insert((x, y), computer.get_output().unwrap());
        }
    }
    println!("{}", computer.reset().add_inputs(vec![3, 2]).run().get_output().unwrap());

    out
}

#[aoc(day19, part1)]
fn part1(beam: &HashMap<(i64, i64), i64>) -> i64 {
    beam.values().sum()
}

#[aoc(day19, part2)]
fn part2(beam: &HashMap<(i64, i64), i64>) -> i64 {
    for y in 0..50 {
        let mut sum = 0;
        for x in 0..50 {
            print!("{}", [' ', '#'][*beam.get(&(x, y)).unwrap() as usize]);
            sum += *beam.get(&(x, y)).unwrap();
        }
        println!("{}", sum);
    }
    //let mut computer = Computer::new_from_text(input);
    0
}
