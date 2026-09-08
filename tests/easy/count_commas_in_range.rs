// 3870. Count Commas in Range
// https://leetcode.com/problems/count-commas-in-range/

struct Solution;

impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        todo!()
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
}
