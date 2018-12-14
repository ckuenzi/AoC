fn main() {
    let input = 939601;

    let digits = (input as f64).log10().ceil() as usize;
    let digit_mod = 10_usize.pow(digits as u32);

    let mut board = vec![3, 7];
    let mut elfo = 0;
    let mut twinkels = 1;
    let mut current_score = 37;

    'outer: loop {
        let mut score = board[elfo] + board[twinkels];

        let mut digits = vec![];
        if score >= 10 {
            digits.push(score / 10);
            score %= 10;
        }
        digits.push(score);

        for digit in digits {
            board.push(digit);
            current_score = ((current_score * 10) % digit_mod) + board[board.len() - 1];
            if current_score == input {
                break 'outer;
            }
        }

        elfo = (elfo + board[elfo] + 1) % board.len();
        twinkels = (twinkels + board[twinkels] + 1) % board.len();
    }

    println!("{:?}", board.len() - digits);
}
