use super::intscript::Computer;
use hashbrown::HashSet;

#[aoc(day23, part1)]
fn part1(input: &str) -> i64 {
    let mut computers = vec![Computer::new_from_text(input).clone(); 50];
    for id in 0..computers.len() {
        computers[id].run();
        computers[id].add_input(id as i64);
    }
    for i in (0..50).into_iter().cycle() {
        if computers[i].input.is_empty() {
            computers[i].add_input(-1);
        }
        for packet in computers[i].run().dump_outputs().chunks(3) {
            if packet[0] == 255 {
                return packet[2];
            }
            computers[packet[0] as usize].add_inputs(vec![packet[1], packet[2]]);
        }
    }
    0
}

#[aoc(day23, part2)]
fn part2(input: &str) -> i64 {
    let mut computers = vec![Computer::new_from_text(input).clone(); 50];
    for id in 0..computers.len() {
        computers[id].run();
        computers[id].add_input(id as i64);
    }
    let mut nat = (0, 0);
    let mut past_values = HashSet::new();
    for i in (0..50).into_iter().cycle() {
        if computers[i].input.is_empty() {
            computers[i].add_input(-1);
        }
        for packet in computers[i].run().dump_outputs().chunks(3) {
            if packet[0] == 255 {
                nat.0 = packet[1];
                nat.1 = packet[2];
            } else {
                computers[packet[0] as usize].add_inputs(vec![packet[1], packet[2]]);
            }
        }

        let mut idle = true;
        for i in 0..computers.len() {
            if !computers[i].input.is_empty() {
                idle = false;
            }
        }
        if idle {
            computers[0].add_inputs(vec![nat.0, nat.1]);
            if past_values.contains(&nat.1) && nat.1 != 0{
                return nat.1;
            } else {
                past_values.insert(nat.1);
            }
        }
    }
    0
}
