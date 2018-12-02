use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let mut sum = 0;
    let mut map = HashSet::new();
    map.insert(0);

    for s in input.lines().cycle() { 
        sum += s.parse::<i32>().unwrap();
        if !map.insert(sum) {
            break;
        }
    }
    println!("{}", sum);
}
