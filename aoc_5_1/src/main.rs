fn main() {
    println!(
        "{}",
        include_str!("input.txt")
            .chars()
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
            - 1
    );
}
