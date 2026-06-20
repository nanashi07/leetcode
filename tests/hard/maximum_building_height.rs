// 1840. Maximum Building Height
// https://leetcode.com/problems/maximum-building-height/

struct Solution;

impl Solution {
    pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_building_height::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_max_building_1() {
        let n = 5;
        let restrictions = to_vec2d([[2, 1], [4, 1]]);
        assert_eq!(2, Solution::max_building(n, restrictions));
    }

    #[test]
    fn test_max_building_2() {
        let n = 6;
        let restrictions = vec![];
        assert_eq!(5, Solution::max_building(n, restrictions));
    }

    #[test]
    fn test_max_building_3() {
        let n = 10;
        let restrictions = to_vec2d([[5, 3], [2, 5], [7, 4], [10, 3]]);
        assert_eq!(5, Solution::max_building(n, restrictions));
    }
}
