// 1320. Minimum Distance to Type a Word Using Two Fingers
// https://leetcode.com/problems/minimum-distance-to-type-a-word-using-two-fingers/

struct Solution;

impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        let chars: Vec<usize> = word.bytes().map(|b| (b - b'A') as usize).collect();
        let n = chars.len();
        if n <= 1 {
            return 0;
        }

        fn dist(a: usize, b: usize) -> i32 {
            let (r1, c1) = ((a / 6) as i32, (a % 6) as i32);
            let (r2, c2) = ((b / 6) as i32, (b % 6) as i32);
            (r1 - r2).abs() + (c1 - c2).abs()
        }

        // dp[j] = min cost after typing up to current char,
        // where one finger is on the last typed char and the other is at position j (26 = not placed)
        let mut dp = vec![i32::MAX; 27];
        dp[26] = 0;

        for i in 1..n {
            let prev = chars[i - 1];
            let curr = chars[i];
            let mut new_dp = vec![i32::MAX; 27];

            for j in 0..27 {
                if dp[j] == i32::MAX {
                    continue;
                }
                // Option 1: move the finger on prev to curr, other finger stays at j
                let cost1 = dp[j] + dist(prev, curr);
                if cost1 < new_dp[j] {
                    new_dp[j] = cost1;
                }
                // Option 2: move the finger at j to curr, other finger stays at prev
                let cost2 = if j == 26 { dp[j] } else { dp[j] + dist(j, curr) };
                if cost2 < new_dp[prev] {
                    new_dp[prev] = cost2;
                }
            }

            dp = new_dp;
        }

        *dp.iter().min().unwrap()
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
