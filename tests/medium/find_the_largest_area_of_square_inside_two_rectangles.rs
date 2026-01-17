// 3047. Find the Largest Area of Square Inside Two Rectangles
// https://leetcode.com/problems/find-the-largest-area-of-square-inside-two-rectangles/

struct Solution;

impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_the_largest_area_of_square_inside_two_rectangles::Solution;

    #[test]
    fn test_largest_square_area_1() {
        let bottom_left = [[1, 1], [2, 2], [3, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let top_right = [[3, 3], [4, 4], [6, 6]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::largest_square_area(bottom_left, top_right));
    }

    #[test]
    fn test_largest_square_area_2() {
        let bottom_left = [[1, 1], [1, 3], [1, 5]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let top_right = [[5, 5], [5, 7], [5, 9]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(4, Solution::largest_square_area(bottom_left, top_right));
    }
}
