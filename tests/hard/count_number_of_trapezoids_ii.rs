// 3625. Count Number of Trapezoids II
// https://leetcode.com/problems/count-number-of-trapezoids-ii/

struct Solution;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::count_number_of_trapezoids_ii::Solution;

    #[test]
    fn test_count_trapezoids_1() {
        let points = [[-3, 2], [3, 0], [2, 3], [3, 2], [2, -3]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(2, Solution::count_trapezoids(points));
    }

    #[test]
    fn test_count_trapezoids_2() {
        let points = [[0, 0], [1, 0], [0, 1], [2, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::count_trapezoids(points));
    }
}
