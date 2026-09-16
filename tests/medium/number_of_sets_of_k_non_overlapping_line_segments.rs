// 1621. Number of Sets of K Non-Overlapping Line Segments
// https://leetcode.com/problems/number-of-sets-of-k-non-overlapping-line-segments/

struct Solution;

impl Solution {
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let (n, k) = (n as usize, k as usize);

        // `prev[i]` = ways to place the previous number of segments with every endpoint in 1..=i.
        // Adding one segment whose right end is exactly `i`: its left end `l` may be any point
        // below `i`, and earlier segments must fit inside 1..=l, so new[i] = new[i-1] + sum(prev[1..i]).
        let mut prev = vec![1i64; n + 1]; // zero segments: one way for each prefix
        for _ in 0..k {
            let mut cur = vec![0i64; n + 1];
            let mut sum = 0;
            for i in 1..=n {
                cur[i] = (cur[i - 1] + sum) % MOD;
                sum = (sum + prev[i]) % MOD;
            }
            prev = cur;
        }

        prev[n] as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_sets_of_k_non_overlapping_line_segments::Solution;

    #[test]
    fn test_number_of_sets_1() {
        let n = 4;
        let k = 2;
        assert_eq!(5, Solution::number_of_sets(n, k));
    }

    #[test]
    fn test_number_of_sets_2() {
        let n = 3;
        let k = 1;
        assert_eq!(3, Solution::number_of_sets(n, k));
    }

    #[test]
    fn test_number_of_sets_3() {
        let n = 30;
        let k = 7;
        assert_eq!(796297179, Solution::number_of_sets(n, k));
    }
}
