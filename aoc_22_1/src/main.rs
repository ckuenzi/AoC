fn main() {
    let input = include_str!("test.txt")
        .split(|c| c == ' ' || c == '\n' || c == '\r' || c == ',')
        .filter_map(|n| n.parse::<usize>().ok())
        .collect::<Vec<_>>();
    let depth = input[0];
    let targetx = input[1];
    let targety = input[2];

    let mut map: Vec<Vec<Node>> = vec![vec![Node::new(); targetx + 1]; targety + 1];

    let mut result = 0;

    for y in 0..=targety {
        for x in 0..=targetx {
            if x == 0 {
                map[y][x].gindex = y * 48271;
            } else if y == 0 {
                map[y][x].gindex = x * 16807;
            } else if x == targetx && y == targety {
                map[y][x].cost = 0;
            } else {
                map[y][x].gindex = map[y.wrapping_sub(1)][x].erosion_level
                    * map[y][x.wrapping_sub(1)].erosion_level;
            }
            map[y][x].erosion_level = (map[y][x].gindex + depth) % 20183;
            map[y][x].cost = map[y][x].erosion_level % 3;
            result += map[y][x].cost;
        }
    }

    println!("{}", result);
}

#[derive(Debug, Clone)]
struct Node {
    gindex: usize,
    erosion_level: usize,
    cost: usize,
}

impl Node {
    fn new() -> Self {
        Node {
            gindex: 0,
            erosion_level: 0,
            cost: 0,
        }
    }
}
