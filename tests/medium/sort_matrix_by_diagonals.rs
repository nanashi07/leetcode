// 3446. Sort Matrix by Diagonals
// https://leetcode.com/problems/sort-matrix-by-diagonals/

struct Solution;

impl Solution {
    pub fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        grid.iter().for_each(|l| println!("{:?}", l));
        let mut grid = grid;

        let l = grid.len();
        let mut i = 0;
        let mut j = l - 1;

        while i < l && j < l {
            let mut bubble = false;
            for m in 0..l {
                let ci = i + m;
                let cj = j + m;
                let ni = i + m + 1;
                let nj = j + m + 1;

                if ni < l && nj < l {
                    println!("move {:?} / {:?}", (ni, nj), (i, j));

                    if i == 0 && grid[cj][ci] < grid[nj][ni] {
                        (grid[cj][ci], grid[nj][ni]) = (grid[nj][ni], grid[cj][ci]);
                        bubble = true;
                    } else if i > 0 && grid[cj][ci] > grid[nj][ni] {
                        (grid[cj][ci], grid[nj][ni]) = (grid[nj][ni], grid[cj][ci]);
                        bubble = true;
                    }
                } else {
                    if !bubble {
                        if i == 0 && j > 0 {
                            j -= 1;
                        } else {
                            i += 1;
                        }
                    }
                    break;
                }
            }
        }

        grid.iter().for_each(|l| println!("{:?}", l));

        grid
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::sort_matrix_by_diagonals::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_sort_matrix_1() {
        let grid = to_vec2d([[1, 7, 3], [9, 8, 2], [4, 5, 6]]);
        let output = to_vec2d([[8, 2, 3], [9, 6, 7], [4, 5, 1]]);
        assert_eq!(output, Solution::sort_matrix(grid));
    }

    #[test]
    fn test_sort_matrix_2() {
        let grid = to_vec2d([[0, 1], [1, 2]]);
        let output = to_vec2d([[2, 1], [1, 0]]);
        assert_eq!(output, Solution::sort_matrix(grid));
    }

    #[test]
    fn test_sort_matrix_3() {
        let grid = to_vec2d([[1]]);
        let output = to_vec2d([[1]]);
        assert_eq!(output, Solution::sort_matrix(grid));
    }

    #[test]
    fn test_sort_matrix_4() {
        let grid = to_vec2d([[-1, -2, -3], [-3, -3, -2], [-4, -4, 0]]);
        let output = to_vec2d([[0, -2, -3], [-3, -1, -2], [-4, -4, -3]]);
        assert_eq!(output, Solution::sort_matrix(grid));
    }
}
