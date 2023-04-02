fn binary_search(slice: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = slice.len();

    while left < right {
        let middle = left + (right - left) / 2;

        if slice[middle] == target {
            return Some(middle);
        } else if slice[middle] > target {
            right = middle;
        } else {
            left = middle + 1;
        }
    }

    None
}

fn main() {
    let sorted_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let target = 5;
    let result = binary_search(&sorted_data, target).unwrap();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_slice() {
        let data = vec![];
        let target = 5;
        assert_eq!(binary_search(&data, target), None);
    }

    #[test]
    fn test_single_element() {
        let data = vec![5];
        let target = 5;
        assert_eq!(binary_search(&data, target), Some(0));
    }

    #[test]
    fn test_target_less_than_min() {
        let data = vec![5, 10, 15];
        let target = 1;
        assert_eq!(binary_search(&data, target), None);
    }

    #[test]
    fn test_target_greater_than_max() {
        let data = vec![5, 10, 15];
        let target = 20;
        assert_eq!(binary_search(&data, target), None);
    }

    #[test]
    fn test_all_elements_are_same() {
        let data = vec![5, 5, 5, 5, 5];
        let target = 5;
        // Due to our implementation it will find the middle one
        assert_eq!(binary_search(&data, target), Some(2));
    }

    #[test]
    fn test_target_in_middle() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target = 5;
        assert_eq!(binary_search(&data, target), Some(4));
    }

    #[test]
    fn test_target_at_start() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target = 1;
        assert_eq!(binary_search(&data, target), Some(0));
    }

    #[test]
    fn test_target_at_end() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target = 9;
        assert_eq!(binary_search(&data, target), Some(8));
    }

    #[test]
    fn test_target_in_left_half() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target = 3;
        assert_eq!(binary_search(&data, target), Some(2));
    }

    #[test]
    fn test_target_in_right_half() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target = 7;
        assert_eq!(binary_search(&data, target), Some(6));
    }
}
