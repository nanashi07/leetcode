// # 812. Largest Triangle Area
// https://leetcode.com/problems/largest-triangle-area/

struct Solution;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::largest_triangle_area::Solution;

    #[test]
    fn test_largest_triangle_area_1() {
        let points = [[0, 0], [0, 1], [1, 0], [0, 2], [2, 0]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(2.00000, Solution::largest_triangle_area(points));
    }

    #[test]
    fn test_largest_triangle_area_2() {
        let points = [[1, 0], [0, 0], [0, 1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(0.50000, Solution::largest_triangle_area(points));
    }
}
