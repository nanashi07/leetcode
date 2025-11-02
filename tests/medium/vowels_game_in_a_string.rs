// 3227. Vowels Game in a String
// https://leetcode.com/problems/vowels-game-in-a-string/

struct Solution;

impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        // Alice wins if there's at least one vowel in the string
        // If there are no vowels, Alice cannot make any move and loses
        // If there are vowels, Alice can always make moves and win

        let vowels = ['a', 'e', 'i', 'o', 'u'];

        s.chars().any(|c| vowels.contains(&c.to_ascii_lowercase()))
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::vowels_game_in_a_string::Solution;

    #[test]
    fn test_does_alice_win_1() {
        let s = "leetcoder".to_owned();
        assert_eq!(true, Solution::does_alice_win(s));
    }

    #[test]
    fn test_does_alice_win_2() {
        let s = "bbcd".to_owned();
        assert_eq!(false, Solution::does_alice_win(s));
    }
}
