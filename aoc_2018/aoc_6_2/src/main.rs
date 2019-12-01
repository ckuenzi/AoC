const RANGE: i32 = 500;
const LIMIT: i32 = 10000;

fn main() {
    let coords: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| line.split(|c| c == ',').collect::<Vec<_>>())
        .map(|coord| {
            (
                coord[0].parse::<i32>().unwrap(),
                coord[1][1..].parse::<i32>().unwrap(),
            )
        })
        .collect();

    let mut result = 0;
    for x in -RANGE..RANGE {
        for y in -RANGE..RANGE {
            let distance = coords.iter().fold(0, |mut distance, coord| {
                distance += (x - coord.0).abs() + (y - coord.1).abs();
                distance
            });
            if distance < LIMIT {
                result += 1;
            }
        }
    }
    println!("{}", result);
}
