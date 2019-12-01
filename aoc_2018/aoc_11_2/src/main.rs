fn main() {
    let input = 3463;
    let size: usize = 300;

    let mut grid = vec![[0 as i32; 301]; 301];

    for y in 1..=size {
        for x in 1..=size {
            let rack_id = x as i32 + 10;
            let power_level = (((((rack_id * y as i32) + input) * rack_id) % 1000) / 100) - 5;
            grid[y][x] = power_level;
        }
    }

    let mut max_energy = 0;
    let mut max_coord = (0, 0);
    let mut max_square = 0;
    for square in 1..=300 {
        println!("Square {}", square);
        for y in 1..=(size.wrapping_sub(square - 1)) {
            for x in 1..=(size.wrapping_sub(square - 1)) {
                let mut energy = 0;
                for yp in 0..=square - 1 {
                    for xp in 0..=square - 1 {
                        energy += grid[y + yp][x + xp];
                    }
                }
                if max_energy < energy {
                    max_energy = energy;
                    max_coord = (x, y);
                    max_square = square;
                }
            }
        }
    }

    println!(
        "{},{},{} with energy {}",
        max_coord.0, max_coord.1, max_square, max_energy
    );
}
