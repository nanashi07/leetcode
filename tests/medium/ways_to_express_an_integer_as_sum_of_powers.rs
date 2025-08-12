// # 2787. Ways to Express an Integer as Sum of Powers
// https://leetcode.com/problems/ways-to-express-an-integer-as-sum-of-powers/

struct Solution;

impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::ways_to_express_an_integer_as_sum_of_powers::Solution;

    #[test]
    fn test_number_of_ways_1() {
        let n = 10;
        let x = 2;
        assert_eq!(1, Solution::number_of_ways(n, x));
    }

    #[test]
    fn test_number_of_ways_2() {
        let n = 4;
        let x = 1;
        assert_eq!(2, Solution::number_of_ways(n, x));
    }
}
