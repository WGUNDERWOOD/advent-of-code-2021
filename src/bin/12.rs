use std::collections::HashMap;

#[derive(Clone, PartialEq)]
enum Size {
    Big,
    Small,
}

#[derive(Clone, PartialEq)]
enum Position {
    Start,
    End,
    Middle,
}

#[derive(Clone, PartialEq)]
struct Node {
    id: u32,
    size: Size,
    position: Position,
}

fn get_node(node: &str, id: u32) -> Node {
    let size = if node.chars().next().unwrap().is_uppercase() {
        Size::Big
    } else {
        Size::Small
    };
    let position = if node == "start" {
        Position::Start
    } else if node == "end" {
        Position::End
    } else {
        Position::Middle
    };
    Node{id, size, position}
}

fn parse_input(input: &str) -> (Vec<Node>, Vec<(Node, Node)>) {
    let lines: Vec<&str> = input.split('\n').filter(|x| !x.is_empty()).collect();
    let mut id = 0;
    let mut node_lookup: HashMap<&str, Node> = HashMap::new();
    let mut nodes: Vec<Node> = vec![];
    let mut edges: Vec<(Node, Node)> = vec![];
    for line in lines {
        let line_split: Vec<&str> = line.split('-').collect();
        let source = line_split[0];
        let dest = line_split[1];
        if !node_lookup.contains_key(source) {
            let node = get_node(source, id);
            node_lookup.insert(source, node.clone());
            nodes.push(node);
            id += 1;
        }
        if !node_lookup.contains_key(dest) {
            let node = get_node(dest, id);
            node_lookup.insert(dest, node.clone());
            nodes.push(node);
            id += 1;
        }
        edges.push((node_lookup[source].clone(), node_lookup[dest].clone()));
        edges.push((node_lookup[dest].clone(), node_lookup[source].clone()));
    }
    (nodes, edges)
}

fn find_start_node(nodes: &Vec<Node>) -> Option<Node> {
    for node in nodes {
        if node.position == Position::Start {
            return Some(node.clone())
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let (nodes, edges) = parse_input(&input);
    let mut paths_queue: Vec<Vec<Node>> = vec![vec![find_start_node(&nodes).unwrap()]];
    let mut paths: Vec<Vec<Node>> = vec![];
    while paths_queue.len() > 0 {
        let path = paths_queue.pop().unwrap();
        let node = &path[path.len()-1];
        if node.position == Position::End {
            paths.push(path)
        } else {
            for edge in &edges {
                if edge.0 == *node {
                    if edge.1.size == Size::Big || !path.contains(&edge.1) {
                        let mut new_path = path.clone();
                        new_path.push(edge.1.clone());
                        paths_queue.push(new_path);
                    }
                }
            }
        }
    }
    Some(paths.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (nodes, edges) = parse_input(&input);
    let mut paths_queue: Vec<(Vec<Node>, bool)> = vec![(vec![find_start_node(&nodes).unwrap()], false)];
    let mut paths: Vec<Vec<Node>> = vec![];
    while paths_queue.len() > 0 {
        let (path, twice) = paths_queue.pop().unwrap();
        let node = &path[path.len()-1];
        if node.position == Position::End {
            paths.push(path)
        } else {
            for edge in &edges {
                if edge.0 == *node {
                    if edge.1.size == Size::Big ||
                        !path.contains(&edge.1) ||
                        edge.1.position == Position::Middle && !twice {
                            let mut new_path = path.clone();
                            new_path.push(edge.1.clone());
                            if edge.1.size == Size::Small && path.contains(&edge.1) {
                                paths_queue.push((new_path, true));
                            } else {
                                paths_queue.push((new_path, twice));
                            }
                    }
                }
            }
        }
    }
    Some(paths.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(226));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(3509));
    }
}
