// 3228. Maximum Number of Operations to Move Ones to the End
// https://leetcode.com/problems/maximum-number-of-operations-to-move-ones-to-the-end/

struct Solution;

impl Solution {
    pub fn max_operations(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_number_of_operations_to_move_ones_to_the_end::Solution;

    #[test]
    fn test_max_operations_1() {
        let s = "1001101".to_string();
        assert_eq!(4, Solution::max_operations(s));
    }

    #[test]
    fn test_max_operations_2() {
        let s = "00111".to_string();
        assert_eq!(0, Solution::max_operations(s));
    }
}
