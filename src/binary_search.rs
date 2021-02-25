use std::cmp::Ordering;

fn binary_search<T>(items: Vec<T>, value: T) -> Option<usize>
where
    T: std::cmp::Ord,
{
    if items.is_empty() {
        // Avoid running if no items are provided
        return None;
    }

    let (mut start, mut end) = (0, items.len() - 1);

    loop {
        let idx = (start + end) / 2;
        let attempt = items.get(idx).unwrap();

        if start > end {
            break;
        }

        match attempt.cmp(&value) {
            Ordering::Greater => {
                end = idx - 1;
            }
            Ordering::Less => {
                start = idx + 1;
            }
            Ordering::Equal => {
                return Some(idx);
            }
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
