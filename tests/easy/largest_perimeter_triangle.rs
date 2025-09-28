// # 976. Largest Perimeter Triangle
// https://leetcode.com/problems/largest-perimeter-triangle/

struct Solution;

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::largest_perimeter_triangle::Solution;

    #[test]
    fn test_largest_perimeter_1() {
        let nums = [2, 1, 2].to_vec();
        assert_eq!(5, Solution::largest_perimeter(nums));
    }

    #[test]
    fn test_largest_perimeter_2() {
        let nums = [1, 2, 1, 10].to_vec();
        assert_eq!(0, Solution::largest_perimeter(nums));
    }
}
