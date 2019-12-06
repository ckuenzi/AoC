use std::collections::HashMap;

fn main() {
    let mut orbits = HashMap::<String, Orbit>::new();
    include_str!("inputs\\day06a.txt")
        .lines()
        .map(|s| s.split(')').collect::<Vec<_>>())
        .for_each(|a| {
            orbits
                .entry(a[0].to_string())
                .or_insert(Orbit::new())
                .satellites
                .push(a[1].to_string());
            orbits.entry(a[1].to_string()).or_insert(Orbit::new());
        });
    println!("{}", count_orbits(&mut orbits, "COM".to_string()));
    println!(
        "{}",
        find_path(&mut orbits, "YOU".to_string(), "SAN".to_string()) - 3
    );
}

fn find_path(system: &mut HashMap<String, Orbit>, from: String, to: String) -> u32 {
    if from == to {
        return 1;
    }
    let mut current = system.get_mut(&from).unwrap();
    current.visited = true;
    let mut targets = current.satellites.clone();
    targets.push(current.parent.clone());
    for target in targets {
        let next = system.get_mut(&target).unwrap();
        if next.visited {
            continue;
        }
        let res = find_path(system, target, to.clone());
        if res > 0 {
            return res + 1;
        }
    }
    0
}

fn count_orbits(system: &mut HashMap<String, Orbit>, origin: String) -> u32 {
    let current_orbit_count = system.get(&origin).unwrap().orbit_count;
    let satellites = system.get(&origin).unwrap().satellites.clone();

    let mut total = current_orbit_count;
    for satellite in satellites {
        system.get_mut(&satellite).unwrap().orbit_count = current_orbit_count + 1;
        system.get_mut(&satellite).unwrap().parent = origin.clone();
        total += count_orbits(system, satellite);
    }
    total
}

#[derive(Debug, Clone)]
struct Orbit {
    orbit_count: u32,
    visited: bool,
    parent: String,
    satellites: Vec<String>,
}

impl Orbit {
    fn new() -> Orbit {
        Orbit {
            orbit_count: 0,
            satellites: vec![],
            parent: "".to_string(),
            visited: false,
        }
    }
}
