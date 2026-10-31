// 3665. Twisted Mirror Path Count
// https://leetcode.com/problems/twisted-mirror-path-count/

struct Solution;

impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::twisted_mirror_path_count::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_unique_paths_1() {
        let grid = to_vec2d([[0, 1, 0], [0, 0, 1], [1, 0, 0]]);
        assert_eq!(5, Solution::unique_paths(grid));
    }

    #[test]
    fn test_unique_paths_2() {
        let grid = to_vec2d([[0, 0], [0, 0]]);
        assert_eq!(2, Solution::unique_paths(grid));
    }

    #[test]
    fn test_unique_paths_3() {
        let grid = to_vec2d([[0, 1, 1], [1, 1, 0]]);
        assert_eq!(1, Solution::unique_paths(grid));
    }
}
