#[derive(Copy, Clone)]
struct Field {
    owner: i32,
    distance: i32,
}

fn main() {
    let coords: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| line.split(|c| c == ',').collect::<Vec<_>>())
        .map(|coord| {
            (
                coord[0].parse::<i32>().unwrap() as usize,
                coord[1][1..].parse::<i32>().unwrap() as usize,
            )
        })
        .collect();

    let size = (
        coords.iter().max().unwrap().0 * 2,
        coords.iter().max_by_key(|x| x.1).unwrap().1 * 2,
    );

    let coords = coords
        .iter()
        .map(|c| (c.0 + 100, c.1 + 100))
        .collect::<Vec<_>>();

    let mut grid = vec![
        vec![
            Field {
                owner: -1,
                distance: (size.0 + size.1) as i32
            };
            size.0
        ];
        size.1
    ];

    for element in coords.iter().enumerate() {
        for x in 0..size.0 {
            for y in 0..size.1 {
                let distance = ((element.1).0 as i32 - (x as i32)).abs()
                    + ((element.1).1 as i32 - y as i32).abs();
                if grid[y][x].distance == distance {
                    grid[y][x].owner = -2;
                } else if grid[y][x].distance > distance {
                    grid[y][x].distance = distance;
                    grid[y][x].owner = element.0 as i32;
                }
            }
        }
    }

    let mut areas = vec![(0, true); coords.len() + 1];
    for y in 0..size.1 {
        for x in 0..size.0 {
            if grid[y][x].owner >= 0 {
                if x == 0 || y == 0 || x >= size.0 - 1 || y >= size.1 - 1 {
                    areas[grid[y][x].owner as usize].1 = false;
                }
                areas[grid[y][x].owner as usize].0 += 1;
            }
        }
    }
    println!("{:?}", areas.iter().filter(|x| x.1).max());
}
