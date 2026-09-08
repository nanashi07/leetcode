// 3870. Count Commas in Range
// https://leetcode.com/problems/count-commas-in-range/

struct Solution;

impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        let mut result = 0;
        let mut base = 1_000;
        while base <= n {
            result += n - base + 1;
            base *= 1_000;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::count_commas_in_range::Solution;

    #[test]
    fn test_count_commas_1() {
        let n = 1002;
        assert_eq!(3, Solution::count_commas(n));
    }

    #[test]
    fn test_count_commas_2() {
        let n = 998;
        assert_eq!(0, Solution::count_commas(n));
    }

    #[test]
    fn test_count_commas_3() {
        let n = 2019;
        assert_eq!(1020, Solution::count_commas(n));
    }
}
