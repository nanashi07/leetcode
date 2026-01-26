// 1200. Minimum Absolute Difference
// https://leetcode.com/problems/minimum-absolute-difference/

struct Solution;

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::minimum_absolute_difference::Solution;

    #[test]
    fn test_minimum_abs_difference_1() {
        let arr = [4, 2, 1, 3].to_vec();
        let output = [[1, 2], [2, 3], [3, 4]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(output, Solution::minimum_abs_difference(arr));
    }

    #[test]
    fn test_minimum_abs_difference_2() {
        let arr = [1, 3, 6, 10, 15].to_vec();
        let output = [[1, 3]].iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        assert_eq!(output, Solution::minimum_abs_difference(arr));
    }

    #[test]
    fn test_minimum_abs_difference_3() {
        let arr = [3, 8, -10, 23, 19, -4, -14, 27].to_vec();
        let output = [[-14, -10], [19, 23], [23, 27]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(output, Solution::minimum_abs_difference(arr));
    }
}
