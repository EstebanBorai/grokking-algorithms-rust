use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

type WeightedGraph<'a> = HashMap<Node<'a>, Vec<(Node<'a>, usize)>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Node<'a> {
    name: &'a str,
}

impl<'a> Node<'a> {
    fn new(name: &'a str) -> Node<'a> {
        Node { name }
    }
}

#[derive(Debug)]
struct Visit<N> {
    node: N,
    distance: usize,
}

impl<N> Ord for Visit<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<N> PartialOrd for Visit<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<N> PartialEq for Visit<N> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<N> Eq for Visit<N> {}

#[allow(dead_code)]
fn dijkstra<'a>(
    start: Node<'a>,
    graph: &WeightedGraph<'a>,
) -> HashMap<Node<'a>, usize> {
    let mut distances: HashMap<Node, usize> = HashMap::new();
    let mut visited: HashSet<Node> = HashSet::new();
    let mut to_visit: BinaryHeap<Visit<Node>> = BinaryHeap::new();

    distances.insert(start, 0);
    to_visit.push(Visit {
        node: start,
        distance: 0,
    });

    while let Some(Visit { node, distance }) = to_visit.pop() {
        if !visited.insert(node) {
            continue;
        }

        if let Some(neighbors) = graph.get(&node) {
            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(&neighbor)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    distances.insert(*neighbor, new_distance);
                    to_visit.push(Visit {
                        node: *neighbor,
                        distance: new_distance,
                    });
                }
            }
        }
    }

    distances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dijkstra_finds_shortest_distance() {
        let start_node = Node::new("start");
        let a_node = Node::new("a");
        let b_node = Node::new("b");
        let fin_node = Node::new("fin");

        let mut graph: WeightedGraph = HashMap::new();

        graph.insert(start_node, vec![(a_node, 6), (b_node, 2)]);
        graph.insert(a_node, vec![(fin_node, 1)]);
        graph.insert(b_node, vec![(a_node, 3), (fin_node, 5)]);
        graph.insert(fin_node, vec![]);

        let distances = dijkstra(start_node, &graph);

        assert_eq!(distances.get(&fin_node), Some(&6));
    }
}
