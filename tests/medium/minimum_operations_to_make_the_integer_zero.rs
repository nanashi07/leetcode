// # 2749. Minimum Operations to Make the Integer Zero
// https://leetcode.com/problems/minimum-operations-to-make-the-integer-zero/

struct Solution;

impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_operations_to_make_the_integer_zero::Solution;

    #[test]
    fn test_make_the_integer_zero_1() {
        let num1 = 3;
        let num2 = -2;
        assert_eq!(3, Solution::make_the_integer_zero(num1, num2));
    }

    #[test]
    fn test_make_the_integer_zero_2() {
        let num1 = 5;
        let num2 = 7;
        assert_eq!(-1, Solution::make_the_integer_zero(num1, num2));
    }
}
