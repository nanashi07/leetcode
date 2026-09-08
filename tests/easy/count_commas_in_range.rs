// 3870. Count Commas in Range
// https://leetcode.com/problems/count-commas-in-range/

struct Solution;

impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        let mut n = n;
        let mut c = 0;
        let mut x = 1;
        while n >= 1000 {
            let d = n % 1000 + 1;
            c += x * d;
            n = n / 1000;
            x *= 1000;
        }
        c
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
