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

    // Wrong Answer
    pub fn _find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        println!("strs: {:?}, m: {m}, n: {n}", &strs);

        let mut m = m;
        let mut n = n;
        // let mut strs = strs;
        // strs.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut count = 0;
        let cs = strs
            .iter()
            .map(|s| {
                let mut zero = 0;
                let mut one = 0;
                for c in s.chars() {
                    match c {
                        '0' => zero += 1,
                        '1' => one += 1,
                        _ => {}
                    }
                }
                (zero, one)
            })
            .collect::<Vec<_>>();

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::ones_and_zeroes::Solution;

    #[test]
    fn test_find_max_form_1() {
        let strs = ["10", "0001", "111001", "1", "0"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let m = 5;
        let n = 3;
        assert_eq!(4, Solution::find_max_form(strs, m, n));
    }

    #[test]
    fn test_find_max_form_2() {
        let strs = ["10", "0", "1"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let m = 1;
        let n = 1;
        assert_eq!(2, Solution::find_max_form(strs, m, n));
    }

    #[test]
    fn test_find_max_form_3() {
        let strs = ["00011", "00001", "00001", "0011", "111"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let m = 8;
        let n = 5;
        assert_eq!(3, Solution::find_max_form(strs, m, n));
    }
}
