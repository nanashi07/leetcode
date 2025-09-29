// # 1039. Minimum Score Triangulation of Polygon
// https://leetcode.com/problems/minimum-score-triangulation-of-polygon/

struct Solution;

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_score_triangulation_of_polygon::Solution;

    #[test]
    fn test_min_score_triangulation_1() {
        let values = [1, 2, 3].to_vec();
        assert_eq!(6, Solution::min_score_triangulation(values));
    }

    #[test]
    fn test_min_score_triangulation_2() {
        let values = [3, 7, 4, 5].to_vec();
        assert_eq!(144, Solution::min_score_triangulation(values));
    }

    #[test]
    fn test_min_score_triangulation_3() {
        let values = [1, 3, 1, 4, 1, 5].to_vec();
        assert_eq!(13, Solution::min_score_triangulation(values));
    }
}
