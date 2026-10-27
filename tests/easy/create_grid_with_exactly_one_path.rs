// 3963. Create Grid With Exactly One Path
// https://leetcode.com/problems/create-grid-with-exactly-one-path/

struct Solution;

impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let (rows, cols) = (m as usize, n as usize);
        let mut grid = vec![vec![b'#'; cols]; rows];
        let (mut r, mut c) = (0usize, 0usize);
        let (mut down, mut right) = (rows - 1, cols - 1);
        let mut go_right = true;

        grid[r][c] = b'.';
        while down > 0 || right > 0 {
            // Alternate right/down so the open cells stay a single corridor; once one
            // direction runs out, keep walking in the other one.
            let step_right = if right == 0 {
                false
            } else if down == 0 {
                true
            } else {
                go_right
            };
            if step_right {
                c += 1;
                right -= 1;
            } else {
                r += 1;
                down -= 1;
            }
            go_right = !go_right;
            grid[r][c] = b'.';
        }

        grid.into_iter()
            .map(|row| row.into_iter().map(char::from).collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::create_grid_with_exactly_one_path::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_create_grid_1() {
        let m = 2;
        let n = 3;
        let output = to_string_vec(["..#", "#.."]);
        assert_eq!(output, Solution::create_grid(m, n));
    }

    #[test]
    fn test_create_grid_2() {
        let m = 3;
        let n = 3;
        let output = to_string_vec(["..#", "#..", "##."]);
        assert_eq!(output, Solution::create_grid(m, n));
    }

    #[test]
    fn test_create_grid_3() {
        let m = 1;
        let n = 4;
        let output = to_string_vec(["...."]);
        assert_eq!(output, Solution::create_grid(m, n));
    }
}
