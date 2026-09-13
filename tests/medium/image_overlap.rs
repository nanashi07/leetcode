// 835. Image Overlap
// https://leetcode.com/problems/image-overlap/

struct Solution;

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::image_overlap::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_largest_overlap_1() {
        let img1 = to_vec2d([[1, 1, 0], [0, 1, 0], [0, 1, 0]]);
        let img2 = to_vec2d([[0, 0, 0], [0, 1, 1], [0, 0, 1]]);
        assert_eq!(3, Solution::largest_overlap(img1, img2));
    }

    #[test]
    fn test_largest_overlap_2() {
        let img1 = to_vec2d([[1]]);
        let img2 = to_vec2d([[1]]);
        assert_eq!(1, Solution::largest_overlap(img1, img2));
    }

    #[test]
    fn test_largest_overlap_3() {
        let img1 = to_vec2d([[0]]);
        let img2 = to_vec2d([[0]]);
        assert_eq!(0, Solution::largest_overlap(img1, img2));
    }
}
