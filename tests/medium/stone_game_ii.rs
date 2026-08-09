// 1140. Stone Game II
// https://leetcode.com/problems/stone-game-ii/

struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        if n == 0 {
            return 0;
        }
        let mut suffix_sums = vec![0; n + 1];
        for i in (0..n).rev() {
            suffix_sums[i] = suffix_sums[i + 1] + piles[i];
        }

        let mut dp = vec![vec![0; n + 1]; n];

        for i in (0..n).rev() {
            for m in 1..=n {
                if i + 2 * m >= n {
                    dp[i][m] = suffix_sums[i];
                } else {
                    let mut max_stones = 0;
                    for x in 1..=2 * m {
                        let next_m = m.max(x);
                        let val = suffix_sums[i] - dp[i + x][next_m];
                        max_stones = max_stones.max(val);
                    }
                    dp[i][m] = max_stones;
                }
            }
        }

        dp[0][1]
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::stone_game_ii::Solution;

    #[test]
    fn test_stone_game_ii_1() {
        let piles = [2, 7, 9, 4, 4].to_vec();
        assert_eq!(10, Solution::stone_game_ii(piles));
    }

    #[test]
    fn test_stone_game_ii_2() {
        let piles = [1, 2, 3, 4, 5, 100].to_vec();
        assert_eq!(104, Solution::stone_game_ii(piles));
    }
}
