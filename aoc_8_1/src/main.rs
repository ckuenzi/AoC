fn main() {
    let input = include_str!("input.txt")
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<i32>>();
    let iter = &mut input.iter();
    println!("{:?}", parse(iter));
}

fn parse<'a, T>(iter: &mut T) -> i32
where
    T: Iterator<Item = &'a i32>,
{
    let (children, data) = (iter.next().unwrap(), *iter.next().unwrap() as usize);
    (0..*children).into_iter().map(|_| parse(iter)).sum::<i32>() + iter.take(data).sum::<i32>()
}
