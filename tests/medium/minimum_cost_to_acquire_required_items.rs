// 3789. Minimum Cost to Acquire Required Items
// https://leetcode.com/problems/minimum-cost-to-acquire-required-items/

struct Solution;

impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_cost_to_acquire_required_items::Solution;

    #[test]
    fn test_minimum_cost_1() {
        let cost1: i32 = 3;
        let cost2: i32 = 2;
        let cost_both: i32 = 1;
        let need1: i32 = 3;
        let need2: i32 = 2;
        assert_eq!(
            3,
            Solution::minimum_cost(cost1, cost2, cost_both, need1, need2)
        );
    }

    #[test]
    fn test_minimum_cost_2() {
        let cost1: i32 = 5;
        let cost2: i32 = 4;
        let cost_both: i32 = 15;
        let need1: i32 = 2;
        let need2: i32 = 3;
        assert_eq!(
            22,
            Solution::minimum_cost(cost1, cost2, cost_both, need1, need2)
        );
    }

    #[test]
    fn test_minimum_cost_3() {
        let cost1: i32 = 5;
        let cost2: i32 = 4;
        let cost_both: i32 = 15;
        let need1: i32 = 0;
        let need2: i32 = 0;
        assert_eq!(
            0,
            Solution::minimum_cost(cost1, cost2, cost_both, need1, need2)
        );
    }
}
