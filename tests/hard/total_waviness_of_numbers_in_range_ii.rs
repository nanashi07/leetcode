// 3753. Total Waviness of Numbers in Range II
// https://leetcode.com/problems/total-waviness-of-numbers-in-range-ii/

struct Solution;

impl Solution {
    pub fn total_waviness(num1: i64, num2: i64) -> i64 {
        fn solve(x: i64) -> i64 {
            if x <= 0 {
                return 0;
            }

            let digits: Vec<u8> = x
                .to_string()
                .bytes()
                .map(|b| b - b'0')
                .collect();
            let n = digits.len();

            let mut memo: Vec<Vec<Vec<[Option<(i64, i64)>; 11]>>> =
                vec![vec![vec![[None; 11]; 2]; 2]; n + 1];

            fn dfs(
                pos: usize,
                tight: usize,
                started: usize,
                prev: usize,
                digits: &[u8],
                memo: &mut [Vec<Vec<[Option<(i64, i64)>; 11]>>],
            ) -> (i64, i64) {
                if pos == digits.len() {
                    return (1, 0);
                }

                if let Some(v) = memo[pos][tight][started][prev] {
                    return v;
                }

                let limit = if tight == 1 { digits[pos] as usize } else { 9 };
                let mut total_count = 0i64;
                let mut total_sum = 0i64;

                for d in 0..=limit {
                    let next_tight = if tight == 1 && d == limit { 1 } else { 0 };

                    if started == 0 && d == 0 {
                        let (cnt, s) = dfs(pos + 1, next_tight, 0, 10, digits, memo);
                        total_count += cnt;
                        total_sum += s;
                    } else {
                        let next_started = 1;
                        let next_prev = d;
                        let (cnt, s) = dfs(pos + 1, next_tight, next_started, next_prev, digits, memo);

                        let add = if started == 1 && prev != d { cnt } else { 0 };
                        total_count += cnt;
                        total_sum += s + add;
                    }
                }

                let res = (total_count, total_sum);
                memo[pos][tight][started][prev] = Some(res);
                res
            }

            let (_, sum) = dfs(0, 1, 0, 10, &digits, &mut memo);
            sum
        }

        solve(num2) - solve(num1 - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::total_waviness_of_numbers_in_range_ii::Solution;

    #[test]
    fn test_total_waviness_1() {
        let num1 = 120;
        let num2 = 130;
        assert_eq!(3, Solution::total_waviness(num1, num2));
    }

    #[test]
    fn test_total_waviness_2() {
        let num1 = 198;
        let num2 = 202;
        assert_eq!(3, Solution::total_waviness(num1, num2));
    }

    #[test]
    fn test_total_waviness_3() {
        let num1 = 4848;
        let num2 = 4848;
        assert_eq!(2, Solution::total_waviness(num1, num2));
    }
}
