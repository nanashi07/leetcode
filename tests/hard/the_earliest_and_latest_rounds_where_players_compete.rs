// # 1900. The Earliest and Latest Rounds Where Players Compete
// https://leetcode.com/problems/the-earliest-and-latest-rounds-where-players-compete/

/*
Problem Description:
In a tournament, there are n players numbered from 1 to n.
They compete in a knockout tournament where in each round,
players are paired up from opposite ends and compete against each other.
Given the positions of two specific players, find the earliest and
latest possible rounds where these two players could meet.

問題描述：
在一個錦標賽中，有n個玩家編號從1到n。
他們參加淘汰賽，每輪中玩家從兩端配對並相互競爭。
給定兩個特定玩家的位置，找出這兩個玩家可能相遇的最早和最晚回合。

Tournament Rules (錦標賽規則):
- Players are arranged in a line from 1 to n
- In each round, player 1 competes with player n, player 2 with player (n-1), etc.
- Winners advance to the next round and are renumbered from 1 to (remaining_players)
- 玩家從1到n排成一線
- 每輪中，玩家1與玩家n競爭，玩家2與玩家(n-1)競爭，等等
- 獲勝者進入下一輪並重新編號從1到(剩餘玩家數)
*/

use std::cmp::{max, min};

struct Solution;

impl Solution {
    // Main function to find earliest and latest rounds where two players compete
    // 主函數：找出兩個玩家競爭的最早和最晚回合
    // https://leetcode.com/problems/the-earliest-and-latest-rounds-where-players-compete/editorial/
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        // Maximum number of players supported (used for memoization array size)
        // 支持的最大玩家數量（用於記憶化陣列大小）
        const MAX_N: usize = 30;

        // Memoization arrays for dynamic programming
        // f[n][first][second] stores the earliest round
        // g[n][first][second] stores the latest round
        // 動態規劃的記憶化陣列
        // f[n][first][second] 儲存最早回合
        // g[n][first][second] 儲存最晚回合
        let mut f = [[[0; MAX_N]; 30]; MAX_N];
        let mut g = [[[0; MAX_N]; 30]; MAX_N];

        // Ensure first <= second for consistency in our DP state
        // 確保 first <= second 以保持DP狀態的一致性
        let mut first = first_player as usize;
        let mut second = second_player as usize;
        if first > second {
            std::mem::swap(&mut first, &mut second);
        }

