// # 3195. Find the Minimum Area to Cover All Ones I
// https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones-i/

struct Solution;

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        println!("grid: {:?}", &grid);

        let w = grid.len();
        let h = grid[0].len();

        let mut left = 0;
        let mut right = grid.len() - 1;
        let mut top = 0;
        let mut bottom = grid[0].len() - 1;

        while left < w && (0..h).fold(0, |a, c| a + grid[left][c]) == 0 {
            left += 1;
        }
        while right > 0 && (0..h).fold(0, |a, c| a + grid[right][c]) == 0 {
            right -= 1;
        }
        while top < h && (0..w).fold(0, |a, c| a + grid[c][top]) == 0 {
            top += 1;
        }
        while bottom > 0 && (0..w).fold(0, |a, c| a + grid[c][bottom]) == 0 {
            bottom -= 1;
        }

        println!("left: {left}, right: {right}, top: {top}, bottom: {bottom}");

        ((right - left + 1) * (bottom - top + 1)) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_the_minimum_area_to_cover_all_ones_i::Solution;

    #[test]
    fn test_minimum_area_1() {
        let grid = [[0, 1, 0], [1, 0, 1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(6, Solution::minimum_area(grid));
    }

    #[test]
    fn test_minimum_area_2() {
        let grid = [[1, 0], [0, 0]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(1, Solution::minimum_area(grid));
    }
}
