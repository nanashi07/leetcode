// 2833. Furthest Point From Origin
// https://leetcode.com/problems/furthest-point-from-origin/

struct Solution;

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let l = moves.chars().filter(|c| *c == 'L').count();
        let r = moves.chars().filter(|c| *c == 'R').count();
        let any = moves.chars().filter(|c| *c == '_').count();

        let d = if l < r { any + r - l } else { any + l - r };

        d as i32
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
