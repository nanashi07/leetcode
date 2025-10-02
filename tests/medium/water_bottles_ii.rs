// # 3100. Water Bottles II
// https://leetcode.com/problems/water-bottles-ii/

struct Solution;

impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::water_bottles_ii::Solution;

    #[test]
    fn test_max_bottles_drunk_1() {
        let num_bottles = 13;
        let num_exchange = 6;
        assert_eq!(15, Solution::max_bottles_drunk(num_bottles, num_exchange));
    }

    #[test]
    fn test_max_bottles_drunk_2() {
        let num_bottles = 10;
        let num_exchange = 3;
        assert_eq!(13, Solution::max_bottles_drunk(num_bottles, num_exchange));
    }
}
