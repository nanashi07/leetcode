// 1536. Minimum Swaps to Arrange a Binary Grid
// https://leetcode.com/problems/minimum-swaps-to-arrange-a-binary-grid/

struct Solution;

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        // Compute trailing zeros for each row
        let mut trailing: Vec<usize> = grid
            .iter()
            .map(|row| row.iter().rev().take_while(|&&x| x == 0).count())
            .collect();

        let mut swaps = 0;
        for i in 0..n {
            let need = n - 1 - i;
            // Find the first row at or after i with enough trailing zeros
            let pos = (i..n).find(|&j| trailing[j] >= need);
            match pos {
                None => return -1,
                Some(j) => {
                    // Bubble row j up to position i
                    for k in (i + 1..=j).rev() {
                        trailing.swap(k, k - 1);
                    }
                    swaps += (j - i) as i32;
                }
            }
        }
        swaps
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_swaps_to_arrange_a_binary_grid::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_min_swaps_1() {
        let grid = to_vec2d([[0, 0, 1], [1, 1, 0], [1, 0, 0]]);
        assert_eq!(3, Solution::min_swaps(grid));
    }

    #[test]
    fn test_min_swaps_2() {
        let grid = to_vec2d([[0, 1, 1, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 1, 1, 0]]);
        assert_eq!(-1, Solution::min_swaps(grid));
    }

    #[test]
    fn test_min_swaps_3() {
        let grid = to_vec2d([[1, 0, 0], [1, 1, 0], [1, 1, 1]]);
        assert_eq!(0, Solution::min_swaps(grid));
    }
}
