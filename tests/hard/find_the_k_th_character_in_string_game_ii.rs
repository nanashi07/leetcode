// # 3307. Find the K-th Character in String Game II
// https://leetcode.com/problems/find-the-k-th-character-in-string-game-ii/

struct Solution;

impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_the_k_th_character_in_string_game_ii::Solution;

    #[test]
    fn test_kth_character_1() {
        let k = 5;
        let operations = [0, 0, 0].to_vec();
        assert_eq!('a', Solution::kth_character(k, operations));
    }

    #[test]
    fn test_kth_character_2() {
        let k = 10;
        let operations = [0, 1, 0, 1].to_vec();
        assert_eq!('b', Solution::kth_character(k, operations));
    }
}
