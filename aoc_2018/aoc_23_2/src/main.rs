use std::collections::HashMap;

fn main() {
    let bots: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let nums: Vec<_> = line
                .split(|c| c == '<' || c == ',' || c == '>' || c == '=')
                .filter_map(|n| n.parse::<i32>().ok())
                .collect();
            Bot::new(nums[0], nums[1], nums[2], nums[3])
        }).collect();
    let range = 15;
    let mut res = Vec3::new(0, 0, 0);
    let origin = Vec3::new(0, 0, 0);
    let mut scale: i32 = 100000000;
    while scale >= 1 {
        println!("Scale: {}", scale);
        let mut points: HashMap<Vec3, i32> = HashMap::new();
        for x in res.x - range..=res.x + range {
            for y in res.y - range..=res.y + range {
                for z in res.z - range..=res.z + range {
                    for bot in bots.iter() {
                        let bot = Bot::scale(bot, scale);
                        let point = Vec3::new(x, y, z);
                        let range = match scale == 1 {
                            true => bot.r,
                            false => bot.r + 1,
                        };
                        if Vec3::distance(&point, &bot.pos) <= range {
                            let pointer = points.entry(point).or_insert(0);
                            *pointer += 1;
                        }
                    }
                }
            }
        }
        let best_point = points
            .iter()
            .max_by(|a, b| {
                if a.1 != b.1 {
                    a.1.cmp(&b.1)
                } else {
                    Vec3::distance(&b.0, &origin).cmp(&Vec3::distance(&a.0, &origin))
                }
            }).unwrap();

        res = Vec3::new(
            best_point.0.x * 10,
            best_point.0.y * 10,
            best_point.0.z * 10,
        );

        scale /= 10;
    }
    println!("Result: {:#?}", (res.x + res.y + res.z) / 10);
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Bot {
    pos: Vec3,
    r: i32,
}

impl Bot {
    fn new(x: i32, y: i32, z: i32, r: i32) -> Bot {
        Bot {
            pos: Vec3::new(x, y, z),
            r: r,
        }
    }

    fn scale(bot: &Bot, scale: i32) -> Bot {
        Bot {
            pos: Vec3::new(bot.pos.x / scale, bot.pos.y / scale, bot.pos.z / scale),
            r: bot.r / scale,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn new(x: i32, y: i32, z: i32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
    fn distance(a: &Vec3, b: &Vec3) -> i32 {
        (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
    }
}
