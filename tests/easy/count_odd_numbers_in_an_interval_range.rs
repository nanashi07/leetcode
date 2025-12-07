// 1523. Count Odd Numbers in an Interval Range
// https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/

struct Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::count_odd_numbers_in_an_interval_range::Solution;

    #[test]
    fn test_count_odds_1() {
        let low = 3;
        let high = 7;
        assert_eq!(3, Solution::count_odds(low, high));
    }

    #[test]
    fn test_count_odds_2() {
        let low = 8;
        let high = 10;
        assert_eq!(1, Solution::count_odds(low, high));
    }
}
