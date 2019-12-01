fn main() {
    let input = include_str!("test.txt");
    let armies: Vec<_> = input.split("\r\n\r\n").collect();
    let mut id_count = 0;
    let mut groups: Vec<Group> = armies[0].lines().skip(1).map(|l| parse(l, true)).collect();
    armies[1].lines().skip(1).for_each(|l| groups.push(parse(l, false)));
    for group in groups.iter_mut() {
        group.id = id_count;
        id_count += 1;
    }
 
    groups.sort_by(|a, b| {if a.ap * a.units != b.ap * b.units{
        (a.ap * a.units).cmp(&(b.ap * b.units))
    }else{
        a.init.cmp(&b.init)
    }});
    groups.reverse();
    //println!("{:#?}", groups);
    // for group in groups {
    //     println!("{}", group.ap * group.units);
    // }

    for a in 0..groups.len() {
        let mut max_damage = 0;
        let mut defender_id = 0;
        let mut targets: Vec<(usize, i32)> = vec![]; //id, damage
        for d in 0..groups.len() {
            if groups[d].is_immun == groups[a].is_immun || groups[a].is_paired || groups[d].is_paired{
                continue;
            }
            targets.push((groups[d].id, get_damage(&groups[a], &groups[d])));
            targets.sort_by(|a, b| {
                if a.1 != b.1{
                    b.1.cmp(&a.1)
                }else{
                    if get_ep(&groups[a.0]) != get_ep(&groups[b.0]){
                        get_ep(&groups[b.0]).cmp(&get_ep(&groups[a.0]))
                    }else{
                        groups[b.0].init.cmp(&groups[a.0].init)
                    }
                }
                
            });
        }
        if !targets.is_empty(){
            let d = index_from_id(targets[0].0, &groups);
            groups[d].is_attacker = false;
            groups[a].is_attacker = true;
            groups[d].is_attacker = false;
            groups[a].is_paired = true;
            groups[d].is_paired = true;
            groups[a].current_target = targets[0].0;
        }
    }
    
    println!("{:#?}", groups);

}

fn index_from_id(id: usize, groups: &Vec<Group>) -> usize{
    for i in 0..groups.len(){
        if groups[i].id == id{
            return i;
        }
    }
    panic!("cant find group with id {}", id);
}

fn get_ep(group: &Group) -> i32{
    group.ap * group.units
}

fn get_damage(attacker: &Group, defender : &Group) -> i32  {
    let is_immune = defender.immune.contains(&attacker.element);
    let is_weak = defender.weak.contains(&attacker.element);
    if is_immune && is_weak {
        panic!("Cant be immune and weak at the same time");
    }
    if is_immune {
        return 0;
    }
    if is_weak {
        return attacker.units * attacker.ap * 2;
    }
    return attacker.units * attacker.ap;
}

fn parse(line: &str, is_immun: bool) -> Group{
    let numbers = line.split(' ').filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>();
    let damage: Damage = parse_damage(line.split("does").nth(1).unwrap().split(' ').nth(2).unwrap());

    let immun_str: Vec<_> = line.split(|c| c=='(' || c==')').collect();

    let mut immune: Vec<Damage> = vec![];
    let mut weak: Vec<Damage> = vec![];

    if immun_str.len() == 3 {
        for part in immun_str[1].split("; "){
            let mut words = part.split(|c| c==' ' || c==',').filter(|w| w!=&"" && w!=&"to");
            let is_immun = words.next().unwrap() == "immune";
            for word in words{
                if is_immun{
                    immune.push(parse_damage(word));
                }else {
                    weak.push(parse_damage(word));
                }
            }
            
        }
    }

    

    Group{is_attacker: false, is_immun:is_immun, units:numbers[0],hp:numbers[1],ap:numbers[2],init:numbers[3],element:damage, immune: immune, weak:weak, id:0, current_target:0, is_paired: false}
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
    id: usize,
    is_immun: bool,
    units: i32,
    hp: i32,
    ap: i32,
    init: i32,
    element: Damage,
    immune: Vec<Damage>,
    weak: Vec<Damage>,
    current_target: usize,
    is_paired: bool,
    is_attacker: bool,
}

