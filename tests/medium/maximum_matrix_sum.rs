// 1975. Maximum Matrix Sum
// https://leetcode.com/problems/maximum-matrix-sum/

struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut total_sum: i64 = 0;
        let mut min_abs_value = i32::MAX;
        let mut negative_count = 0;

        for row in &matrix {
            for &val in row {
                let abs_val = val.abs();
                total_sum += abs_val as i64;
                min_abs_value = min_abs_value.min(abs_val);

                if val < 0 {
                    negative_count += 1;
                }
            }
        }

        // If we have an odd number of negatives, we must keep one negative
        // The best strategy is to make the smallest absolute value negative
        if negative_count % 2 == 1 {
            total_sum - 2 * min_abs_value as i64
        } else {
            total_sum
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_matrix_sum::Solution;

    #[test]
    fn test_max_matrix_sum_1() {
        let matrix = [[1, -1], [-1, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(4, Solution::max_matrix_sum(matrix));
    }

    #[test]
    fn test_max_matrix_sum_2() {
        let matrix = [[1, 2, 3], [-1, -2, -3], [1, 2, 3]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(16, Solution::max_matrix_sum(matrix));
    }
}
