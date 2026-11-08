// 1267. Count Servers that Communicate
// https://leetcode.com/problems/count-servers-that-communicate/

struct Solution;

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut rows = vec![0; m];
        let mut cols = vec![0; n];

        for (r, row) in grid.iter().enumerate() {
            for (c, &v) in row.iter().enumerate() {
                if v == 1 {
                    rows[r] += 1;
                    cols[c] += 1;
                }
            }
        }

        let mut ans = 0;
        for (r, row) in grid.iter().enumerate() {
            for (c, &v) in row.iter().enumerate() {
                if v == 1 && (rows[r] > 1 || cols[c] > 1) {
                    ans += 1;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_servers_that_communicate::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_count_servers_1() {
        let grid = to_vec2d([[1, 0], [0, 1]]);
        assert_eq!(0, Solution::count_servers(grid));
    }

    #[test]
    fn test_count_servers_2() {
        let grid = to_vec2d([[1, 0], [1, 1]]);
        assert_eq!(3, Solution::count_servers(grid));
    }

    #[test]
    fn test_count_servers_3() {
        let grid = to_vec2d([[1, 1, 0, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 1]]);
        assert_eq!(4, Solution::count_servers(grid));
    }
}
