// # 3539. Find Sum of Array Product of Magical Sequences
// https://leetcode.com/problems/find-sum-of-array-product-of-magical-sequences/

struct Solution;

impl Solution {
    pub fn magical_sum(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = m as usize;
        let k = k as usize;
        let mod_val = 1_000_000_007u64;

        fn quickmul(x: u64, mut y: u64, mod_val: u64) -> u64 {
            let mut res = 1;
            let mut cur = x % mod_val;
            while y > 0 {
                if y & 1 == 1 {
                    res = res * cur % mod_val;
                }
                y >>= 1;
                cur = cur * cur % mod_val;
            }
            res
        }

        let mut fac = vec![0u64; m + 1];
        fac[0] = 1;
        for i in 1..=m {
            fac[i] = fac[i - 1] * i as u64 % mod_val;
        }

        let mut ifac = vec![0u64; m + 1];
        ifac[0] = 1;
        ifac[1] = 1;
        for i in 2..=m {
            ifac[i] = quickmul(i as u64, mod_val - 2, mod_val);
        }
        for i in 2..=m {
            ifac[i] = ifac[i - 1] * ifac[i] % mod_val;
        }

        let mut nums_power = vec![vec![0u64; m + 1]; n];
        for i in 0..n {
            nums_power[i][0] = 1;
            let num = nums[i] as u64 % mod_val;
            for j in 1..=m {
                nums_power[i][j] = nums_power[i][j - 1] * num % mod_val;
            }
        }

        let mut f = vec![vec![vec![vec![0u64; k + 1]; m * 2 + 1]; m + 1]; n];
        for j in 0..=m {
            f[0][j][j][0] = nums_power[0][j] * ifac[j] % mod_val;
        }

        for i in 0..n - 1 {
            for j in 0..=m {
                for p in 0..=m * 2 {
                    for q in 0..=k {
                        let current = f[i][j][p][q];
                        let q2 = (p % 2) as usize + q;
                        if q2 > k {
                            break;
                        }

                        for r in 0..=(m - j) {
                            let p2 = (p / 2) as usize + r;
                            let add_val =
                                current * nums_power[i + 1][r] % mod_val * ifac[r] % mod_val;
                            f[i + 1][j + r][p2][q2] = (f[i + 1][j + r][p2][q2] + add_val) % mod_val;
                        }
                    }
                }
            }
        }

        let mut res = 0u64;
        for p in 0..=m * 2 {
            for q in 0..=k {
                if p.count_ones() as usize + q == k {
                    res = (res + f[n - 1][m][p][q] * fac[m] % mod_val) % mod_val;
                }
            }
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_sum_of_array_product_of_magical_sequences::Solution;

    #[test]
    fn test_magical_sum_1() {
        let m = 5;
        let k = 5;
        let nums = [1, 10, 100, 10000, 1000000].to_vec();
        assert_eq!(991600007, Solution::magical_sum(m, k, nums));
    }

    #[test]
    fn test_magical_sum_2() {
        let m = 2;
        let k = 2;
        let nums = [5, 4, 3, 2, 1].to_vec();
        assert_eq!(170, Solution::magical_sum(m, k, nums));
    }

    #[test]
    fn test_magical_sum_3() {
        let m = 1;
        let k = 1;
        let nums = [28].to_vec();
        assert_eq!(28, Solution::magical_sum(m, k, nums));
    }
}
