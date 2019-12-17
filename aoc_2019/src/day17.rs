use super::intscript::Computer;
use hashbrown::HashMap;
use itertools::Itertools;
use std::fmt;

#[aoc(day17, part1)]
fn part1(input: &str) -> i32 {
    let mut computer = Computer::new_from_text(input);
    let (map, _) = get_map(&mut computer);
    let mut result = 0;

    for scaffold in map.values().filter(|n| n.tile == '#') {
        let mut intersection = true;
        for adj in scaffold.pos.adjacent() {
            if !map.contains_key(&adj) || map.get(&adj).unwrap().tile != '#' {
                intersection = false;
                break;
            }
        }
        if intersection {
            result += scaffold.pos.x * scaffold.pos.y;
        }
    }
    result
}

#[aoc(day17, part2)]
fn part2(input: &str) -> i32 {
    /*let mut sequence = get_sequence(input);
    println!("{:?}", sequence);
    for offset in 1..sequence.len() {
        let mut count = 0;
        for i in 0..sequence.len() - offset {
            //println!("{} - {} -> {}", sequence[i], sequence[i+offset] , sequence[i].eq(&sequence[i + offset]));
            if sequence[i].eq(&sequence[i + offset]) {
                count += 1;
            } else {
                if count > 3 {
                    for j in (i-count)..i{
                        print!("{:?}", sequence[j]);
                    }
                    println!();
                }
                count = 0;
            }
            println!("{}", count);
        }
    }*/

    let mut computer = Computer::new_from_text(input);
    computer.write(0, 2).run();
    computer.input_string("B,C,C,B,C,A,B,A,C,A\n"); //sequence
    computer.input_string("R,12,L,6,L,6,L,8\n"); //A
    computer.input_string("L,4,L,6,L,8,L,12\n"); //B
    computer.input_string("L,8,R,12,L,12\n"); //C
    computer.input_string("n\n"); //feed?
    computer.run();
    //println!("{}", computer.output_string());
    let out = computer.get_outputs();
    println!("{}", out[out.len()-1]);

    //println!("{:?}", sequence);

    0
}

fn get_sequence(input: &str) -> Vec<String> {
    let mut computer = Computer::new_from_text(input);
    let (mut map, mut pos) = get_map(&mut computer);
    let mut dir = Dir::Up;
    let mut sequence = vec![];
    loop {
        if let Some(turn) = turn(&map, &pos, &mut dir) {
           //sequence = format!("{}{:?},", sequence, turn);
            let steps = walk(&mut map, &mut pos, &dir);
            sequence.push(format!("{:?},{:?}", turn, steps));
        } else {
            break;
        }

        //sequence = format!("{}{},", sequence, steps);

    }
    //print_map(&map);
    sequence
}

fn walk(map: &mut HashMap<Pos, Node>, pos: &mut Pos, dir: &Dir) -> u32 {
    let mut steps = 0;
    loop {
        let next_step = pos.step(&dir);
        if map.contains_key(&next_step) && map.get(&next_step).unwrap().tile == '#' {
            steps += 1;
            map.get_mut(&pos).unwrap().visited = true;
            pos.x = next_step.x;
            pos.y = next_step.y;
        } else {
            break;
        }
    }

    steps
}

fn turn(map: &HashMap<Pos, Node>, pos: &Pos, dir: &mut Dir) -> Option<Dir> {
    let mut next = Pos::new(0, 0);
    for adj in pos.adjacent() {
        if let Some(node) = map.get(&adj) {
            if node.tile == '#' && !node.visited {
                next = node.pos.clone();
                continue;
            }
        }
    }
    if let Some(new_dir) = Dir::from_pos(&Pos::new(next.x - pos.x, next.y - pos.y)) {
        let out = dir.turn_direction(&new_dir);
        *dir = new_dir;
        return Some(out);
    } else {
        return None;
    }
}

impl Dir {
    pub fn from_pos(pos: &Pos) -> Option<Dir> {
        if pos.x.abs() > 1 || pos.y.abs() > 1 {
            return None;
        }
        Some(match (pos.x, pos.y) {
            (-1, 0) => Dir::Left,
            (1, 0) => Dir::Right,
            (0, 1) => Dir::Down,
            (0, -1) => Dir::Up,
            _ => panic!("Cant be converted to dir"),
        })
    }

    pub fn turn_direction(&self, next: &Dir) -> Dir {
        match self {
            Dir::Up => match next {
                Dir::Right => Dir::Right,
                Dir::Left => Dir::Left,
                _ => panic!("Invalid turn"),
            },
            Dir::Down => match next {
                Dir::Right => Dir::Left,
                Dir::Left => Dir::Right,
                _ => panic!("Invalid turn"),
            },
            Dir::Right => match next {
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
                _ => panic!("Invalid turn"),
            },
            Dir::Left => match next {
                Dir::Up => Dir::Right,
                Dir::Down => Dir::Left,
                _ => panic!("Invalid turn"),
            },
        }
    }
}

#[derive(Clone, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Debug for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::Up => 'U',
                Dir::Down => 'D',
                Dir::Left => 'L',
                Dir::Right => 'R',
            }
        )
    }
}

fn get_map(computer: &mut Computer) -> (HashMap<Pos, Node>, Pos) {
    let mut map = HashMap::new();

    let mut pos = Pos::new(0, 0);
    let mut start = Pos::new(0, 0);
    for input in computer.run().dump_outputs() {
        match input {
            10 => {
                pos.x = 0;
                pos.y += 1;
            }
            35 => {
                map.insert(
                    Pos::new(pos.x, pos.y),
                    Node::new(Pos::new(pos.x, pos.y), '#'),
                );
                pos.x += 1;
            }
            46 => {
                map.insert(
                    Pos::new(pos.x, pos.y),
                    Node::new(Pos::new(pos.x, pos.y), '.'),
                );
                pos.x += 1;
            }
            94 => {
                map.insert(
                    Pos::new(pos.x, pos.y),
                    Node::new(Pos::new(pos.x, pos.y), '#'),
                );
                start = pos.clone();
                pos.x += 1;
            }
            _ => panic!("u dun gooofd"),
        }
    }
    (map, start)
}

#[derive(Debug)]
struct Node {
    pos: Pos,
    tile: char,
    visited: bool,
}

impl Node {
    pub fn new(pos: Pos, tile: char) -> Node {
        Node {
            pos,
            tile,
            visited: false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }

    pub fn adjacent(&self) -> Vec<Pos> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|(x, y)| Pos::new(self.x + x, self.y + y))
            .collect()
    }

    pub fn step(&self, dir: &Dir) -> Pos {
        let mut next = self.clone();
        match dir {
            Dir::Up => next.y -= 1,
            Dir::Down => next.y += 1,
            Dir::Left => next.x -= 1,
            Dir::Right => next.x += 1,
        }
        next
    }
}

#[test]
fn turn_test() {
    assert_eq!(Dir::Up.turn_direction(&Dir::Right), Dir::Right);
    assert_eq!(Dir::Up.turn_direction(&Dir::Left), Dir::Left);
    assert_eq!(Dir::Up.turn_direction(&Dir::Right), Dir::Right);
    assert_eq!(Dir::from_pos(&Pos::new(10 - 10, 11 - 10)), Some(Dir::Right));
    assert_eq!(Dir::from_pos(&Pos::new(9 - 10, 10 - 10)), Some(Dir::Up));
    assert_eq!(Dir::from_pos(&Pos::new(11 - 10, 10 - 10)), Some(Dir::Down));
    assert_eq!(Dir::from_pos(&Pos::new(10 - 10, 9 - 10)), Some(Dir::Left));
}
