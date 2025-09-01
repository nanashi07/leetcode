// # 1792. Maximum Average Pass Ratio
// https://leetcode.com/problems/maximum-average-pass-ratio/

struct Solution;

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_average_pass_ratio::Solution;

    #[test]
    fn test_max_average_ratio_1() {
        let classes = [[1, 2], [3, 5], [2, 2]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let extra_students = 2;
        assert_eq!(
            0.78333,
            Solution::max_average_ratio(classes, extra_students)
        );
    }

    #[test]
    fn test_max_average_ratio_2() {
        let classes = [[2, 4], [3, 9], [4, 5], [2, 10]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let extra_students = 4;
        assert_eq!(
            0.53485,
            Solution::max_average_ratio(classes, extra_students)
        );
    }
}
