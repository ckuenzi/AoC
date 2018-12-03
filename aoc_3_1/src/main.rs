fn main() {
    const SIZE: usize = 1001;
    const ELVES: usize = 1408;
    let mut fabric: [[i32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut scores: [i32; ELVES] = [0; ELVES];
    let mut want: [i32; ELVES] = [0; ELVES];
    let mut result = 0;
    let squares = include_str!("input.txt").lines().map(|x| parse_line(x));

    for square in squares {
        let index = square.id as usize;
        want[index] = square.height * square.length;
        for x in square.x..square.x + square.length {
            for y in square.y..square.y + square.height {
                let x = x as usize;
                let y = y as usize;
                let fab_id = fabric[x][y];

                if fab_id == -1 {
                    continue;
                } else if fab_id == 0 {
                    fabric[x][y] = square.id;
                } else if fab_id != square.id {
                    fabric[x][y] = -1;
                    result += 1;
                }
            }
        }
    }
    println!("Number of overlaps {}", result);

    for x in 0..SIZE {
        for y in 0..SIZE {
            let x = x as usize;
            let y = y as usize;
            let fab_id = fabric[x][y] as usize;
            if fab_id > 10000 {
                continue;
            }
            scores[fab_id] += 1;
        }
    }

    for i in 1..ELVES {
        if want[i] == scores[i] {
            println!("Intact square {}", i)
        }
    }
}

struct Square {
    id: i32,
    x: i32,
    y: i32,
    length: i32,
    height: i32,
}

fn parse_line(s: &str) -> Square {
    let mut out = Square {
        id: 0,
        x: 0,
        y: 0,
        height: 0,
        length: 0,
    };

    out.id = int_from_substr_pattern(s, &"#", &" ");
    out.x = int_from_substr_pattern(s, &"@ ", &",");
    out.y = int_from_substr_pattern(s, &",", &":");
    out.length = int_from_substr_pattern(s, &": ", &"x");
    out.height = *&s[s.find("x").unwrap() + 1..].parse::<i32>().unwrap();
    out
}

fn int_from_substr_pattern(input: &str, start: &str, end: &str) -> i32 {
    *&input[input.find(start).unwrap() + start.len()..input.find(end).unwrap()]
        .parse::<i32>()
        .unwrap()
}
