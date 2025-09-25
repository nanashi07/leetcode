// # 120. Triangle
// https://leetcode.com/problems/triangle/

struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::triangle::Solution;

    #[test]
    fn test_minimum_total_1() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(11, Solution::minimum_total(triangle));
    }

    #[test]
    fn test_minimum_total_2() {
        let triangle = vec![vec![-10]];
        assert_eq!(-10, Solution::minimum_total(triangle));
    }
}
