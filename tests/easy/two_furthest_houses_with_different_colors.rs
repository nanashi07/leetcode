// 2078. Two Furthest Houses With Different Colors
// https://leetcode.com/problems/two-furthest-houses-with-different-colors/

struct Solution;

impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::two_furthest_houses_with_different_colors::Solution;

    #[test]
    fn test_max_distance_1() {
        let colors = [1, 1, 1, 6, 1, 1, 1].to_vec();
        assert_eq!(3, Solution::max_distance(colors));
    }

    #[test]
    fn test_max_distance_2() {
        let colors = [1, 8, 3, 8, 3].to_vec();
        assert_eq!(4, Solution::max_distance(colors));
    }

    #[test]
    fn test_max_distance_3() {
        let colors = [0, 1].to_vec();
        assert_eq!(1, Solution::max_distance(colors));
    }
}
