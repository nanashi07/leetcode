// 3453. Separate Squares I
// https://leetcode.com/problems/separate-squares-i/

struct Solution;

impl Solution {
    // https://leetcode.com/problems/separate-squares-i/editorial/
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut max_y: f64 = 0.0;
        let mut total_area: f64 = 0.0;

        for sq in &squares {
            let l = sq[2] as f64;
            total_area += l * l;
            max_y = max_y.max((sq[1] + sq[2]) as f64);
        }

        let mut lo = 0.0;
        let mut hi = max_y;
        let eps = 1e-5;
        while (hi - lo).abs() > eps {
            let mid = (hi + lo) / 2.0;
            if Self::check(mid, &squares, total_area) {
                hi = mid;
            } else {
                lo = mid;
            }
        }

        hi
    }

    fn check(limit_y: f64, squares: &Vec<Vec<i32>>, total_area: f64) -> bool {
        let mut area = 0.0;

        for sq in squares {
            let y = sq[1] as f64;
            let l = sq[2] as f64;
            if y < limit_y {
                let overlap = (limit_y - y).min(l);
                area += l * overlap;
            }
        }

        area >= total_area / 2.0
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::separate_squares_i::Solution;

    #[test]
    fn test_separate_squares_1() {
        let squares = [[0, 0, 1], [2, 2, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(
            1.00000,
            (100000.0 * Solution::separate_squares(squares)).floor() / 100000.0
        );
    }

    #[test]
    fn test_separate_squares_2() {
        let squares = [[0, 0, 2], [1, 1, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(
            1.16667,
            (100000.0 * Solution::separate_squares(squares)).floor() / 100000.0
        );
    }

    #[test]
    fn test_separate_squares_3() {
        let squares = [[23, 29, 3], [28, 29, 4]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(
            30.78572,
            (100000.0 * Solution::separate_squares(squares)).floor() / 100000.0
        );
    }
}
