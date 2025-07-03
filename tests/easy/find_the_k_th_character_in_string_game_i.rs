// # 3304. Find the K-th Character in String Game I
// https://leetcode.com/problems/find-the-k-th-character-in-string-game-i/

struct Solution;

impl Solution {
    pub fn kth_character(k: i32) -> char {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_the_k_th_character_in_string_game_i::Solution;

    #[test]
    fn test_kth_character_1() {
        let k = 5;
        assert_eq!('b', Solution::kth_character(k));
    }

    #[test]
    fn test_kth_character_2() {
        let k = 10;
        assert_eq!('c', Solution::kth_character(k));
    }
}
