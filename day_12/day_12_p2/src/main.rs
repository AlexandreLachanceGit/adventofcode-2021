use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

struct Node {
    near: Vec<String>,
    is_small: bool,
}

struct Graph {
    nodes: HashMap<String, Node>,
}

impl Graph {
    fn add_edge(&mut self, a: String, b: String) {
        self.add_directed_edge(a.clone(), b.clone());
        self.add_directed_edge(b, a);
    }
    fn add_directed_edge(&mut self, a: String, b: String) {
        if self.nodes.contains_key(&a) {
            self.nodes.get_mut(&a).unwrap().near.push(b);
        } else {
            self.nodes.insert(
                a.clone(),
                Node {
                    near: vec![b],
                    is_small: a.chars().nth(0).unwrap().is_lowercase(),
                },
            );
        }
    }
}

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let iter = BufReader::new(file).lines();

    let mut paths: i32 = 0;
    let mut graph = Graph {
        nodes: HashMap::new(),
    };
    for (_index, line) in iter.enumerate() {
        let line = line.unwrap();
        let mut line_iter = line.split("-");
        let first = line_iter.next().unwrap();
        let second = line_iter.next().unwrap();
        graph.add_edge(first.into(), second.into());
    }

    find_paths(&mut graph, "start".into(), &mut paths, Vec::new());

    println!("{}", paths);
}

fn find_paths(graph: &mut Graph, current: String, count: &mut i32, mut visited: Vec<String>) {
    if current == "end" {
        *count += 1;
        return;
    }

    if graph.nodes[&current].is_small {
        visited.push(current.clone());
    }

    for n in graph.nodes.get_mut(&current).unwrap().near.clone() {
        if !graph.nodes[&n].is_small
            || !visited.contains(&n)
            || (can_visit(&visited) && n != "start")
        {
            find_paths(graph, n, count, visited.clone());
        }
    }
}

fn can_visit(vec: &Vec<String>) -> bool {
    for i in vec {
        if count(vec, &i) == 2 {
            return false;
        }
    }

    true
}

fn count(vec: &Vec<String>, e: &String) -> i32 {
    let mut c = 0;
    for i in vec {
        if *i == *e {
            c += 1;
        }
    }
    c
}
