fn main() {
    const SIZE: usize = 1000;
    const ELVES: usize = 1408;
    let mut fabric: [[i32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut have: [i32; ELVES] = [0; ELVES];
    let mut want: [i32; ELVES] = [0; ELVES];
    let mut result = 0;

    let lines = include_str!("input.txt").lines();

    for line in lines {
        let res = line
            .split(|c| c == '#' || c == ' ' || c == '@' || c == ',' || c == ':' || c == 'x')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();
        let index = res[0] as usize;
        want[index] = (res[3] as i32) * (res[4] as i32);
        for x in res[1]..res[1] + res[3] {
            for y in res[2]..res[2] + res[4] {
                let fab_id = fabric[x][y];

                if fab_id == -1 {
                    continue;
                } else if fab_id == 0 {
                    fabric[x][y] = res[0] as i32;
                } else if fab_id != res[0] as i32 {
                    fabric[x][y] = -1;
                    result += 1;
                }
            }
        }
    }

    println!("Number of overlaps {}", result);

    for x in 0..SIZE {
        for y in 0..SIZE {
            let fab_id = fabric[x][y] as usize;
            if fab_id > 10000 {
                continue;
            }
            have[fab_id] += 1;
        }
    }

    for i in 1..ELVES {
        if want[i] == have[i] {
            println!("Intact square {}", i)
        }
    }
}
