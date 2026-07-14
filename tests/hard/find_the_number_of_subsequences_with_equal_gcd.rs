// 3336. Find the Number of Subsequences With Equal GCD
// https://leetcode.com/problems/find-the-number-of-subsequences-with-equal-gcd/

struct Solution;

impl Solution {
    pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;

        fn gcd(mut a: usize, mut b: usize) -> usize {
            while b != 0 {
                let r = a % b;
                a = b;
                b = r;
            }
            a
        }

        let max_num = nums.iter().copied().max().unwrap() as usize;
        let size = max_num + 1;
        let mut dp = vec![vec![0_i32; size]; size];
        dp[0][0] = 1;

        for num in nums {
            let value = num as usize;
            let mut next = dp.clone();

            for g1 in 0..size {
                for g2 in 0..size {
                    let ways = dp[g1][g2];
                    if ways == 0 {
                        continue;
                    }

                    let ng1 = if g1 == 0 { value } else { gcd(g1, value) };
                    let ng2 = if g2 == 0 { value } else { gcd(g2, value) };

                    next[ng1][g2] = ((next[ng1][g2] as i64 + ways as i64) % MOD as i64) as i32;
                    next[g1][ng2] = ((next[g1][ng2] as i64 + ways as i64) % MOD as i64) as i32;
                }
            }

            dp = next;
        }

        let mut answer = 0_i64;
        for g in 1..size {
            answer += dp[g][g] as i64;
        }

        (answer % MOD as i64) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_the_number_of_subsequences_with_equal_gcd::Solution;

    #[test]
    fn test_subsequence_pair_count_1() {
        let nums = [1, 2, 3, 4].to_vec();
        assert_eq!(10, Solution::subsequence_pair_count(nums));
    }

    #[test]
    fn test_subsequence_pair_count_2() {
        let nums = [10, 20, 30].to_vec();
        assert_eq!(2, Solution::subsequence_pair_count(nums));
    }

    #[test]
    fn test_subsequence_pair_count_3() {
        let nums = [1, 1, 1, 1].to_vec();
        assert_eq!(50, Solution::subsequence_pair_count(nums));
    }
}
