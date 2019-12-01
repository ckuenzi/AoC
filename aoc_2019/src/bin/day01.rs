pub fn main() {
    part1();
    part2();
}

fn part1() {
    println!(
        "{}",
        include_str!("inputs\\day01a.txt")
            .lines()
            .map(|x| calculate_fuel(x.parse::<i32>().unwrap()))
            .sum::<i32>()
    );
}

fn part2() {
    println!(
        "{}",
        include_str!("inputs\\day01a.txt")
            .lines()
            .map(|x| rocket_equation(x.parse::<i32>().unwrap()))
            .sum::<i32>()
    );
}

fn calculate_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn rocket_equation(mass: i32) -> i32 {
    let fuel = calculate_fuel(mass);
    if fuel <= 0 {
        0
    } else {
        fuel + rocket_equation(fuel)
    }
}

#[test]
fn test_calculation() {
    assert_eq!(2, calculate_fuel(12));
    assert_eq!(2, calculate_fuel(14));
    assert_eq!(654, calculate_fuel(1969));
    assert_eq!(33583, calculate_fuel(100756));
}

#[test]
fn test_rocket_equation() {
    assert_eq!(2, rocket_equation(14));
    assert_eq!(966, rocket_equation(1969));
    assert_eq!(50346, rocket_equation(100756));
}
