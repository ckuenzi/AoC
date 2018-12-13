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
    let mut ids = 0;

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
                    removed: false,
                    id: ids,
                });
                ids += 1;
            }
            '<' => {
                map[y.0][x.0].rail_type = Some(Type::Horizontal);
                carts.push(Cart {
                    pos: Vec2 { x: x.0, y: y.0 },
                    dir: Direction::Left,
                    next_turn: Direction::Left,
                    removed: false,
                    id: ids,
                });
                ids += 1;
            }
            '|' => map[y.0][x.0].rail_type = Some(Type::Vertical),
            'v' => {
                map[y.0][x.0].rail_type = Some(Type::Vertical);
                carts.push(Cart {
                    pos: Vec2 { x: x.0, y: y.0 },
                    dir: Direction::Down,
                    next_turn: Direction::Left,
                    removed: false,
                    id: ids,
                });
                ids += 1;
            }
            '^' => {
                map[y.0][x.0].rail_type = Some(Type::Vertical);
                carts.push(Cart {
                    pos: Vec2 { x: x.0, y: y.0 },
                    dir: Direction::Up,
                    next_turn: Direction::Left,
                    removed: false,
                    id: ids,
                });
                ids += 1;
            }
            '+' => map[y.0][x.0].rail_type = Some(Type::Intersection),
            '\\' => map[y.0][x.0].rail_type = Some(Type::CornerBck),
            '/' => map[y.0][x.0].rail_type = Some(Type::CornerFwd),
            _ => panic!("Unknown Symbol: {}", x.1),
        })
    });
    let mut carts_left = carts.len();
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

    let n_carts = carts.len();

    'outer: loop {
        let mut copy: Vec<(usize, usize, i32)> = vec![];
        let mut to_remove: Vec<i32> = vec![];

        for cart in carts.iter() {
            if !cart.removed {
                copy.push((cart.pos.x, cart.pos.y, cart.id));
            }
        }

        carts.sort();
        for i in 0..n_carts {
            let cart = &mut carts[i];
            if to_remove.contains(&cart.id) || cart.removed {
                continue;
            }
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

            for other in copy.iter_mut() {
                if other.0 == cart.pos.x && other.1 == cart.pos.y && other.2 != cart.id {
                    to_remove.push(cart.id);
                    to_remove.push(other.2);
                    cart.removed = true;
                    println!("Collision at {},{}", cart.pos.x, cart.pos.y);
                    carts_left -= 2;
                }

                if other.2 == cart.id {
                    other.0 = cart.pos.x;
                    other.1 = cart.pos.y;
                }
            }
        }

        for cart in carts.iter_mut() {
            if to_remove.contains(&cart.id) {
                cart.removed = true;
            }
        }
        if carts_left == 1 {
            break 'outer;
        }
    }
    for cart in carts {
        if !cart.removed {
            println!("Last cart Position: {},{}", cart.pos.x, cart.pos.y);
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
    removed: bool,
    id: i32,
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
