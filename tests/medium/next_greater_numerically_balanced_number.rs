// # 2048. Next Greater Numerically Balanced Number
// https://leetcode.com/problems/next-greater-numerically-balanced-number/

struct Solution;

impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::next_greater_numerically_balanced_number::Solution;

    #[test]
    fn test_next_beautiful_number_1() {
        let n = 1;
        assert_eq!(22, Solution::next_beautiful_number(n));
    }

    #[test]
    fn test_next_beautiful_number_2() {
        let n = 1000;
        assert_eq!(1333, Solution::next_beautiful_number(n));
    }

    #[test]
    fn test_next_beautiful_number_3() {
        let n = 3000;
        assert_eq!(3133, Solution::next_beautiful_number(n));
    }
}