        // Call the recursive DP function
        // 調用遞歸的DP函數
        let (earliest, latest) = Self::dp(n as usize, first, second, &mut f, &mut g);
        vec![earliest, latest]
    }

    // Dynamic programming function to calculate earliest and latest meeting rounds
    // 動態規劃函數計算最早和最晚相遇回合
    fn dp(
        n: usize,                      // Number of players in current round | 當前回合的玩家數量
        first: usize,                  // Position of first player | 第一個玩家的位置
        second: usize,                 // Position of second player | 第二個玩家的位置
        f: &mut [[[i32; 30]; 30]; 30], // Memoization for earliest | 最早回合的記憶化
        g: &mut [[[i32; 30]; 30]; 30], // Memoization for latest | 最晚回合的記憶化
    ) -> (i32, i32) {
        // Check if we've already computed this state
        // 檢查是否已經計算過這個狀態
        if f[n][first][second] != 0 {
            return (f[n][first][second], g[n][first][second]);
        }

        // Base case: If the two players are paired against each other
        // They will meet in the current round (round 1)
        // 基礎情況：如果兩個玩家相互配對
        // 他們將在當前回合相遇（第1回合）
        if first + second == n + 1 {
            return (1, 1);
        }

        // Symmetric situation handling:
        // If first + second > n + 1, we can transform the problem
        // by flipping the positions (symmetry of the tournament bracket)
        // 對稱情況處理：
        // 如果 first + second > n + 1，我們可以通過翻轉位置來轉換問題
        // （錦標賽括號的對稱性）
        if first + second > n + 1 {
            let (x, y) = Self::dp(n, n + 1 - second, n + 1 - first, f, g);
            f[n][first][second] = x;
            g[n][first][second] = y;
            return (x, y);
        }

        // Initialize variables to track earliest and latest rounds
        // 初始化變數來跟蹤最早和最晚回合
        let mut earliest = i32::MAX;
        let mut latest = i32::MIN;

        // Calculate number of players that will advance to next round
        // 計算將晉級到下一輪的玩家數量
        let n_half = (n + 1) / 2;

        if second <= n_half {
            // Case 1: Both players are on the left half or center
            // They won't be paired against each other this round
            // 情況1：兩個玩家都在左半邊或中心
            // 他們這輪不會相互配對

            // Try all possible ways the players can advance
            // i represents how many players before 'first' advance
            // j represents how many players between 'first' and 'second' advance
            // 嘗試玩家晉級的所有可能方式
            // i 表示在 'first' 之前有多少玩家晉級
            // j 表示在 'first' 和 'second' 之間有多少玩家晉級
            for i in 0..first {
                for j in 0..(second - first) {
                    let (x, y) = Self::dp(n_half, i + 1, i + j + 2, f, g);
                    earliest = min(earliest, x);
                    latest = max(latest, y);
                }
            }
        } else {
            // Case 2: Second player is on the right half
            // More complex calculation needed due to the pairing mechanism
            // 情況2：第二個玩家在右半邊
            // 由於配對機制需要更複雜的計算

            // s_prime is the mirrored position of second player from the right
            // s_prime 是第二個玩家從右邊的鏡像位置
            let s_prime = n + 1 - second;

            // Calculate the offset for positioning in next round
            // 計算下一輪定位的偏移量
            let mid = (n - 2 * s_prime + 1) / 2;

            // Try all possible advancement scenarios
            // 嘗試所有可能的晉級情況
            for i in 0..first {
                for j in 0..(s_prime - first) {
                    let (x, y) = Self::dp(n_half, i + 1, i + j + mid + 2, f, g);
                    earliest = min(earliest, x);
                    latest = max(latest, y);
                }
            }
        }

        // Store results in memoization arrays (add 1 for current round)
        // 將結果儲存在記憶化陣列中（為當前回合加1）
        f[n][first][second] = earliest + 1;
        g[n][first][second] = latest + 1;
        (f[n][first][second], g[n][first][second])
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::the_earliest_and_latest_rounds_where_players_compete::Solution;

    // Test case 1: Players at positions 2 and 4 in a tournament of 11 players
    // 測試用例1：在11個玩家的錦標賽中，位置2和4的玩家
    #[test]
    fn test_earliest_and_latest_1() {
        let n = 11; // Total number of players | 玩家總數
        let first_player = 2; // Position of first player | 第一個玩家的位置
        let second_player = 4; // Position of second player | 第二個玩家的位置

        // Expected result: earliest round 3, latest round 4
        // 預期結果：最早第3輪，最晚第4輪
        assert_eq!(
            vec![3, 4],
            Solution::earliest_and_latest(n, first_player, second_player)
        );
    }

    // Test case 2: Players at positions 1 and 5 in a tournament of 5 players
    // These players are paired against each other in the first round
    // 測試用例2：在5個玩家的錦標賽中，位置1和5的玩家
    // 這些玩家在第一輪就相互配對
    #[test]
    fn test_earliest_and_latest_2() {
        let n = 5; // Total number of players | 玩家總數
        let first_player = 1; // Position of first player | 第一個玩家的位置
        let second_player = 5; // Position of second player | 第二個玩家的位置

        // Expected result: both earliest and latest are round 1
        // because players 1 and 5 are paired in the first round
        // 預期結果：最早和最晚都是第1輪
        // 因為玩家1和5在第一輪就配對了
        assert_eq!(
            vec![1, 1],
            Solution::earliest_and_latest(n, first_player, second_player)
        );
    }
}
