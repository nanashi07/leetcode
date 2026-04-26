// 1559. Detect Cycles in 2D Grid
// https://leetcode.com/problems/detect-cycles-in-2d-grid/

struct Solution;

impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        todo!()
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
