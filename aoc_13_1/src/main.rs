use std::cmp::Ordering;

fn main() {
    const SIZE: usize = 150;
    let mut map = vec![
        [Node {
            up: None,
            down: None,
            left: None,
            right: None,
            rail_type: None
        }; SIZE];
        SIZE
    ];

    let mut carts = vec![];

    include_str!("input.txt").lines().enumerate().for_each(|y| {
        y.1.chars().enumerate().for_each(|x| match x.1 {
            ' ' => (),
            '-' => map[y.0][x.0].rail_type = Some(Type::Horizontal),
            '>' => {
                map[y.0][x.0].rail_type = Some(Type::Horizontal);
                carts.push(Cart {
                    pos: Vec2 { x: x.0, y: y.0 },
                    dir: Direction::Right,
                    next_turn: Direction::Left,
                });
            }
            '<' => {
                map[y.0][x.0].rail_type = Some(Type::Horizontal);
                carts.push(Cart {
                    pos: Vec2 { x: x.0, y: y.0 },
                    dir: Direction::Left,
                    next_turn: Direction::Left,
                });
            }
            '|' => map[y.0][x.0].rail_type = Some(Type::Vertical),
            'v' => {
                map[y.0][x.0].rail_type = Some(Type::Vertical);
                carts.push(Cart {
                    pos: Vec2 { x: x.0, y: y.0 },
                    dir: Direction::Down,
                    next_turn: Direction::Left,
                });
            }
            '^' => {
                map[y.0][x.0].rail_type = Some(Type::Vertical);
                carts.push(Cart {
                    pos: Vec2 { x: x.0, y: y.0 },
                    dir: Direction::Up,
                    next_turn: Direction::Left,
                });
            }
            '+' => map[y.0][x.0].rail_type = Some(Type::Intersection),
            '\\' => map[y.0][x.0].rail_type = Some(Type::CornerBck),
            '/' => map[y.0][x.0].rail_type = Some(Type::CornerFwd),
            _ => panic!("Unknown Symbol: {}", x.1),
        })
    });

    for y in 0..SIZE {
        for x in 0..SIZE {
            if y > 0 && map[y - 1][x].rail_type.is_some() {
                map[y][x].up = Some(Vec2 { y: y - 1, x: x });
            }
            if x > 0 && map[y][x - 1].rail_type.is_some() {
                map[y][x].left = Some(Vec2 { y: y, x: x - 1 });
            }
            if y < SIZE - 1 && map[y + 1][x].rail_type.is_some() {
                map[y][x].down = Some(Vec2 { y: y + 1, x: x });
            }
            if x < SIZE - 1 && map[y][x + 1].rail_type.is_some() {
                map[y][x].right = Some(Vec2 { y: y, x: x + 1 });
            }
        }
    }

    'outer: loop {
        let mut taken: Vec<(usize, usize)> = vec![];
        let mut collisions = 0;
        carts.sort();
        for cart in carts.iter_mut() {
            cart.dir = match map[cart.pos.y][cart.pos.x].rail_type.unwrap() {
                Type::Vertical | Type::Horizontal => cart.dir,
                Type::CornerFwd => match cart.dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                },
                Type::CornerBck => match cart.dir {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                },
                Type::Intersection => match cart.dir {
                    Direction::Up => match cart.next_turn {
                        Direction::Right => Direction::Right,
                        Direction::Left => Direction::Left,
                        Direction::Up | Direction::Down => Direction::Up,
                    },
                    Direction::Down => match cart.next_turn {
                        Direction::Right => Direction::Left,
                        Direction::Left => Direction::Right,
                        Direction::Up | Direction::Down => Direction::Down,
                    },
                    Direction::Left => match cart.next_turn {
                        Direction::Right => Direction::Up,
                        Direction::Left => Direction::Down,
                        Direction::Up | Direction::Down => Direction::Left,
                    },
                    Direction::Right => match cart.next_turn {
                        Direction::Right => Direction::Down,
                        Direction::Left => Direction::Up,
                        Direction::Up | Direction::Down => Direction::Right,
                    },
                },
            };
            cart.next_turn = match map[cart.pos.y][cart.pos.x].rail_type.unwrap() {
                Type::Intersection => match cart.next_turn {
                    Direction::Left => Direction::Up,
                    Direction::Up | Direction::Down => Direction::Right,
                    Direction::Right => Direction::Left,
                },
                _ => cart.next_turn,
            };
            cart.pos = match cart.dir {
                Direction::Up => map[cart.pos.y][cart.pos.x].up.unwrap(),
                Direction::Down => map[cart.pos.y][cart.pos.x].down.unwrap(),
                Direction::Left => map[cart.pos.y][cart.pos.x].left.unwrap(),
                Direction::Right => map[cart.pos.y][cart.pos.x].right.unwrap(),
            };

            for pos in taken.iter() {
                if pos.0 == cart.pos.x && pos.1 == cart.pos.y {
                    collisions += 1;
                }
            }
            taken.push((cart.pos.x, cart.pos.y));
            if collisions > 0 {
                println!("Collision at: {},{}", cart.pos.x, cart.pos.y);
                break 'outer;
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Type {
    Vertical,
    Horizontal,
    Intersection,
    CornerFwd,
    CornerBck,
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
struct Cart {
    pos: Vec2,
    dir: Direction,
    next_turn: Direction,
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        if self.pos.y == other.pos.y {
            self.pos.x.cmp(&other.pos.x)
        } else {
            self.pos.y.cmp(&other.pos.y)
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        if self.pos.y == other.pos.y {
            Some(self.pos.x.cmp(&other.pos.x))
        } else {
            Some(self.pos.y.cmp(&other.pos.y))
        }
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Self) -> bool {
        (self.pos.x, self.pos.y) == (other.pos.x, other.pos.y)
    }
}

impl Eq for Cart {}

#[derive(Copy, Clone)]
struct Vec2 {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
struct Node {
    up: Option<Vec2>,
    down: Option<Vec2>,
    left: Option<Vec2>,
    right: Option<Vec2>,
    rail_type: Option<Type>,
}
