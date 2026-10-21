// 3863. Minimum Operations to Sort a String
// https://leetcode.com/problems/minimum-operations-to-sort-a-string/

struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_operations_to_sort_a_string::Solution;

    #[test]
    fn test_min_operations_1() {
        let s = "dog".to_string();
        assert_eq!(1, Solution::min_operations(s));
    }

    #[test]
    fn test_min_operations_2() {
        let s = "card".to_string();
        assert_eq!(2, Solution::min_operations(s));
    }

    #[test]
    fn test_min_operations_3() {
        let s = "gf".to_string();
        assert_eq!(-1, Solution::min_operations(s));
    }
}
