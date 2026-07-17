// 3700. Number of ZigZag Arrays II
// https://leetcode.com/problems/number-of-zigzag-arrays-ii/

struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let n = n as u64;
        let m = (r - l + 1) as usize;

        if n == 1 {
            return (m as u64 % MOD) as i32;
        }
        if n == 2 {
            return ((m as u64 % MOD) * ((m as u64 - 1) % MOD) % MOD) as i32;
        }

        let sz = 2 * m;

        let mut mat = vec![vec![0u64; sz]; sz];
        for v in 0..m {
            for u in 0..v {
                mat[v][m + u] = 1;
            }
        }
        for v in 0..m {
            for u in (v + 1)..m {
                mat[m + v][u] = 1;
            }
        }

        let mat = mat_pow(&mat, n - 1, sz, MOD);

        let mut ans = 0u64;
        for (i, _) in mat.iter().take(sz).enumerate() {
            let mut row_sum = 0u64;
            for (j, _) in mat.iter().take(sz).enumerate() {
                row_sum = (row_sum + mat[i][j]) % MOD;
            }
            ans = (ans + row_sum) % MOD;
        }
        ans as i32
    }
}

fn mat_mul(a: &[Vec<u64>], b: &[Vec<u64>], sz: usize, md: u64) -> Vec<Vec<u64>> {
    let mut c = vec![vec![0u64; sz]; sz];
    for i in 0..sz {
        for k in 0..sz {
            if a[i][k] == 0 {
                continue;
            }
            for j in 0..sz {
                c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % md;
            }
        }
    }
    c
}

#[allow(clippy::needless_range_loop)]
fn mat_pow(base: &[Vec<u64>], mut exp: u64, sz: usize, md: u64) -> Vec<Vec<u64>> {
    let mut result = vec![vec![0u64; sz]; sz];
    for i in 0..sz {
        result[i][i] = 1;
    }
    let mut b = base.to_vec();
    while exp > 0 {
        if exp & 1 == 1 {
            result = mat_mul(&result, &b, sz, md);
        }
        b = mat_mul(&b, &b, sz, md);
        exp >>= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::hard::number_of_zigzag_arrays_ii::Solution;

    #[test]
    fn test_zig_zag_arrays_1() {
        let n = 3;
        let l = 4;
        let r = 5;
        assert_eq!(2, Solution::zig_zag_arrays(n, l, r));
    }

    #[test]
    fn test_zig_zag_arrays_2() {
        let n = 3;
        let l = 1;
        let r = 3;
        assert_eq!(10, Solution::zig_zag_arrays(n, l, r));
    }
}
