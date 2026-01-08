use itertools::Itertools;
use std::ops::Sub;
use std::cmp::Ordering;

fn main() {}

fn is_that(nums: impl IntoIterator<Item: Copy + PartialOrd + Sub<Output: PartialEq>>) -> bool {
    nums.into_iter()
        .sorted_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .tuple_windows()
        .map(|(a, b)| b - a)
        .all_equal()
}

#[test]
fn test1() {
    let a = vec![7, 5, 9, 3, 1];
    assert!(is_that(a));
    let b = vec![1, 3, 5, 7, 8];
    assert!(!is_that(b));
}
