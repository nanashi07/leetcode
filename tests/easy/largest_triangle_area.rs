// # 812. Largest Triangle Area
// https://leetcode.com/problems/largest-triangle-area/

struct Solution;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        println!("points: {:?}", &points);

        let mut max_area: f64 = 0.0;
        let n = points.len();

        // Try all combinations of 3 points
        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    let area = Self::calculate_area(&points[i], &points[j], &points[k]);
                    max_area = max_area.max(area);
                }
            }
        }

        max_area
    }

    // Calculate triangle area using cross product formula
    // For points (x1,y1), (x2,y2), (x3,y3):
    // Area = 0.5 * |x1(y2-y3) + x2(y3-y1) + x3(y1-y2)|
    fn calculate_area(p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>) -> f64 {
        let x1 = p1[0] as f64;
        let y1 = p1[1] as f64;
        let x2 = p2[0] as f64;
        let y2 = p2[1] as f64;
        let x3 = p3[0] as f64;
        let y3 = p3[1] as f64;

        let area = 0.5 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs();
        area
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
