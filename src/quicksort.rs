#[allow(dead_code)]
fn quicksort<T>(coll: &Vec<T>) -> Vec<T>
where
    T: std::cmp::PartialOrd + Sized + Copy + Clone,
{
    // To keep return types consistent we preallocate
    // the clone for the provided `Vec<T>` in order
    // to retrieve it.
    //
    // This strategy has a performance penalty but
    // helps avoid performing algorithms _in situ_
    let coll_clone: Vec<T> = coll.clone();

    if coll.len() < 2 {
        // If theres no elements or theres only
        // one element we retrieve the `Vec<T>`
        // provided.
        // In terms of D&C this is the base case
        // which also breaks recursiveness
        return coll_clone;
    }

    // This is the recursive case where we plan
    // to minimize the size of the problem thus
    // applying a D&C mindset
    let pivot = coll_clone.first().unwrap();

    let less_than_pivot: Vec<T> = quicksort(&coll.iter().filter(|i| *i < pivot).cloned().collect());
    let mut greater_than_pivot: Vec<T> =
        quicksort(&coll.iter().filter(|i| *i > pivot).cloned().collect());

    let mut result = less_than_pivot;

    result.push(*pivot);
    result.append(&mut greater_than_pivot);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_a_collection() {
        let items = vec![8, 2, 4, 6, 5, 7, 10];
        let result = quicksort(&items);

        assert_eq!(result, vec![2, 4, 5, 6, 7, 8, 10]);
    }
}
