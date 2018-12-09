fn main() {
    let players = 428;
    let marbles = 70825;

    let mut player = 0;
    let mut scores: Vec<u64> = vec![0; players];
    let mut position = 1;
    let mut circle = vec![0];

    for marble in 1..=marbles {
        if marble % 23 == 0 {
            if position < 7 {
                position += circle.len();
            }
            position = (position - 7) % (circle.len());
            scores[player] += marble + circle.remove(position);
        } else {
            position = (position + 1) % (circle.len()) + 1;
            circle.insert(position, marble);
        }
        player = (player + 1) % players;
    }

    println!("{}", scores.iter().max().unwrap());
}
