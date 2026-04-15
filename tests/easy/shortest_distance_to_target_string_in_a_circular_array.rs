// 2515. Shortest Distance to Target String in a Circular Array
// https://leetcode.com/problems/shortest-distance-to-target-string-in-a-circular-array/

struct Solution;

impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::shortest_distance_to_target_string_in_a_circular_array::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_closest_target_1() {
        let words = to_string_vec(["hello", "i", "am", "leetcode", "hello"]);
        let target = "hello".to_string();
        let start_index = 1;
        assert_eq!(1, Solution::closest_target(words, target, start_index));
    }

    #[test]
    fn test_closest_target_2() {
        let words = to_string_vec(["a", "b", "leetcode"]);
        let target = "leetcode".to_string();
        let start_index = 0;
        assert_eq!(1, Solution::closest_target(words, target, start_index));
    }

    #[test]
    fn test_closest_target_3() {
        let words = to_string_vec(["i", "eat", "leetcode"]);
        let target = "ate".to_string();
        let start_index = 0;
        assert_eq!(-1, Solution::closest_target(words, target, start_index));
    }
}
