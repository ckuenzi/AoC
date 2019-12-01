fn main() {
    let mut a = 0;
    let mut b = 1;
    let limit = 10551275;
    while b <= limit {
        if limit % b == 0 {
            a += b;
            println!("a: {}, b: {}", a, b);
        }
        b += 1;
    }
}
