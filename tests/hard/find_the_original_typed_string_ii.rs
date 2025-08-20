// # 3333. Find the Original Typed String II
// https://leetcode.com/problems/find-the-original-typed-string-ii/
//
// Problem Description / 問題描述:
// Given a string `word` that represents a typed string, and an integer `k`,
// return the number of possible original strings that could have resulted
// in the typed string, where the original string has exactly `k` characters.
//
// 給定一個字符串 `word` 表示打字輸入的結果，以及一個整數 `k`，
// 返回可能產生該打字結果的原始字符串數量，其中原始字符串恰好有 `k` 個字符。
//
// When typing, adjacent identical characters might be typed multiple times
// accidentally, but each character in the original string is typed at least once.
//
// 在打字時，相鄰的相同字符可能會被意外多次輸入，
// 但原始字符串中的每個字符至少會被輸入一次。
//
// Algorithm Explanation / 算法解釋:
// 1. Group consecutive identical characters and count their lengths
//    將連續的相同字符分組並計算其長度
// 2. Calculate total possible combinations without length constraint
//    計算無長度限制時的總可能組合數
// 3. Use dynamic programming to count invalid combinations (length < k)
//    使用動態規劃計算無效組合數（長度 < k）
// 4. Subtract invalid combinations from total
//    從總數中減去無效組合數

struct Solution;

impl Solution {
    // https://leetcode.com/problems/find-the-original-typed-string-ii/solutions/6910352/kotlin-rust/
    // https://leetcode.com/problems/find-the-original-typed-string-ii/solutions/5982440/optimized-tabulation/
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        // Step 1: Group consecutive identical characters
        // 步驟1：將連續的相同字符分組
        // Example: "aabbcc" -> [2, 2, 2] (lengths of each group)
        // 例如："aabbcc" -> [2, 2, 2]（每組的長度）
        let g = word
            .as_bytes()
            .chunk_by(|a, b| a == b)
            .map(|c| c.len() as i64)
            .collect::<Vec<_>>();

        const M: i64 = 1_000_000_007; // Modulo for large numbers / 大數取模
        let mut all = 1;

        // Step 2: Calculate total possible combinations
        // 步驟2：計算總可能組合數
        // For each group of length `c`, we can choose 1 to c characters
        // 對於長度為 `c` 的每組，我們可以選擇 1 到 c 個字符
        for &c in &g {
            all = (all * c) % M
        }

        // Step 3: Early return if k is small enough
        // 步驟3：如果 k 足夠小則提前返回
        // If k <= number of groups, all combinations are valid
        // 如果 k <= 組數，所有組合都是有效的
        if k as usize <= g.len() {
            return all as i32;
        };

        // Step 4: Calculate how many extra characters we need
        // 步驟4：計算需要多少額外字符
        // We need at least one character from each group (g.len() characters)
        // Additional characters needed: k - g.len()
        // 我們需要從每組至少取一個字符（g.len() 個字符）
        // 需要的額外字符數：k - g.len()
        let n = k as usize - g.len();

        // Step 5: Transform groups for DP calculation
        // 步驟5：為動態規劃計算轉換組
        // Remove groups with only 1 character (no extra choices)
        // Subtract 1 from each group length (since we must take 1 character)
        // 移除只有1個字符的組（沒有額外選擇）
        // 從每組長度減1（因為我們必須取1個字符）
        let g = g
            .iter()
            .filter(|&&c| c > 1)
            .map(|&c| c - 1)
            .collect::<Vec<_>>();

        // Step 6: Dynamic Programming setup
        // 步驟6：動態規劃設置
        // dp[i][j] = number of ways to choose j extra characters from first i groups
        // dp[i][j] = 從前 i 組中選擇 j 個額外字符的方法數
        let mut dp = vec![vec![0; n]; 2];
        dp[0][0] = 1i64; // Base case: 0 extra characters from 0 groups
        let mut ps = vec![0; n + 1]; // Prefix sum array / 前綴和數組

        // Step 7: Fill DP table
        // 步驟7：填充動態規劃表
        for i in 0..g.len() {
            // Build prefix sum for current row
            // 為當前行構建前綴和
            for kk in 0..n {
                ps[kk + 1] = (ps[kk] + dp[i % 2][kk]) % M;
                // Calculate number of ways to choose kk extra characters
                // using range sum from prefix array
                // 使用前綴數組的範圍和計算選擇 kk 個額外字符的方法數
                dp[1 - (i % 2)][kk] = (ps[kk + 1] - ps[(0.max(kk as i64 - g[i])) as usize]) % M;
            }
        }

        // Step 8: Count invalid combinations (length < k)
        // 步驟8：計算無效組合數（長度 < k）
        // Sum all ways to choose 0 to n-1 extra characters
        // 計算選擇 0 到 n-1 個額外字符的所有方法數
        let mut bad = 0;
        for i in 0..n {
            bad = (bad + dp[g.len() % 2][i]) % M
        }

        // Step 9: Return valid combinations
        // 步驟9：返回有效組合數
        // Total combinations - Invalid combinations
        // 總組合數 - 無效組合數
        ((all + M - bad) % M) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_the_original_typed_string_ii::Solution;

    #[test]
    fn test_possible_string_count_1() {
        let word = "aabbccdd".to_owned();
        let k = 7;
        assert_eq!(5, Solution::possible_string_count(word, k));
    }

    #[test]
    fn test_possible_string_count_2() {
        let word = "aabbccdd".to_owned();
        let k = 8;
        assert_eq!(1, Solution::possible_string_count(word, k));
    }

    #[test]
    fn test_possible_string_count_3() {
        let word = "aaabbb".to_owned();
        // aaabbb, aabbb, abbb,  | [0,0], [1,0], [2,0]
        // aaabb,  aabb,  abb,   | [0,1], [1,1], [2,1]
        // aaab,   aab,  [ab]    | [0,2], [1,2]m [2,2]
        let k = 3;
        assert_eq!(8, Solution::possible_string_count(word, k));
    }

    #[test]
    fn test_possible_string_count_4() {
        let word = "bbbblllliizzzznnnnna".to_owned();
        // aaabbb, aabbb, abbb,  | [0,0], [1,0], [2,0]
        // aaabb,  aabb,  abb,   | [0,1], [1,1], [2,1]
        // aaab,   aab,  [ab]    | [0,2], [1,2]m [2,2]
        let k = 20;
        assert_eq!(8, Solution::possible_string_count(word, k));
    }
}
