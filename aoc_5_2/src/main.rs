fn main() {
    let input = include_str!("input.txt");
    println!(
        "{:?}",
        (b'a'..b'z')
            .map(|c| (
                input
                    .clone()
                    .chars()
                    .filter(|&x| x as u8 != c && x as u8 != c - 32)
                    .fold(vec![], |mut acc, c| {
                        if let Some(&last) = acc.last() {
                            if c as u8 ^ 32 == last as u8 {
                                acc.pop();
                                return acc;
                            }
                        }
                        acc.push(c);
                        acc
                    })
                    .len()
                    - 1,
                char::from(c),
            ))
            .min()
            .unwrap()
    )
}
