fn main() {
    let points = include_str!("input.txt").lines().map(|line| {
        let nums: Vec<_> = line
            .split(|c| c == '<' || c == ',' || c == '>' || c == '=')
            .filter_map(|n| n.parse::<i32>().ok())
            .collect();
        (Vec3::new(nums[0], nums[1], nums[2]), nums[3])
    });
    let strong_point = points.clone().max_by_key(|p| p.1).unwrap();
    let center = strong_point.0;
    let range = strong_point.1;

    println!(
        "{:?}",
        points
            .filter(|p| Vec3::distance(&p.0, &center) <= range)
            .count()
    );
}

#[derive(Debug, Clone)]
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
