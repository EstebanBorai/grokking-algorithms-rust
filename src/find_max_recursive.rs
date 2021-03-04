#[allow(dead_code)]
fn find_max_recursive(coll: &Vec<i32>) -> i32 {
    if coll.len() == 0 {
        return 0;
    }

    let mut coll_clone = coll.clone();

    // A find_max recursive function is defined internally to
    // provide the state capabilities and protect the behavior
    // of the function
    fn find_max(max_num: i32, coll: &mut Vec<i32>) -> i32 {
        if coll.len() == 0 {
            return max_num;
        }

        let last = coll.pop().unwrap();

        if last > max_num {
            find_max(last, coll)
        } else {
            find_max(max_num, coll)
        }
    }

    find_max(*coll.first().unwrap(), &mut coll_clone)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_greatest_value() {
        let items = vec![8, 12, 128, 1, 90, -1, 32];

        assert_eq!(find_max_recursive(&items), 128);
    }

    #[test]
    fn returns_zero_if_empty_vec_is_provided() {
        let items = vec![];

        assert_eq!(find_max_recursive(&items), 0);
    }
}
