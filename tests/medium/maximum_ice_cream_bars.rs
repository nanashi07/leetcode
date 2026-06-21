// 1833. Maximum Ice Cream Bars
// https://leetcode.com/problems/maximum-ice-cream-bars/

struct Solution;

impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_ice_cream_bars::Solution;

    #[test]
    fn test_max_ice_cream_1() {
        let costs = [1, 3, 2, 4, 1].to_vec();
        let coins = 7;
        assert_eq!(4, Solution::max_ice_cream(costs, coins));
    }

    #[test]
    fn test_max_ice_cream_2() {
        let costs = [10, 6, 8, 7, 7, 8].to_vec();
        let coins = 5;
        assert_eq!(0, Solution::max_ice_cream(costs, coins));
    }

    #[test]
    fn test_max_ice_cream_3() {
        let costs = [1, 6, 3, 1, 2, 5].to_vec();
        let coins = 20;
        assert_eq!(6, Solution::max_ice_cream(costs, coins));
    }
}
