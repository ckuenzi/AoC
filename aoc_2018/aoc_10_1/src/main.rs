fn main() {
    let mut stars = include_str!("input.txt")
        .lines()
        .map(|line| {
            let nums = line
                .split(|c| c == '<' || c == ',' || c == '>' || c == ' ')
                .filter_map(|s| s.parse::<i64>().ok())
                .collect::<Vec<_>>();
            Star::new(nums[0], nums[1], nums[2], nums[3])
        })
        .collect::<Vec<_>>();
    let mut seconds = 0;
    let mut area_last = 30000000000;
    let mut area = 20000000000;
    let mut start = Vec2 { x: 0, y: 0 };
    let mut end = Vec2 { x: 0, y: 0 };
    while area < area_last {
        area_last = area;
        stars.iter_mut().for_each(|star| star.forward(1));

        start = Vec2 {
            x: stars.iter().map(|star| star.position.x).min().unwrap(),
            y: stars.iter().map(|star| star.position.y).min().unwrap(),
        };
        end = Vec2 {
            x: stars.iter().map(|star| star.position.x).max().unwrap(),
            y: stars.iter().map(|star| star.position.y).max().unwrap(),
        };

        area = (end.x - start.x) * (end.y - start.y);
        seconds += 1;
    }
    stars.iter_mut().for_each(|star| star.forward(-1));
    seconds -= 1;

    for y in start.y..end.y {
        for x in start.x..end.x {
            print!(
                "{}",
                match stars
                    .iter()
                    .any(|star| star.position.x == x && star.position.y == y)
                {
                    true => '#',
                    false => ' ',
                }
            );
        }
        println!()
    }

    println!("Seconds: {}", seconds);
}

struct Vec2 {
    x: i64,
    y: i64,
}
struct Star {
    position: Vec2,
    velocity: Vec2,
}

impl Star {
    fn forward(&mut self, steps: i64) {
        self.position.x += steps * self.velocity.x;
        self.position.y += steps * self.velocity.y;
    }

    fn new(x: i64, y: i64, vx: i64, vy: i64) -> Star {
        return Star {
            position: Vec2 { x: x, y: y },
            velocity: Vec2 { x: vx, y: vy },
        };
    }
}
