// 1559. Detect Cycles in 2D Grid
// https://leetcode.com/problems/detect-cycles-in-2d-grid/

struct Solution;

impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut parent: Vec<usize> = (0..m * n).collect();
        let mut rank = vec![0u8; m * n];

        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        fn union(parent: &mut Vec<usize>, rank: &mut Vec<u8>, a: usize, b: usize) -> bool {
            let ra = find(parent, a);
            let rb = find(parent, b);
            if ra == rb {
                return true; // cycle detected
            }
            if rank[ra] < rank[rb] {
                parent[ra] = rb;
            } else if rank[ra] > rank[rb] {
                parent[rb] = ra;
            } else {
                parent[rb] = ra;
                rank[ra] += 1;
            }
            false
        }

        for i in 0..m {
            for j in 0..n {
                let idx = i * n + j;
                // Only check right and down to avoid duplicate edges
                if j + 1 < n && grid[i][j] == grid[i][j + 1] {
                    if union(&mut parent, &mut rank, idx, idx + 1) {
                        return true;
                    }
                }
                if i + 1 < m && grid[i][j] == grid[i + 1][j] {
                    if union(&mut parent, &mut rank, idx, idx + n) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::detect_cycles_in_2d_grid::Solution;
    use crate::shared::vec2d::to_char_vec2d;

    #[test]
    fn test_contains_cycle_1() {
        let grid = to_char_vec2d([
            ["a", "a", "a", "a"],
            ["a", "b", "b", "a"],
            ["a", "b", "b", "a"],
            ["a", "a", "a", "a"],
        ]);
        assert_eq!(true, Solution::contains_cycle(grid));
    }

    #[test]
    fn test_contains_cycle_2() {
        let grid = to_char_vec2d([
            ["c", "c", "c", "a"],
            ["c", "d", "c", "c"],
            ["c", "c", "e", "c"],
            ["f", "c", "c", "c"],
        ]);
        assert_eq!(true, Solution::contains_cycle(grid));
    }

    #[test]
    fn test_contains_cycle_3() {
        let grid = to_char_vec2d([["a", "b", "b"], ["b", "z", "b"], ["b", "b", "a"]]);
        assert_eq!(false, Solution::contains_cycle(grid));
    }
}
