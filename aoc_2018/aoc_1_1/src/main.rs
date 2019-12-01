fn main() {
    println!("{}", include_str!("input.txt").lines().map(|x| x.parse::<i32>().unwrap()).sum::<i32>());
}
