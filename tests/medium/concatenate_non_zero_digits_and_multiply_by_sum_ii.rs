// 3756. Concatenate Non-Zero Digits and Multiply by Sum II
// https://leetcode.com/problems/concatenate-non-zero-digits-and-multiply-by-sum-ii/

struct Solution;

impl Solution {
    pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MOD: i64 = 1_000_000_007;
        const INV10: i64 = 700000005;

        let n = s.len();
        let mut pow10 = vec![1i64; n + 1];
        let mut pow_inv10 = vec![1i64; n + 1];
        for i in 1..=n {
            pow10[i] = (pow10[i - 1] * 10) % MOD;
            pow_inv10[i] = (pow_inv10[i - 1] * INV10) % MOD;
        }

        let mut c_prefix = vec![0; n + 1];
        let mut sum_prefix = vec![0i64; n + 1];
        let mut a = vec![0i64; n + 1];

        let bytes = s.as_bytes();
        for i in 0..n {
            let digit = (bytes[i] - b'0') as i64;
            let is_nonzero = digit != 0;
            c_prefix[i + 1] = c_prefix[i] + if is_nonzero { 1 } else { 0 };
            sum_prefix[i + 1] = sum_prefix[i] + digit;

            let contribution = if is_nonzero {
                (digit * pow_inv10[c_prefix[i + 1]]) % MOD
            } else {
                0
            };
            a[i + 1] = (a[i] + contribution) % MOD;
        }

        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            if c_prefix[r + 1] - c_prefix[l] == 0 {
                ans.push(0);
            } else {
                let diff = (a[r + 1] - a[l] + MOD) % MOD;
                let concat_val = (diff * pow10[c_prefix[r + 1]]) % MOD;
                let sum_val = sum_prefix[r + 1] - sum_prefix[l];
                let val = (concat_val * sum_val) % MOD;
                ans.push(val as i32);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::concatenate_non_zero_digits_and_multiply_by_sum_ii::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_sum_and_multiply_1() {
        let s = "10203004".to_string();
        let queries = to_vec2d([[0, 7], [1, 3], [4, 6]]);
        assert_eq!(
            [12340, 4, 9].to_vec(),
            Solution::sum_and_multiply(s, queries)
        );
    }

    #[test]
    fn test_sum_and_multiply_2() {
        let s = "1000".to_string();
        let queries = to_vec2d([[0, 3], [1, 1]]);
        assert_eq!([1, 0].to_vec(), Solution::sum_and_multiply(s, queries));
    }

    #[test]
    fn test_sum_and_multiply_3() {
        let s = "9876543210".to_string();
        let queries = to_vec2d([[0, 9]]);
        assert_eq!([444444137].to_vec(), Solution::sum_and_multiply(s, queries));
    }
}
