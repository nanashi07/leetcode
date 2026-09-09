// 3871. Count Commas in Range II
// https://leetcode.com/problems/count-commas-in-range-ii/

struct Solution;

impl Solution {
    pub fn count_commas(n: i64) -> i64 {
        let mut result = 0;
        let mut base = 1_000i64;
        while base <= n {
            result += n - base + 1;
            base *= 1_000;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_commas_in_range_ii::Solution;

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
}
