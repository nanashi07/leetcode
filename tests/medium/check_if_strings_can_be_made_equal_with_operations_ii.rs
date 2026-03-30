// 2840. Check if Strings Can be Made Equal With Operations II
// https://leetcode.com/problems/check-if-strings-can-be-made-equal-with-operations-ii/

struct Solution;

impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::check_if_strings_can_be_made_equal_with_operations_ii::Solution;

    #[test]
    fn test_check_strings_1() {
        let s1 = "abcdba".to_string();
        let s2 = "cabdab".to_string();
        assert_eq!(true, Solution::check_strings(s1, s2));
    }

    #[test]
    fn test_check_strings_2() {
        let s1 = "abe".to_string();
        let s2 = "bea".to_string();
        assert_eq!(false, Solution::check_strings(s1, s2));
    }
}
