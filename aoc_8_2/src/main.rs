fn main() {
    let input = &mut include_str!("input.txt")
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
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
    let mut child_data = vec![];
    for _ in 0..children {
        child_data.push(parse(input));
    }
    for _ in 0..data {
        if children == 0 {
            sum += input.remove(0);
        } else {
            if let Some(s) = child_data.get((input.remove(0) - 1) as usize) {
                sum += s;
            }
        }
    }
    sum
}
