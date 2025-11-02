// 498. Diagonal Traverse
// https://leetcode.com/problems/diagonal-traverse/

struct Solution;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat[0].len();
        let mut result = Vec::new();

        let mut row = 0;
        let mut col = 0;
        let mut going_up = true;

        for _ in 0..(m * n) {
            result.push(mat[row][col]);

            if going_up {
                // Moving diagonally up-right
                if col == n - 1 {
                    // Hit right boundary, move down and change direction
                    row += 1;
                    going_up = false;
                } else if row == 0 {
                    // Hit top boundary, move right and change direction
                    col += 1;
                    going_up = false;
                } else {
                    // Normal diagonal movement up-right
                    row -= 1;
                    col += 1;
                }
            } else {
                // Moving diagonally down-left
                if row == m - 1 {
                    // Hit bottom boundary, move right and change direction
                    col += 1;
                    going_up = true;
                } else if col == 0 {
                    // Hit left boundary, move down and change direction
                    row += 1;
                    going_up = true;
                } else {
                    // Normal diagonal movement down-left
                    row += 1;
                    col -= 1;
                }
            }
        }

        result
    }

    // incorrect
    pub fn _find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        println!("mat: {:?}", &mat);

        let m = mat.len() as i32;
        let n = mat[0].len() as i32;

        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut r = vec![];

        for s in 0..=(m - 1 + n - 1) {
            println!("[{}, {}]", i, j);
            let mv = if i <= j { -1 } else { 1 };
            for x in 0..=s {
                if i - mv * x < m && j + mv * x < n {
                    // println!("{:?}", (i - mv * x, j + mv * x));
                    r.push(mat[(i - mv * x) as usize][(j + mv * x) as usize]);
                }
            }

            if j <= i {
                i += 1;
                if i >= m {
                    i = m - 1;
                    j += 1;
                }
            } else {
                j += 1;
                if j >= n {
                    j = n - 1;
                    i += 1;
                }
            }

            (i, j) = (j, i);
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::diagonal_traverse::Solution;

    #[test]
    fn test_find_diagonal_order_1() {
        let mat = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(
            [1, 2, 4, 7, 5, 3, 6, 8, 9].to_vec(),
            Solution::find_diagonal_order(mat)
        );
    }

    #[test]
    fn test_find_diagonal_order_2() {
        let mat = [[1, 2], [3, 4]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!([1, 2, 3, 4].to_vec(), Solution::find_diagonal_order(mat));
    }

    #[test]
    fn test_find_diagonal_order_3() {
        let mat = [[6, 9, 7]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!([6, 9, 7].to_vec(), Solution::find_diagonal_order(mat));
    }

    #[test]
    fn test_find_diagonal_order_4() {
        let mat = [[2, 5, 8], [4, 0, -1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(
            [2, 5, 4, 0, 8, -1].to_vec(),
            Solution::find_diagonal_order(mat)
        );
    }
}
