fn main() {
    let input = include_str!("test.txt")
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<_>>();
    let iter = &mut input.iter();
    println!("{:?}", parse(iter));
}

fn parse<'a, T>(iter: &mut T) -> i32
where
    T: Iterator<Item = &'a i32>,
{
    let (children, data) = (iter.next().unwrap(), iter.next().unwrap());
    (0..*children).into_iter().map(|_| parse(iter)).sum::<i32>()
        + iter.take(*data as usize).sum::<i32>()
}
