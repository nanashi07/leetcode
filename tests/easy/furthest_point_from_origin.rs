// 2833. Furthest Point From Origin
// https://leetcode.com/problems/furthest-point-from-origin/

struct Solution;

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::furthest_point_from_origin::Solution;

    #[test]
    fn test_furthest_distance_from_origin_1() {
        let moves = "L_RL__R".to_string();
        assert_eq!(3, Solution::furthest_distance_from_origin(moves));
    }

    #[test]
    fn test_furthest_distance_from_origin_2() {
        let moves = "_R__LL_".to_string();
        assert_eq!(5, Solution::furthest_distance_from_origin(moves));
    }

    #[test]
    fn test_furthest_distance_from_origin_3() {
        let moves = "_______".to_string();
        assert_eq!(7, Solution::furthest_distance_from_origin(moves));
    }
}
