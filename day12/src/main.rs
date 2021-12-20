use anyhow::{anyhow, bail, Result};
use clap::Parser;
use itertools::Itertools;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Parser)]
#[clap(version = "1.0", author = "Raniz")]
struct Opts {
    #[clap(short, long, default_value = "input")]
    input: String,
    #[clap(short, long)]
    twice: bool,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut input = String::new();
    File::open(opts.input)?.read_to_string(&mut input)?;
    let lines = input
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();

    let graph = Graph::parse(&lines)?;
    let paths = graph.get_paths(opts.twice);
    println!("There are {} paths", paths.len());
    Ok(())
}

#[derive(Eq, PartialEq, Debug)]
struct Graph {
    start: Rc<RefCell<Node>>,
}

#[derive(Eq, Debug)]
struct Node {
    name: String,
    vertices: Vec<Rc<RefCell<Node>>>,
    big: bool,
}

impl Graph {
    fn parse(lines: &[&str]) -> Result<Graph> {
        let mut node_map = HashMap::new();
        for line in lines {
            let (a_name, b_name) = line
                .split("-")
                .collect_tuple()
                .ok_or_else(|| anyhow!("Invalid line {}", line))?;
            if !node_map.contains_key(a_name) {
                node_map.insert(a_name, Rc::new(RefCell::new(Node::new(a_name))));
            }
            if !node_map.contains_key(b_name) {
                node_map.insert(b_name, Rc::new(RefCell::new(Node::new(b_name))));
            }
            let a = &node_map[a_name];
            let b = &node_map[b_name];
            Node::connect(a, b);
        }
        let start = node_map
            .remove("start")
            .ok_or_else(|| anyhow!("No starting node"))?;
        Ok(Graph { start })
    }

    fn get_paths(&self, allow_once_twice: bool) -> HashSet<Vec<String>> {
        let mut paths = HashSet::new();
        let current_path = Vec::new();
        self.find_end(&self.start, &mut paths, allow_once_twice, current_path);
        paths
    }

    fn find_end(
        &self,
        node: &Rc<RefCell<Node>>,
        paths: &mut HashSet<Vec<String>>,
        allow_once_twice: bool,
        current_path: Vec<String>,
    ) {
        println!("{}: {:?}", allow_once_twice, current_path);
        let name = node.deref().borrow().name.clone();
        if name == "end" {
            let mut path = current_path.clone();
            path.push("end".to_string());
            paths.insert(path);
            return;
        }
        let mut current_path = current_path.clone();
        current_path.push(name.clone());
        for neighbour in &node.deref().borrow().vertices {
            let neighbour_name = neighbour.deref().borrow().name.clone();
            if neighbour_name == "start" {
                continue;
            } else if neighbour.deref().borrow().big || !current_path.contains(&neighbour_name) {
                self.find_end(neighbour, paths, allow_once_twice, current_path.clone())
            } else if allow_once_twice {
                self.find_end(neighbour, paths, false, current_path.clone())
            }
        }
    }
}

impl Node {
    fn new(name: impl Into<String>) -> Node {
        let name = name.into();
        let big = name.chars().next().unwrap().is_uppercase();
        Node {
            name,
            vertices: Vec::new(),
            big,
        }
    }

    fn connect(a: &Rc<RefCell<Node>>, b: &Rc<RefCell<Node>>) {
        a.deref().borrow_mut().vertices.push(b.clone());
        b.deref().borrow_mut().vertices.push(a.clone());
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

#[cfg(test)]
mod test {
    use crate::{Graph, Node};
    use maplit::hashset;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn small_graph() -> Graph {
        let start = Rc::new(RefCell::new(Node::new("start")));
        let a = Rc::new(RefCell::new(Node::new("A")));
        let b = Rc::new(RefCell::new(Node::new("b")));
        let c = Rc::new(RefCell::new(Node::new("c")));
        let d = Rc::new(RefCell::new(Node::new("d")));
        let end = Rc::new(RefCell::new(Node::new("end")));
        Node::connect(&start, &a);
        Node::connect(&start, &b);
        Node::connect(&a, &b);
        Node::connect(&a, &c);
        Node::connect(&a, &end);
        Node::connect(&b, &d);
        Node::connect(&b, &end);
        Graph { start }
    }

    #[test]
    fn test_parse() {
        let lines = ["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];
        let graph = Graph::parse(&lines);
        assert!(graph.is_ok());
        assert_eq!(small_graph(), graph.unwrap());
    }

    #[test]
    fn test_traverse() {
        let graph = small_graph();
        let expected = hashset![
            vec!["start", "A", "b", "A", "c", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "b", "A", "end"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "b", "end"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "c", "A", "b", "A", "end"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "c", "A", "b", "end"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "c", "A", "end"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "end"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "b", "A", "c", "A", "end"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "b", "A", "end"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "b", "end"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
        ];

        assert_eq!(expected, graph.get_paths(false));
    }

    #[test]
    fn test_traverse_twice() {
        let graph = small_graph();
        let expected = hashset![
            vec!["start", "A", "b", "A", "b", "A", "c", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "b", "A", "b", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "b", "A", "b", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "b", "A", "c", "A", "b", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "b", "A", "c", "A", "b", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "b", "A", "c", "A", "c", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "b", "A", "c", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "b", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "b", "d", "b", "A", "c", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "b", "d", "b", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "b", "d", "b", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "b", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "c", "A", "b", "A", "b", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "c", "A", "b", "A", "b", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "c", "A", "b", "A", "c", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "c", "A", "b", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "c", "A", "b", "d", "b", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "c", "A", "b", "d", "b", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "c", "A", "b", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "c", "A", "c", "A", "b", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "c", "A", "c", "A", "b", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "c", "A", "c", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "c", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "b", "A", "b", "A", "c", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "b", "A", "b", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "b", "A", "b", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "b", "A", "c", "A", "b", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "b", "A", "c", "A", "b", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "b", "A", "c", "A", "c", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "b", "A", "c", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "b", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "b", "d", "b", "A", "c", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "b", "d", "b", "A", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "b", "d", "b", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            vec!["start", "b", "end",]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
        ];

        assert_eq!(expected, graph.get_paths(true));
    }
}
