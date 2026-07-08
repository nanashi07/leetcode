// 3756. Concatenate Non-Zero Digits and Multiply by Sum II
// https://leetcode.com/problems/concatenate-non-zero-digits-and-multiply-by-sum-ii/

struct Solution;

impl Solution {
    pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::concatenate_non_zero_digits_and_multiply_by_sum_ii::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_sum_and_multiply_1() {
        let s = "10203004".to_string();
        let queries = to_vec2d([[0, 7], [1, 3], [4, 6]]);
        assert_eq!(
            [12340, 4, 9].to_vec(),
            Solution::sum_and_multiply(s, queries)
        );
    }

    #[test]
    fn test_sum_and_multiply_2() {
        let s = "1000".to_string();
        let queries = to_vec2d([[0, 3], [1, 1]]);
        assert_eq!([1, 0].to_vec(), Solution::sum_and_multiply(s, queries));
    }

    #[test]
    fn test_sum_and_multiply_3() {
        let s = "9876543210".to_string();
        let queries = to_vec2d([[0, 9]]);
        assert_eq!([444444137].to_vec(), Solution::sum_and_multiply(s, queries));
    }
}
