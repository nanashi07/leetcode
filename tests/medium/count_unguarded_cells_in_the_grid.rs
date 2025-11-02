// # 2257. Count Unguarded Cells in the Grid
// https://leetcode.com/problems/count-unguarded-cells-in-the-grid/

struct Solution;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_unguarded_cells_in_the_grid::Solution;

    #[test]
    fn test_count_unguarded_1() {
        let m = 4;
        let n = 6;
        let guards = [[0, 0], [1, 1], [2, 3]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let walls = [[0, 1], [2, 2], [1, 4]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(7, Solution::count_unguarded(m, n, guards, walls));
    }

    #[test]
    fn test_count_unguarded_2() {
        let m = 3;
        let n = 3;
        let guards = [[1, 1]].into_iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        let walls = [[0, 1], [1, 0], [2, 1], [1, 2]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(4, Solution::count_unguarded(m, n, guards, walls));
    }
}
