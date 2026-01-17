// 3047. Find the Largest Area of Square Inside Two Rectangles
// https://leetcode.com/problems/find-the-largest-area-of-square-inside-two-rectangles/

struct Solution;

impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let n = bottom_left.len();
        let mut max_area = 0i64;

        // Check all pairs of rectangles
        for i in 0..n {
            for j in (i + 1)..n {
                // Find intersection of rectangles i and j
                // Intersection bottom-left corner: max of both bottom-left corners
                let intersect_bl_x = bottom_left[i][0].max(bottom_left[j][0]);
                let intersect_bl_y = bottom_left[i][1].max(bottom_left[j][1]);

                // Intersection top-right corner: min of both top-right corners
                let intersect_tr_x = top_right[i][0].min(top_right[j][0]);
                let intersect_tr_y = top_right[i][1].min(top_right[j][1]);

                // Check if there's a valid intersection
                if intersect_bl_x < intersect_tr_x && intersect_bl_y < intersect_tr_y {
                    // Calculate width and height of intersection
                    let width = intersect_tr_x - intersect_bl_x;
                    let height = intersect_tr_y - intersect_bl_y;

                    // The largest square has side length = min(width, height)
                    let side = width.min(height);
                    let area = (side as i64) * (side as i64);

                    max_area = max_area.max(area);
                }
            }
        }

        max_area
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

    #[test]
    fn test_largest_square_area_3() {
        let bottom_left = [[1, 1], [2, 2], [1, 2]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let top_right = [[3, 3], [4, 4], [3, 4]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::largest_square_area(bottom_left, top_right));
    }

    #[test]
    fn test_largest_square_area_4() {
        let bottom_left = [[1, 1], [3, 3], [3, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let top_right = [[2, 2], [4, 4], [4, 2]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(0, Solution::largest_square_area(bottom_left, top_right));
    }
}
