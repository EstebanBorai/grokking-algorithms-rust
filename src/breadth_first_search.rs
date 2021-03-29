use std::collections::HashMap;

use crate::queue::Queue;

type Graph = HashMap<String, Vec<String>>;

#[allow(dead_code)]
fn breadth_first_search<F>(graph: Graph, name: String, pred: F) -> Option<String>
where
    F: Fn(String) -> bool,
{
    let mut queue: Queue<String> = Queue::new();
    let mut searched: Vec<String> = Vec::new();
    queue.enqueue_many(&graph[&name]);

    while !queue.is_empty() {
        let current = queue.dequeue();

        if searched.iter().find(|e| e == &&current).is_none() {
            if pred(current.clone()) {
                return Some(current);
            }

            queue.enqueue_many(&graph[&current]);
            searched.push(current);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_graph() -> Graph {
        let mut graph: Graph = HashMap::new();

        graph.insert(
            String::from("you"),
            vec!["alice".to_string(), "bob".to_string(), "claire".to_string()],
        );
        graph.insert(
            String::from("bob"),
            vec!["anuj".to_string(), "peggy".to_string()],
        );
        graph.insert(String::from("alice"), vec!["peggy".to_string()]);
        graph.insert(
            String::from("claire"),
            vec!["thom".to_string(), "jonny".to_string()],
        );
        graph.insert(String::from("anuj"), Vec::new());
        graph.insert(String::from("peggy"), Vec::new());
        graph.insert(String::from("thom"), Vec::new());
        graph.insert(String::from("jonny"), Vec::new());

        graph
    }

    #[test]
    fn finds_peggy_in_the_graph() {
        let graph = make_graph();
        let predicate = |n: String| n == String::from("peggy");
        let result = breadth_first_search(graph, String::from("you"), predicate);

        assert!(result.is_some());
        assert_eq!(Some(String::from("peggy")), result);
    }

    #[test]
    fn doesnt_finds_pickle_in_the_graph() {
        let graph = make_graph();
        let predicate = |n: String| n == String::from("pickle");
        let result = breadth_first_search(graph, String::from("you"), predicate);

        assert!(result.is_none());
    }
}
