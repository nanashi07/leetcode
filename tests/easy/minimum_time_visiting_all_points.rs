// 1266. Minimum Time Visiting All Points
// https://leetcode.com/problems/minimum-time-visiting-all-points/

struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        println!("points: {:?}", &points);

        let mut seconds = 0;

        for i in 0..points.len() - 1 {
            let point = &points[i];
            let x = point[0];
            let y = point[1];

            let next = &points[i + 1];
            let nx = next[0];
            let ny = next[1];
            seconds += (x - nx).abs().max((y - ny).abs())
        }

        seconds
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::minimum_time_visiting_all_points::Solution;

    #[test]
    fn test_min_time_to_visit_all_points_1() {
        let points = [[1, 1], [3, 4], [-1, 0]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(7, Solution::min_time_to_visit_all_points(points));
    }

    #[test]
    fn test_min_time_to_visit_all_points_2() {
        let points = [[3, 2], [-2, 2]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(5, Solution::min_time_to_visit_all_points(points));
    }
}
