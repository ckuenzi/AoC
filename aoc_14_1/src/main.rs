fn main() {
    let input = 939601;
    let mut board = vec![3, 7];
    let mut elfo = 0;
    let mut twinkels = 1;

    let mut score;
    let mut print_count = 10;

    for _ in 0..input + 10 {
        score = board[elfo] + board[twinkels];
        if score >= 10 {
            board.push(score / 10);
            score %= 10;
            if board.len() > input {
                print_count -= 1;
                print!("{}", board[board.len() - 1]);
                if print_count == 0 {
                    break;
                }
            }
        }
        board.push(score);
        if board.len() > input {
            print_count -= 1;
            print!("{}", board[board.len() - 1]);
            if print_count == 0 {
                break;
            }
        }

        elfo = (elfo + board[elfo] + 1) % board.len();
        twinkels = (twinkels + board[twinkels] + 1) % board.len();
    }
}
