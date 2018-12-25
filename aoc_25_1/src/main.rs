fn main() {
    let mut graph: Vec<Node> = include_str!("input.txt")
        .lines()
        .fold(vec![], |mut out, line| {
            let nums: Vec<_> = line
                .split(',')
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            out.push(Node {
                pos: Vec4::new(nums[0], nums[1], nums[2], nums[3]),
                connected: vec![],
                removed: false,
            });
            out
        });

    for i in 0..graph.len() {
        for j in i..graph.len() {
            if graph[i].pos.distance(&graph[j].pos) <= 3 && i != j {
                graph[i].connected.push(j);
                graph[j].connected.push(i);
            }
        }
    }

    let mut result = 0;
    while !graph_empty(&graph) {
        result += 1;
        let mut to_remove: Vec<usize> = vec![graph.iter().enumerate().fold(0, |mut out, node| {
            if !node.1.removed {
                out = node.0;
            }
            out
        })];

        while !to_remove.is_empty() {
            let n = to_remove.pop().unwrap();
            graph[n].removed = true;
            for o in graph[n].connected.clone() {
                if !graph[o].removed {
                    to_remove.push(o);
                }
            }
        }
    }

    println!("{}", result);
}

fn graph_empty(graph: &Vec<Node>) -> bool {
    graph.iter().fold(true, |mut out, node| {
        if !node.removed {
            out = false;
        }
        out
    })
}

#[derive(Debug, Clone)]
struct Node {
    pos: Vec4,
    connected: Vec<usize>,
    removed: bool,
}

#[derive(Debug, Clone)]
struct Vec4 {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Vec4 {
    fn distance(&self, other: &Vec4) -> i32 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.w - other.w).abs()
    }

    fn new(x: i32, y: i32, z: i32, w: i32) -> Vec4 {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}
