// 3418. Maximum Amount of Money Robot Can Earn
// https://leetcode.com/problems/maximum-amount-of-money-robot-can-earn/

struct Solution;

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_amount_of_money_robot_can_earn::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_maximum_amount_1() {
        let coins = to_vec2d([[0, 1, -1], [1, -2, 3], [2, -3, 4]]);
        assert_eq!(8, Solution::maximum_amount(coins));
    }

    #[test]
    fn test_maximum_amount_2() {
        let coins = to_vec2d([[10, 10, 10], [10, 10, 10]]);
        assert_eq!(40, Solution::maximum_amount(coins));
    }
}
