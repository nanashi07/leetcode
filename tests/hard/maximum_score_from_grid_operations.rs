// 3225. Maximum Score From Grid Operations
// https://leetcode.com/problems/maximum-score-from-grid-operations/

struct Solution;

impl Solution {
    pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid.len();
        let mut prefix = vec![vec![0i64; n + 1]; n];
        for j in 0..n {
            for i in 0..n {
                prefix[j][i + 1] = prefix[j][i] + grid[i][j] as i64;
            }
        }

        let mut prev_pick = vec![0i64; n + 1];
        let mut prev_skip = vec![0i64; n + 1];

        for j in 1..n {
            let mut curr_pick = vec![0i64; n + 1];
            let mut curr_skip = vec![0i64; n + 1];

            for curr in 0..=n {
                for prev in 0..=n {
                    if curr > prev {
                        // Current column's black region extends deeper than prev's.
                        // Score white cells in column j-1 at rows [prev, curr) from right neighbor.
                        let score = prefix[j - 1][curr] - prefix[j - 1][prev];
                        curr_pick[curr] = curr_pick[curr].max(prev_skip[prev] + score);
                        curr_skip[curr] = curr_skip[curr].max(prev_skip[prev] + score);
                    } else {
                        // Previous column's black region extends deeper.
                        // Score white cells in column j at rows [curr, prev) from left neighbor.
                        let score = prefix[j][prev] - prefix[j][curr];
                        curr_pick[curr] = curr_pick[curr].max(prev_pick[prev] + score);
                        curr_skip[curr] = curr_skip[curr].max(prev_pick[prev]);
                    }
                }
            }

            prev_pick = curr_pick;
            prev_skip = curr_skip;
        }

        *prev_pick.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_score_from_grid_operations::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_maximum_score_1() {
        let grid = to_vec2d([
            [0, 0, 0, 0, 0],
            [0, 0, 3, 0, 0],
            [0, 1, 0, 0, 0],
            [5, 0, 0, 3, 0],
            [0, 0, 0, 0, 2],
        ]);
        assert_eq!(11, Solution::maximum_score(grid));
    }

    #[test]
    fn test_maximum_score_2() {
        let grid = to_vec2d([
            [10, 9, 0, 0, 15],
            [7, 1, 0, 8, 0],
            [5, 20, 0, 11, 0],
            [0, 0, 0, 1, 2],
            [8, 12, 1, 10, 3],
        ]);
        assert_eq!(94, Solution::maximum_score(grid));
    }
}
