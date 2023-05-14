use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::BinaryHeap;

fn parse_cavern(input: &str) -> Vec<Vec<i32>> {
    let cavern_rows = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();
    let n = cavern_rows.len();
    let mut cavern = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            cavern[i][j] = cavern_rows[i]
                .chars()
                .nth(j)
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap()
        }
    }
    return cavern;
}

fn get_enlarged_cavern(cavern: &Vec<Vec<i32>>, factor: i32) -> Vec<Vec<i32>> {

    let n = cavern.len();
    let enlarged_n = factor as usize * n;
    let mut enlarged_cavern: Vec<Vec<i32>> = vec![vec![0; enlarged_n]; enlarged_n];

    for i in 0..enlarged_n {
        for j in 0..enlarged_n {
            let increment = (i / n + j / n) as i32;
            enlarged_cavern[i][j] = (cavern[i % n][j % n] + increment - 1) % 9 + 1;
        }
    }
    return enlarged_cavern
}

fn get_neighbors(current: (i32, i32), n: i32) -> HashMap<(i32, i32), bool> {
    let mut neighbors: HashMap<(i32, i32), bool> = HashMap::new();
    for i in [current.0 - 1, current.0 + 1] {
        if (0 <= i) && (i <= n - 1) {
            neighbors.insert((i, current.1), true);
        }
    }
    for j in [current.1 - 1, current.1 + 1] {
        if (0 <= j) && (j <= n - 1) {
            neighbors.insert((current.0, j), true);
        }
    }
    return neighbors;
}

#[derive(Copy, Clone)]
struct Edge {
    node: i32,
    risk: i32,
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct State {
    node: i32,
    dist: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

}

fn get_nodes_edges(cavern: Vec<Vec<i32>>) -> (Vec<i32>, Vec<Vec<Edge>>) {

    let n = cavern.len() as i32;

    let nodes: Vec<i32> = (0..n*n).map(|x| x as i32).collect();
    let mut edges: Vec<Vec<Edge>> = vec![];

    for i in 0..n {
        for j in 0..n {
        let mut node_edges: Vec<Edge> = vec![];
            let neighbors = get_neighbors((i as i32, j as i32), n as i32);
            for position in neighbors.keys() {
                let node = n * position.0 + position.1;
                let risk = cavern[position.0 as usize][position.1 as usize];
                let edge = Edge{ node, risk };
                node_edges.push(edge)
            }
            edges.push(node_edges);
        }
    }

    return (nodes, edges)
}


fn get_best_risk(nodes: &Vec<i32>, edges: &Vec<Vec<Edge>>) -> i32 {

    let n = nodes.len() as i32;
    let mut checking: BinaryHeap<State> = BinaryHeap::new();
    checking.push(State{node: 0, dist: 0});

    let mut distances: Vec<i32> = nodes.iter().map(|_| i32::MAX).collect();
    distances[0] = 0;

    while let Some(State{node, dist}) = checking.pop() {

        if node == n - 1 { return dist }

        for edge in &edges[node as usize] {
            let next = State{node: edge.node, dist: edge.risk + dist};

            if next.dist < distances[next.node as usize] {
                checking.push(next);
                distances[next.node as usize] = next.dist;
            }
        }
    }

    return distances[(n-1) as usize];
}

pub fn part_one(input: &str) -> Option<u32> {
    let cavern = parse_cavern(input);
    let (nodes, edges) = get_nodes_edges(cavern);
    let best_risk = get_best_risk(&nodes, &edges);
    return Some(best_risk as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    let cavern = parse_cavern(input);
    let enlarged_cavern = get_enlarged_cavern(&cavern, 5);
    let (nodes, edges) = get_nodes_edges(enlarged_cavern);
    let best_risk = get_best_risk(&nodes, &edges);
    return Some(best_risk as u32);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(40));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(315));
    }
}
