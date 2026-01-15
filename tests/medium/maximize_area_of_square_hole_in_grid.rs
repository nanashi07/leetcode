// 2943. Maximize Area of Square Hole in Grid
// https://leetcode.com/problems/maximize-area-of-square-hole-in-grid/

struct Solution;

impl Solution {
    pub fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximize_area_of_square_hole_in_grid::Solution;

    #[test]
    fn test_maximize_square_hole_area_1() {
        let n = 2;
        let m = 1;
        let h_bars = [2, 3].to_vec();
        let v_bars = [2].to_vec();
        assert_eq!(4, Solution::maximize_square_hole_area(n, m, h_bars, v_bars));
    }

    #[test]
    fn test_maximize_square_hole_area_2() {
        let n = 1;
        let m = 1;
        let h_bars = [2].to_vec();
        let v_bars = [2].to_vec();
        assert_eq!(4, Solution::maximize_square_hole_area(n, m, h_bars, v_bars));
    }

    #[test]
    fn test_maximize_square_hole_area_3() {
        let n = 2;
        let m = 3;
        let h_bars = [2, 3].to_vec();
        let v_bars = [2, 4].to_vec();
        assert_eq!(4, Solution::maximize_square_hole_area(n, m, h_bars, v_bars));
    }
}
