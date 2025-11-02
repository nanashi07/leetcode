// 3021. Alice and Bob Playing Flower Game
// https://leetcode.com/problems/alice-and-bob-playing-flower-game/

struct Solution;

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        println!("n: {n}, m: {m}");

        // Alice wins when the total number of flowers is odd
        // This happens when one player picks odd flowers and the other picks even flowers

        // Count odd and even numbers from 1 to n
        let odd_n = (n + 1) / 2;
        let even_n = n / 2;

        // Count odd and even numbers from 1 to m
        let odd_m = (m + 1) / 2;
        let even_m = m / 2;

        // Alice wins when: (odd_n * even_m) + (even_n * odd_m)
        let result = (odd_n as i64 * even_m as i64) + (even_n as i64 * odd_m as i64);

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::alice_and_bob_playing_flower_game::Solution;

    #[test]
    fn test_flower_game_1() {
        let n = 3;
        let m = 2;
        assert_eq!(3, Solution::flower_game(n, m));
    }

    #[test]
    fn test_flower_game_2() {
        let n = 1;
        let m = 1;
        assert_eq!(0, Solution::flower_game(n, m));
    }
}
