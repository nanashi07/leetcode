// 474. Ones and Zeroes
// https://leetcode.com/problems/ones-and-zeroes/

struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;

        let mut dp = vec![vec![0; n + 1]; m + 1];

        for s in &strs {
            let zeros = s.chars().filter(|&c| c == '0').count();
            let ones = s.len() - zeros;

            for i in (zeros..=m).rev() {
                for j in (ones..=n).rev() {
                    dp[i][j] = dp[i][j].max(dp[i - zeros][j - ones] + 1);
                }
            }
        }

        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::ones_and_zeroes::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_find_max_form_1() {
        let strs = to_string_vec(["10", "0001", "111001", "1", "0"]);
        let m = 5;
        let n = 3;
        assert_eq!(4, Solution::find_max_form(strs, m, n));
    }

    #[test]
    fn test_find_max_form_2() {
        let strs = to_string_vec(["10", "0", "1"]);
        let m = 1;
        let n = 1;
        assert_eq!(2, Solution::find_max_form(strs, m, n));
    }

    #[test]
    fn test_find_max_form_3() {
        let strs = to_string_vec(["00011", "00001", "00001", "0011", "111"]);
        let m = 8;
        let n = 5;
        assert_eq!(3, Solution::find_max_form(strs, m, n));
    }
}
