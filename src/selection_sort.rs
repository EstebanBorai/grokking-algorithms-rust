fn find_smallest<T>(collection: &Vec<T>) -> usize
where
    T: std::cmp::Ord + std::fmt::Display,
{
    let mut smallest_index: usize = 0;
    let mut smallest_value: &T = &collection[smallest_index];

    for (idx, el) in collection.iter().enumerate() {
        if el < smallest_value {
            smallest_index = idx;
            smallest_value = el;
        }
    }

    smallest_index
}

#[allow(dead_code)]
pub fn selection_sort<T>(collection: &Vec<T>) -> Vec<T>
where
    T: std::cmp::Ord + std::fmt::Display + std::clone::Clone,
{
    let mut result: Vec<T> = Vec::new();
    let mut copy = collection.clone();

    for _ in 0..collection.len() {
        let smallest = find_smallest(&copy);

        result.push(copy.remove(smallest));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_a_collection() {
        let items = vec![8, 2, 4, 6, 5, 7, 10];
        let result = selection_sort(&items);

        assert_eq!(result, vec![2, 4, 5, 6, 7, 8, 10]);
    }

    #[test]
    fn finds_smallest() {
        let items = vec![3, 5, 7, -1, 8, 40, 1];
        let result = find_smallest(&items);

        assert_eq!(result, 3);
    }
}
