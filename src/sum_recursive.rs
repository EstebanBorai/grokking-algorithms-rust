fn sum_recursive(coll: &Vec<u32>) -> u32 {
    if coll.len() == 0 {
        return 0;
    }

    let mut coll_clone = coll.clone();
    let last = coll_clone.pop().unwrap();

    return last + sum_recursive(&coll_clone);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_all_elements_in_vec() {
        let sum_digits = vec![2, 4, 6];

        assert_eq!(sum_recursive(&sum_digits), 12);
    }

    #[test]
    fn returns_zero_if_empty_vec_is_provided() {
        let sum_digits = vec![];

        assert_eq!(sum_recursive(&sum_digits), 0);
    }
}
