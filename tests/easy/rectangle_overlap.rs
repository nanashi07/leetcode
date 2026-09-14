// 836. Rectangle Overlap
// https://leetcode.com/problems/rectangle-overlap/

struct Solution;

impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::rectangle_overlap::Solution;

    #[test]
    fn test_is_rectangle_overlap_1() {
        let rec1 = [0, 0, 2, 2].to_vec();
        let rec2 = [1, 1, 3, 3].to_vec();
        assert!(Solution::is_rectangle_overlap(rec1, rec2));
    }

    #[test]
    fn test_is_rectangle_overlap_2() {
        let rec1 = [0, 0, 1, 1].to_vec();
        let rec2 = [1, 0, 2, 1].to_vec();
        assert!(!Solution::is_rectangle_overlap(rec1, rec2));
    }

    #[test]
    fn test_is_rectangle_overlap_3() {
        let rec1 = [0, 0, 1, 1].to_vec();
        let rec2 = [2, 2, 3, 3].to_vec();
        assert!(!Solution::is_rectangle_overlap(rec1, rec2));
    }
}
