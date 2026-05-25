// 1871. Jump Game VII
// https://leetcode.com/problems/jump-game-vii/

struct Solution;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let bytes = s.as_bytes();
        let n = bytes.len();

        if bytes[n - 1] != b'0' {
            return false;
        }

        let min_jump = min_jump as usize;
        let max_jump = max_jump as usize;

        let mut reachable = vec![false; n];
        reachable[0] = true;

        // Number of reachable indices in the current sliding window
        // [i - max_jump, i - min_jump].
        let mut window_count = 0i32;

        for i in 1..n {
            if i >= min_jump && reachable[i - min_jump] {
                window_count += 1;
            }

            if i > max_jump && reachable[i - max_jump - 1] {
                window_count -= 1;
            }

            if bytes[i] == b'0' && window_count > 0 {
                reachable[i] = true;
            }
        }

        reachable[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::jump_game_vii::Solution;

    #[test]
    fn test_can_reach_1() {
        let s = "011010".to_string();
        let min_jump = 2;
        let max_jump = 3;
        assert_eq!(true, Solution::can_reach(s, min_jump, max_jump));
    }

    #[test]
    fn test_can_reach_2() {
        let s = "01101110".to_string();
        let min_jump = 2;
        let max_jump = 3;
        assert_eq!(false, Solution::can_reach(s, min_jump, max_jump));
    }
}
