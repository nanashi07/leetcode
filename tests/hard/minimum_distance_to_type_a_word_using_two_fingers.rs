// 1320. Minimum Distance to Type a Word Using Two Fingers
// https://leetcode.com/problems/minimum-distance-to-type-a-word-using-two-fingers/

struct Solution;

impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_distance_to_type_a_word_using_two_fingers::Solution;

    #[test]
    fn test_minimum_distance_1() {
        let word = "CAKE".to_string();
        assert_eq!(3, Solution::minimum_distance(word));
    }

    #[test]
    fn test_minimum_distance_2() {
        let word = "HAPPY".to_string();
        assert_eq!(6, Solution::minimum_distance(word));
    }
}
