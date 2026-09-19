// 1401. Circle and Rectangle Overlapping
// https://leetcode.com/problems/circle-and-rectangle-overlapping/

struct Solution;

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::circle_and_rectangle_overlapping::Solution;

    #[test]
    fn test_check_overlap_1() {
        let radius = 1;
        let x_center = 0;
        let y_center = 0;
        let x1 = 1;
        let y1 = -1;
        let x2 = 3;
        let y2 = 1;
        assert!(Solution::check_overlap(
            radius, x_center, y_center, x1, y1, x2, y2
        ));
    }

    #[test]
    fn test_check_overlap_2() {
        let radius = 1;
        let x_center = 1;
        let y_center = 1;
        let x1 = 1;
        let y1 = -3;
        let x2 = 2;
        let y2 = -1;
        assert!(!Solution::check_overlap(
            radius, x_center, y_center, x1, y1, x2, y2
        ));
    }

    #[test]
    fn test_check_overlap_3() {
        let radius = 1;
        let x_center = 0;
        let y_center = 0;
        let x1 = -1;
        let y1 = 0;
        let x2 = 0;
        let y2 = 1;
        assert!(Solution::check_overlap(
            radius, x_center, y_center, x1, y1, x2, y2
        ));
    }
}
