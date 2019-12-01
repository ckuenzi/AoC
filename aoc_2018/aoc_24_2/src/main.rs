use std::cmp::Reverse;

fn main() {
    let input = include_str!("input.txt");
    let armies: Vec<_> = input.split("\r\n\r\n").collect();
    let mut groups0: Vec<Group> = armies[0].lines().skip(1).map(|l| parse(l, true)).collect();
    armies[1]
        .lines()
        .skip(1)
        .for_each(|l| groups0.push(parse(l, false)));

    let mut boost = 0;
    'boost: loop {
        let mut groups = groups0.clone();
        boost += 1;
        for i in 0..groups.len() {
            if groups[i].team {
                groups[i].ap += boost;
            }
        }
        'battle: loop {
            let mut total_killed = 0;
            let mut attackers: Vec<_> = (0..groups.len()).collect();
            attackers.sort_by_key(|&i| Reverse((groups[i].ep(), groups[i].init)));
            let mut attacks: Vec<(usize, usize)> = vec![];
            for a in attackers {
                let mut defenders: Vec<_> = (0..groups.len())
                    .filter(|&d| !groups[d].is_defending && groups[a].team != groups[d].team)
                    .collect();
                defenders.sort_by_key(|&d| {
                    Reverse((groups[a].damage(&groups[d]), groups[d].ep(), groups[d].init))
                });
                if defenders.is_empty() {
                    continue;
                }
                let d = defenders[0];
                if groups[a].damage(&groups[d]) > 0 {
                    attacks.push((a, d));
                    groups[d].is_defending = true;
                }
            }

            attacks.sort_by_key(|&t| Reverse(groups[t.0].init));

            for (a, d) in attacks {
                if groups[a].units <= 0 {
                    continue;
                }
                groups[d].units -= groups[a].damage(&groups[d]) / groups[d].hp;
                total_killed += groups[a].damage(&groups[d]) / groups[d].hp;
            }

            let mut army1_alive = false;
            let mut army2_alive = false;
            for i in 0..groups.len() {
                groups[i].is_defending = false;
                if groups[i].team {
                    army1_alive = true;
                } else {
                    army2_alive = true;
                }
            }
            groups = groups
                .into_iter()
                .filter(|g| g.units >= 0)
                .collect::<Vec<Group>>();

            if !army1_alive && army2_alive {
                break 'battle;
            }
            if army1_alive && !army2_alive {
                let result = groups.iter().fold(0, |mut out, g| {
                    out += g.units;
                    out
                });
                println!("Boost: {}", boost);
                println!("Units: {}", result);
                break 'boost;
            }
            if total_killed == 0 {
                break 'battle;
            }
        }
    }
}

fn parse_damage(text: &str) -> Damage {
    match text {
        "radiation" => Damage::Radiation,
        "fire" => Damage::Fire,
        "slashing" => Damage::Slashing,
        "cold" => Damage::Cold,
        "bludgeoning" => Damage::Bludgeoning,
        _ => panic!("Unknown damage {}", text),
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Damage {
    Radiation,
    Fire,
    Slashing,
    Cold,
    Bludgeoning,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Group {
    team: bool,
    units: i32,
    hp: i32,
    ap: i32,
    init: i32,
    element: Damage,
    immune: Vec<Damage>,
    weak: Vec<Damage>,
    is_defending: bool,
}

impl Group {
    fn damage(&self, defender: &Group) -> i32 {
        let is_immune = defender.immune.contains(&self.element);
        let is_weak = defender.weak.contains(&self.element);
        if is_immune && is_weak {
            panic!("Cant be immune and weak at the same time");
        }
        if is_immune {
            return 0;
        }
        if is_weak {
            return self.units * self.ap * 2;
        }
        return self.units * self.ap;
    }

    fn ep(&self) -> i32 {
        self.ap * self.units
    }
}

fn parse(line: &str, is_immun: bool) -> Group {
    let numbers = line
        .split(' ')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<i32>>();
    let damage: Damage = parse_damage(
        line.split("does")
            .nth(1)
            .unwrap()
            .split(' ')
            .nth(2)
            .unwrap(),
    );

    let immun_str: Vec<_> = line.split(|c| c == '(' || c == ')').collect();

    let mut immune: Vec<Damage> = vec![];
    let mut weak: Vec<Damage> = vec![];

    if immun_str.len() == 3 {
        for part in immun_str[1].split("; ") {
            let mut words = part
                .split(|c| c == ' ' || c == ',')
                .filter(|w| w != &"" && w != &"to");
            let is_immun = words.next().unwrap() == "immune";
            for word in words {
                if is_immun {
                    immune.push(parse_damage(word));
                } else {
                    weak.push(parse_damage(word));
                }
            }
        }
    }

    Group {
        team: is_immun,
        units: numbers[0],
        hp: numbers[1],
        ap: numbers[2],
        init: numbers[3],
        element: damage,
        immune: immune,
        weak: weak,
        is_defending: false,
    }
}
