use std::cmp::Ordering;

#[warn(dead_code)]
fn binary_search<T>(list: Vec<T>, item: T) -> Option<usize>
where
    T: std::cmp::Ord,
{
    if list.is_empty() {
        return None;
    };
    let (mut low, mut high) = (0, list.len() - 1);

    while low < high {
        let mid = (low + high) / 2;

        match list[mid].cmp(&item) {
            Ordering::Greater => high = mid,
            Ordering::Less => low = mid + 1,
            Ordering::Equal => return Some(mid),
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_none_if_items_is_empty() {
        let items = vec![];
        let result = binary_search(items, 1);

        assert_eq!(result, None);
    }

    #[test]
    fn doesnt_finds_if_collection_is_not_sorted() {
        let items = vec![1024, 32, 16, 512, 256, 64, 128];
        let result = binary_search(items, 1024);

        assert_eq!(result, None);
    }

    #[test]
    fn finds_index_of_element() {
        let items = vec![16, 32, 64, 128, 256, 512, 1024];
        let result = binary_search(items, 256);

        assert_eq!(result, Some(4));
    }
}
