use hashbrown::HashMap;
use itertools::Itertools;

#[aoc(day14, part1)]
fn part1(input: &str) -> u64 {
    let mut graph = get_graph(input);
    calc_ore(&mut graph, 1)
}

#[aoc(day14, part2)]
fn part2(input: &str) -> u64 {
    let mut graph = get_graph(input);
    let max: u64 = 1000000000000;
    let mut upper = 1;
    while calc_ore(&mut graph, upper) < max {
        upper *= 2;
    }
    let mut lower = upper / 2 + 1;
    let mut middle = 0;
    while upper - lower > 1 {
        middle = (upper + lower + 1) / 2;
        match calc_ore(&mut graph, middle) < max {
            true => lower = middle,
            false => upper = middle,
        }
    }
    middle
}

macro_rules! get_node {
    ($n:expr, $g:ident) => {{
        $g.get_mut(&$n).unwrap()
    }};
}

fn calc_ore(graph: &mut HashMap<String, Node>, fuel: u64) -> u64 {
    get_node!(*"FUEL", graph).used = fuel;

    let mut current = "FUEL".to_string();
    while !current.eq(&"ORE") {
        get_node!(current, graph).visited = true;
        for prev in get_node!(current, graph).incoming.clone() {
            get_node!(prev.0, graph).visits += 1;
            let reactions = (get_node!(current, graph).used + 1) / prev.2;
            get_node!(prev.0, graph).used += reactions * prev.1;
        }
        for (name, node) in graph.iter() {
            if node.children - node.visits == 0 && !node.visited {
                current = name.clone();
                break;
            }
        }
    }
    let res = get_node!(*"ORE", graph).used;
    for node in graph.values_mut() {
        node.used = 0;
        node.visited = false;
        node.visits = 0;
    }
    res
}

fn get_graph(input: &str) -> HashMap<String, Node> {
    let mut graph = HashMap::<String, _>::new();
    let mut parents = Vec::<String>::new();
    for line in input.lines() {
        let parts = line.split(|c| c == '=' || c == '>').collect::<Vec<_>>();
        let left = parts[0].trim().replace(", ", ",");
        let parts = parts[2].trim().split(' ').collect_vec();
        let to = parts[1].to_string();
        let product = parts[0].parse::<u64>().unwrap();

        for connection in left.split(',') {
            let parts = connection.split(' ').collect_vec();
            let from = parts[1].to_string();
            let cost = parts[0].parse::<u64>().unwrap();
            parents.push(from.clone());
            graph
                .entry(to.clone())
                .or_insert(Node::new())
                .incoming
                .push((from, cost, product));
        }
    }
    graph.insert("ORE".to_string(), Node::new());

    for parent in parents {
        get_node!(parent, graph).children += 1;
    }
    graph
}

#[derive(Debug)]
struct Node {
    incoming: Vec<(String, u64, u64)>,
    children: u64,
    visits: u64,
    used: u64,
    visited: bool,
}

impl Node {
    fn new() -> Node {
        Node {
            incoming: vec![],
            used: 0,
            children: 0,
            visited: false,
            visits: 0,
        }
    }
}

#[test]
fn big_part1() {
    assert_eq!(
        part1(
            &"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX",
        ),
        2210736
    );
}

#[test]
fn big_part2() {
    assert_eq!(
        part2(
            &"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX",
        ),
        460664
    );
}
