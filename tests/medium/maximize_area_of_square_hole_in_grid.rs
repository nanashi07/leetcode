// 2943. Maximize Area of Square Hole in Grid
// https://leetcode.com/problems/maximize-area-of-square-hole-in-grid/

struct Solution;

impl Solution {
    pub fn maximize_square_hole_area(_n: i32, _m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        // Helper function to find the maximum consecutive sequence length
        fn max_consecutive(mut bars: Vec<i32>) -> i32 {
            if bars.is_empty() {
                return 0;
            }

            bars.sort_unstable();
            let mut max_len = 1;
            let mut current_len = 1;

            for i in 1..bars.len() {
                if bars[i] == bars[i - 1] + 1 {
                    current_len += 1;
                    max_len = max_len.max(current_len);
                } else {
                    current_len = 1;
                }
            }

            max_len
        }

        // Find the longest consecutive sequences in both directions
        let h_consecutive = max_consecutive(h_bars);
        let v_consecutive = max_consecutive(v_bars);

        // The side length of the square hole is (consecutive_count + 1)
        // We take the minimum because we need a square
        let side_length = h_consecutive.min(v_consecutive) + 1;

        // Return the area
        side_length * side_length
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
