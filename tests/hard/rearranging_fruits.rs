// # 2561. Rearranging Fruits
// https://leetcode.com/problems/rearranging-fruits/

struct Solution;

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::rearranging_fruits::Solution;

    #[test]
    fn test_min_cost_1() {
        let basket1 = [4, 2, 2, 2].to_vec();
        let basket2 = [1, 4, 1, 2].to_vec();
        assert_eq!(1, Solution::min_cost(basket1, basket2));
    }

    #[test]
    fn test_min_cost_2() {
        let basket1 = [2, 3, 4, 1].to_vec();
        let basket2 = [3, 2, 5, 1].to_vec();
        assert_eq!(1, Solution::min_cost(basket1, basket2));
    }
}
