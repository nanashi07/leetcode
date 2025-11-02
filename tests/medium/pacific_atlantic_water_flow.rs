// 417. Pacific Atlantic Water Flow
// https://leetcode.com/problems/pacific-atlantic-water-flow/

struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if heights.is_empty() || heights[0].is_empty() {
            return vec![];
        }

        let m = heights.len();
        let n = heights[0].len();

        let mut pacific = vec![vec![false; n]; m];
        let mut atlantic = vec![vec![false; n]; m];

        // DFS from Pacific ocean (top and left edges)
        for i in 0..m {
            Self::dfs(&heights, &mut pacific, i, 0);
        }
        for j in 0..n {
            Self::dfs(&heights, &mut pacific, 0, j);
        }

        // DFS from Atlantic ocean (bottom and right edges)
        for i in 0..m {
            Self::dfs(&heights, &mut atlantic, i, n - 1);
        }
        for j in 0..n {
            Self::dfs(&heights, &mut atlantic, m - 1, j);
        }

        // Collect cells reachable from both oceans
        let mut result = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if pacific[i][j] && atlantic[i][j] {
                    result.push(vec![i as i32, j as i32]);
                }
            }
        }

        result
    }

    fn dfs(heights: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
        let m = heights.len();
        let n = heights[0].len();

        visited[i][j] = true;

        // Explore all 4 directions
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for (di, dj) in directions {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            // Check bounds
            if ni < 0 || ni >= m as i32 || nj < 0 || nj >= n as i32 {
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            // Skip if already visited
            if visited[ni][nj] {
                continue;
            }

            // Water can flow from higher or equal height to current cell
            // So we explore neighbors with height >= current height
            if heights[ni][nj] >= heights[i][j] {
                Self::dfs(heights, visited, ni, nj);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::pacific_atlantic_water_flow::Solution;

    #[test]
    fn test_pacific_atlantic_1() {
        let heights = [
            [1, 2, 2, 3, 5],
            [3, 2, 3, 4, 4],
            [2, 4, 5, 3, 1],
            [6, 7, 1, 4, 5],
            [5, 1, 1, 2, 4],
        ]
        .into_iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        let output = [[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(output, Solution::pacific_atlantic(heights));
    }

    #[test]
    fn test_pacific_atlantic_2() {
        let heights = [[1]].into_iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        let output = [[0, 0]].into_iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        assert_eq!(output, Solution::pacific_atlantic(heights));
    }
}
