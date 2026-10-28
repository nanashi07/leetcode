// 3882. Minimum XOR Path in a Grid
// https://leetcode.com/problems/minimum-xor-path-in-a-grid/

const WORD_BITS: usize = u64::BITS as usize;

struct Solution;

impl Solution {
    // A minimum XOR so far is not optimal substructure (a larger prefix can end up smaller once
    // later cells are XORed in), so keep the whole *set* of reachable XOR values per cell as a
    // bitset. Every path XOR stays inside `0..size`, where `size` is the next power of two above
    // the largest cell value, so one row of cells needs `n * size / 64` words.
    // Time O(m * n * size / 64), space O(n * size / 64).
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let max = grid
            .iter()
            .flat_map(|row| row.iter().copied())
            .max()
            .unwrap_or(0) as usize;

        // Smallest power of two strictly above every cell value; XORs of such values stay below it.
        let mut size = 1usize;
        while size <= max {
            size <<= 1;
        }
        let words = (size / WORD_BITS).max(1);
        let n = grid[0].len();

        // `dp[j]` is the reachable set of the cell above (`i - 1`) until it is overwritten with
        // the current row's value, which is also the left neighbour of `j + 1`.
        let mut dp = vec![0u64; n * words];
        let mut union = vec![0u64; words];
        let mut next = vec![0u64; words];

        for (i, row) in grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                union.fill(0);
                if i == 0 && j == 0 {
                    union[0] = 1; // empty prefix, the start cell is XORed in below
                } else {
                    if i > 0 {
                        Self::or_into(&mut union, &dp[j * words..][..words]);
                    }
                    if j > 0 {
                        Self::or_into(&mut union, &dp[(j - 1) * words..][..words]);
                    }
                }

                next.fill(0);
                Self::xor_into(&mut next, &union, cell as usize);
                dp[j * words..][..words].copy_from_slice(&next);
            }
        }

        // The cheapest path is the lowest reachable value of the bottom-right cell; a grid always
        // has at least one path, so the bitset is never empty.
        dp[(n - 1) * words..]
            .iter()
            .enumerate()
            .find_map(|(word, &bits)| {
                (bits != 0).then_some(word * WORD_BITS + bits.trailing_zeros() as usize)
            })
            .unwrap_or_default() as i32
    }

    /// `dst |= src` for two bitsets of equal length.
    fn or_into(dst: &mut [u64], src: &[u64]) {
        dst.iter_mut().zip(src).for_each(|(d, s)| *d |= *s);
    }

    /// Set in `dst` every value obtained by XORing a set bit of `src` with `value`.
    fn xor_into(dst: &mut [u64], src: &[u64], value: usize) {
        for (word, mut bits) in src.iter().copied().enumerate() {
            while bits != 0 {
                let target = (word * WORD_BITS + bits.trailing_zeros() as usize) ^ value;
                dst[target / WORD_BITS] |= 1 << (target % WORD_BITS);
                bits &= bits - 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_xor_path_in_a_grid::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_min_cost_1() {
        let grid = to_vec2d([[1, 2], [3, 4]]);
        assert_eq!(6, Solution::min_cost(grid));
    }

    #[test]
    fn test_min_cost_2() {
        let grid = to_vec2d([[6, 7], [5, 8]]);
        assert_eq!(9, Solution::min_cost(grid));
    }

    #[test]
    fn test_min_cost_3() {
        let grid = to_vec2d([[2, 7, 5]]);
        assert_eq!(0, Solution::min_cost(grid));
    }
}
