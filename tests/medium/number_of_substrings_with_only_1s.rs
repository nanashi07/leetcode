// 1513. Number of Substrings With Only 1s
// https://leetcode.com/problems/number-of-substrings-with-only-1s/

struct Solution;

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_substrings_with_only_1s::Solution;

    #[test]
    fn test_num_sub_1() {
        let s = "0110111".to_string();
        assert_eq!(9, Solution::num_sub(s));
    }

    #[test]
    fn test_num_sub_2() {
        let s = "101".to_string();
        assert_eq!(2, Solution::num_sub(s));
    }

    #[test]
    fn test_num_sub_3() {
        let s = "111111".to_string();
        assert_eq!(21, Solution::num_sub(s));
    }
}
