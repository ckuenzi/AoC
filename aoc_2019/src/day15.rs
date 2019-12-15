use super::intscript::Computer;
use hashbrown::HashMap;

#[aoc(day15, part1)]
fn part1(input: &str) -> String {
    let mut maze = HashMap::<(i32, i32), Node>::new();
    let mut moves: Vec<((i32, i32), u64)> = vec![((0, 0), 0)];
    maze.insert(
        (0, 0),
        Node::new((0, 0), 1, 0, Computer::new_from_text(input)),
    );

    while !moves.is_empty() {
        let (p, d) = moves.pop().unwrap();
        for dir in vec![((-1, 0), 1), ((1, 0), 2), ((0, -1), 3), ((0, 1), 4)] {
            let mut new_bot = maze.get(&p).unwrap().robot.clone();
            let pn = (
                maze.get(&p).unwrap().p.0 + (dir.0).0,
                maze.get(&p).unwrap().p.1 + (dir.0).1,
            );
            if maze.contains_key(&pn)
                && maze.get(&pn).unwrap().distance <= maze.get(&p).unwrap().distance
            {
                continue;
            }
            let tn = new_bot.siso(dir.1);
            maze.insert(
                pn,
                Node::new(pn, tn, maze.get(&p).unwrap().distance + 1, new_bot),
            );
            if tn != 0 {
                moves.push((pn, d + 1));
            }
        }
    }

    let mut oxygen = (0, 0);
    let mut part1 = 0;
    for node in maze.iter_mut() {
        if node.1.tile == 2 {
            oxygen = *node.0;
            part1 = node.1.distance;
        }
        node.1.distance = std::u32::MAX;
    }

    moves.push((oxygen, 0));
    while !moves.is_empty() {
        let (p, d) = moves.pop().unwrap();
        maze.get_mut(&p).unwrap().distance = d as u32;
        for dir in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let pn = (
                maze.get(&p).unwrap().p.0 + dir.0,
                maze.get(&p).unwrap().p.1 + dir.1,
            );
            if !maze.contains_key(&pn)
                || (maze.get(&pn).unwrap().distance <= maze.get(&p).unwrap().distance)
                || maze.get(&pn).unwrap().tile == 0
            {
                continue;
            }
            moves.push((pn, d + 1));
        }
    }

    let part2 = maze
        .values()
        .filter(|n| n.tile == 1)
        .max_by_key(|n| n.distance)
        .unwrap()
        .distance;

    format!("{}\n{}", part1, part2)
}

struct Node {
    p: (i32, i32),
    tile: i64,
    distance: u32,
    robot: Computer,
}

impl Node {
    pub fn new(p: (i32, i32), tile: i64, distance: u32, robot: Computer) -> Node {
        Node {
            p,
            tile,
            distance,
            robot,
        }
    }
}
