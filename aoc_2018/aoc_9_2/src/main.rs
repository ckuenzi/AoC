fn main() {
    let players = 428;
    let marbles = 7082500;
    let mut id_getter = (1..).into_iter();
    let mut player = 0;
    let mut scores: Vec<u64> = vec![0; players];
    let mut circle = vec![(0, 0, 0); marbles];
    let mut id = 0;

    for marble in 1..=marbles {
        if marble % 23 == 0 {
            for _ in 0..7 {
                id = circle[id].2;
            }
            scores[player] += (marble + circle[id].0) as u64;
            let next_id = circle[id].1;
            let prev_id = circle[id].2;
            circle[next_id].2 = prev_id;
            circle[prev_id].1 = next_id;
            id = next_id;
        } else {
            let next_id = circle[id].1;
            let next_id2 = circle[next_id].1;
            let new_id = id_getter.next().unwrap();
            circle[new_id] = (marble, next_id2, next_id);
            circle[next_id].1 = new_id;
            circle[next_id2].2 = new_id;
            id = new_id;
        }
        player = (player + 1) % players;
    }
    println!("{}", scores.iter().max().unwrap());
}
