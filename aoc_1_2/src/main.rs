use std::collections::HashMap;
use std::process::exit;

fn main() {
    let input = include_str!("input.txt");

    let mut sum = 0;
    let mut map = HashMap::new();

    map.insert(0, true);

    while true{
        for s in input.lines() { 
            let s: i32 = s.parse().unwrap();
            sum += s;
            if map.insert(sum, true) != None {
                println!("{}", sum);
                exit(0);
            }
        }
    }

    println!("{}", sum);
}
