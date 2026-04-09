// 3655. XOR After Range Multiplication Queries II
// https://leetcode.com/problems/xor-after-range-multiplication-queries-ii/

struct Solution;

impl Solution {
    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let n = nums.len();
        let sqrt_n = ((n as f64).sqrt() as usize).max(1);

        // p[i] = accumulated product of all multipliers affecting index i
        let mut p = vec![1i64; n];

        // Precompute modular inverses for v in 1..=max_v
        let max_v = queries.iter().map(|q| q[3] as usize).max().unwrap_or(1);
        let mut inv = vec![1i64; max_v + 1];
        for i in 2..=max_v {
            inv[i] = (MOD - (MOD / i as i64) * inv[(MOD % i as i64) as usize] % MOD) % MOD;
        }

        // Separate queries: small k uses diff-array, large k processed directly
        let mut small_k_queries: Vec<Vec<(usize, usize, i64)>> = vec![vec![]; sqrt_n + 1];

        for q in &queries {
            let (l, r, k, v) = (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as i64);
            if k > sqrt_n {
                let mut idx = l;
                while idx <= r {
                    p[idx] = p[idx] * v % MOD;
                    idx += k;
                }
            } else {
                small_k_queries[k].push((l, r, v));
            }
        }

        // Process small-k queries via multiplicative difference arrays on congruence classes
        for k in 1..=sqrt_n {
            if small_k_queries[k].is_empty() {
                continue;
            }

            let mut by_class: Vec<Vec<(usize, usize, i64)>> = vec![vec![]; k];
            for &(l, r, v) in &small_k_queries[k] {
                by_class[l % k].push((l, r, v));
            }

            for c in 0..k {
                if by_class[c].is_empty() {
                    continue;
                }

                let m = (n - 1 - c) / k + 1; // virtual array length for class c
                let mut diff = vec![1i64; m + 1];

                for &(l, r, v) in &by_class[c] {
                    let j_start = (l - c) / k;
                    let j_end = j_start + (r - l) / k;
                    diff[j_start] = diff[j_start] * v % MOD;
                    diff[j_end + 1] = diff[j_end + 1] * inv[v as usize] % MOD;
                }

                let mut running = 1i64;
                for j in 0..m {
                    running = running * diff[j] % MOD;
                    p[c + j * k] = p[c + j * k] * running % MOD;
                }
            }
        }

        let mut result = 0i64;
        for i in 0..n {
            result ^= nums[i] as i64 * p[i] % MOD;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::xor_after_range_multiplication_queries_ii::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_xor_after_queries_1() {
        let nums = [1, 1, 1].to_vec();
        let queries = to_vec2d([[0, 2, 1, 4]]);
        assert_eq!(4, Solution::xor_after_queries(nums, queries));
    }

    #[test]
    fn test_xor_after_queries_2() {
        let nums = [2, 3, 1, 5, 4].to_vec();
        let queries = to_vec2d([[1, 4, 2, 3], [0, 2, 1, 2]]);
        assert_eq!(31, Solution::xor_after_queries(nums, queries));
    }
}
