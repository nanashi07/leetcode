// 3197. Find the Minimum Area to Cover All Ones II
// https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones-ii/

struct Solution;

impl Solution {
    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        // Find all positions with 1s
        let mut ones = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    ones.push((i, j));
                }
            }
        }

        let mut min_area = i32::MAX;

        // Case 1: Three horizontal strips
        for i1 in 1..m {
            for i2 in i1 + 1..m {
                let rect1 = Self::get_bounding_rect(&ones, Some((0, i1 - 1)), None);
                let rect2 = Self::get_bounding_rect(&ones, Some((i1, i2 - 1)), None);
                let rect3 = Self::get_bounding_rect(&ones, Some((i2, m - 1)), None);

                if let (Some(r1), Some(r2), Some(r3)) = (rect1, rect2, rect3) {
                    let area = Self::calc_area(r1) + Self::calc_area(r2) + Self::calc_area(r3);
                    min_area = min_area.min(area);
                }
            }
        }

        // Case 2: Three vertical strips
        for j1 in 1..n {
            for j2 in j1 + 1..n {
                let rect1 = Self::get_bounding_rect(&ones, None, Some((0, j1 - 1)));
                let rect2 = Self::get_bounding_rect(&ones, None, Some((j1, j2 - 1)));
                let rect3 = Self::get_bounding_rect(&ones, None, Some((j2, n - 1)));

                if let (Some(r1), Some(r2), Some(r3)) = (rect1, rect2, rect3) {
                    let area = Self::calc_area(r1) + Self::calc_area(r2) + Self::calc_area(r3);
                    min_area = min_area.min(area);
                }
            }
        }

        // Case 3: Top rectangle + bottom two rectangles (horizontal split then vertical)
        for i in 1..m {
            for j in 1..n {
                let rect1 = Self::get_bounding_rect(&ones, Some((0, i - 1)), None);
                let rect2 = Self::get_bounding_rect(&ones, Some((i, m - 1)), Some((0, j - 1)));
                let rect3 = Self::get_bounding_rect(&ones, Some((i, m - 1)), Some((j, n - 1)));

                if let (Some(r1), Some(r2), Some(r3)) = (rect1, rect2, rect3) {
                    let area = Self::calc_area(r1) + Self::calc_area(r2) + Self::calc_area(r3);
                    min_area = min_area.min(area);
                }
            }
        }

        // Case 4: Bottom rectangle + top two rectangles (horizontal split then vertical)
        for i in 1..m {
            for j in 1..n {
                let rect1 = Self::get_bounding_rect(&ones, Some((0, i - 1)), Some((0, j - 1)));
                let rect2 = Self::get_bounding_rect(&ones, Some((0, i - 1)), Some((j, n - 1)));
                let rect3 = Self::get_bounding_rect(&ones, Some((i, m - 1)), None);

                if let (Some(r1), Some(r2), Some(r3)) = (rect1, rect2, rect3) {
                    let area = Self::calc_area(r1) + Self::calc_area(r2) + Self::calc_area(r3);
                    min_area = min_area.min(area);
                }
            }
        }

        // Case 5: Left rectangle + right two rectangles (vertical split then horizontal)
        for j in 1..n {
            for i in 1..m {
                let rect1 = Self::get_bounding_rect(&ones, None, Some((0, j - 1)));
                let rect2 = Self::get_bounding_rect(&ones, Some((0, i - 1)), Some((j, n - 1)));
                let rect3 = Self::get_bounding_rect(&ones, Some((i, m - 1)), Some((j, n - 1)));

                if let (Some(r1), Some(r2), Some(r3)) = (rect1, rect2, rect3) {
                    let area = Self::calc_area(r1) + Self::calc_area(r2) + Self::calc_area(r3);
                    min_area = min_area.min(area);
                }
            }
        }

        // Case 6: Right rectangle + left two rectangles (vertical split then horizontal)
        for j in 1..n {
            for i in 1..m {
                let rect1 = Self::get_bounding_rect(&ones, Some((0, i - 1)), Some((0, j - 1)));
                let rect2 = Self::get_bounding_rect(&ones, Some((i, m - 1)), Some((0, j - 1)));
                let rect3 = Self::get_bounding_rect(&ones, None, Some((j, n - 1)));

                if let (Some(r1), Some(r2), Some(r3)) = (rect1, rect2, rect3) {
                    let area = Self::calc_area(r1) + Self::calc_area(r2) + Self::calc_area(r3);
                    min_area = min_area.min(area);
                }
            }
        }

        min_area
    }

    // Get bounding rectangle for 1s within the specified row and column ranges
    fn get_bounding_rect(
        ones: &[(usize, usize)],
        row_range: Option<(usize, usize)>,
        col_range: Option<(usize, usize)>,
    ) -> Option<(usize, usize, usize, usize)> {
        let mut min_row = usize::MAX;
        let mut max_row = 0;
        let mut min_col = usize::MAX;
        let mut max_col = 0;
        let mut found = false;

        for &(r, c) in ones {
            // Check if this point is within the specified ranges
            if let Some((r_min, r_max)) = row_range {
                if r < r_min || r > r_max {
                    continue;
                }
            }
            if let Some((c_min, c_max)) = col_range {
                if c < c_min || c > c_max {
                    continue;
                }
            }

            found = true;
            min_row = min_row.min(r);
            max_row = max_row.max(r);
            min_col = min_col.min(c);
            max_col = max_col.max(c);
        }

        if found {
            Some((min_row, max_row, min_col, max_col))
        } else {
            None
        }
    }

    // Calculate area of rectangle given (min_row, max_row, min_col, max_col)
    fn calc_area(rect: (usize, usize, usize, usize)) -> i32 {
        let (min_row, max_row, min_col, max_col) = rect;
        ((max_row - min_row + 1) * (max_col - min_col + 1)) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_the_minimum_area_to_cover_all_ones_ii::Solution;

    #[test]
    fn test_minimum_sum_1() {
        let grid = [[1, 0, 1], [1, 1, 1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(5, Solution::minimum_sum(grid));
    }

    #[test]
    fn test_minimum_sum_2() {
        let grid = [[1, 0, 1, 0], [0, 1, 0, 1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(5, Solution::minimum_sum(grid));
    }
}
