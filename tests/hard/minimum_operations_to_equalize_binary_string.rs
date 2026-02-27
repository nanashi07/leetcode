// 3666. Minimum Operations to Equalize Binary String
// https://leetcode.com/problems/minimum-operations-to-equalize-binary-string/

struct Solution;

impl Solution {
    pub fn min_operations(s: String, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_operations_to_equalize_binary_string::Solution;

    #[test]
    fn test_min_operations_1() {
        let s = "110".to_string();
        let k = 1;
        assert_eq!(1, Solution::min_operations(s, k));
    }

    #[test]
    fn test_min_operations_2() {
        let s = "0101".to_string();
        let k = 3;
        assert_eq!(2, Solution::min_operations(s, k));
    }

    #[test]
    fn test_min_operations_3() {
        let s = "101".to_string();
        let k = 2;
        assert_eq!(-1, Solution::min_operations(s, k));
    }
}
