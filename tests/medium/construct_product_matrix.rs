// 2906. Construct Product Matrix
// https://leetcode.com/problems/construct-product-matrix/

struct Solution;

impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const MOD: i64 = 12345;
        let n = grid.len();
        let m = grid[0].len();
        let total = n * m;

        let flat: Vec<i64> = grid
            .iter()
            .flat_map(|row| row.iter().map(|&x| x as i64 % MOD))
            .collect();

        // prefix[i] = product of flat[0..i] mod MOD
        let mut prefix = vec![1i64; total + 1];
        for i in 0..total {
            prefix[i + 1] = prefix[i] * flat[i] % MOD;
        }

        // suffix[i] = product of flat[i..total] mod MOD
        let mut suffix = vec![1i64; total + 1];
        for i in (0..total).rev() {
            suffix[i] = suffix[i + 1] * flat[i] % MOD;
        }

        (0..n)
            .map(|i| {
                (0..m)
                    .map(|j| {
                        let idx = i * m + j;
                        (prefix[idx] * suffix[idx + 1] % MOD) as i32
                    })
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::construct_product_matrix::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_construct_product_matrix_1() {
        let grid = to_vec2d([[1, 2], [3, 4]]);
        let output = to_vec2d([[24, 12], [8, 6]]);
        assert_eq!(output, Solution::construct_product_matrix(grid));
    }

    #[test]
    fn test_construct_product_matrix_2() {
        let grid = to_vec2d([[12345], [2], [1]]);
        let output = to_vec2d([[2], [0], [0]]);
        assert_eq!(output, Solution::construct_product_matrix(grid));
    }
}
