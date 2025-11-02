// 2787. Ways to Express an Integer as Sum of Powers
// https://leetcode.com/problems/ways-to-express-an-integer-as-sum-of-powers/

struct Solution;

impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = n as usize;
        let x = x as u32;

        // 1) 預先列出所有 <= n 的 x 次方
        let mut powers: Vec<usize> = Vec::new();
        let mut i: i64 = 1;
        loop {
            let p = i.pow(x) as i64;
            if p > n as i64 {
                break;
            }
            powers.push(p as usize);
            i += 1;
        }

        // 2) 1D DP
        let mut dp = vec![0i64; n + 1];
        dp[0] = 1; // 和為 0 的方式：不選

        for &p in &powers {
            // 0/1 背包：j 逆序
            for j in (p..=n).rev() {
                dp[j] += dp[j - p];
                if dp[j] >= MOD {
                    dp[j] -= MOD;
                } // 等價於 %MOD，但更快
            }
        }

        dp[n] as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::ways_to_express_an_integer_as_sum_of_powers::Solution;

    #[test]
    fn test_number_of_ways_1() {
        let n = 10;
        let x = 2;
        assert_eq!(1, Solution::number_of_ways(n, x));
    }

    #[test]
    fn test_number_of_ways_2() {
        let n = 4;
        let x = 1;
        assert_eq!(2, Solution::number_of_ways(n, x));
    }
}
