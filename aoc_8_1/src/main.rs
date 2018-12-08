fn main() {
    let input = &mut include_str!("input.txt")
        .replace('\n', "")
        .split(' ')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<_>>();
    println!("{:?}", parse(input));
}

fn parse(input: &mut std::vec::Vec<i32>) -> i32 {
    let mut sum = 0;
    if input.len() == 0 {
        return sum;
    }
    let children = input.remove(0);
    let data = input.remove(0);
    for _ in 0..children {
        sum += parse(input);
    }
    for _ in 0..data {
        sum += input.remove(0);
    }
    sum
}
