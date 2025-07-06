// # 3333. Find the Original Typed String II
// https://leetcode.com/problems/find-the-original-typed-string-ii/

struct Solution;

impl Solution {
    // https://leetcode.com/problems/find-the-original-typed-string-ii/solutions/6910352/kotlin-rust/
    // https://leetcode.com/problems/find-the-original-typed-string-ii/solutions/5982440/optimized-tabulation/
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        // count each duplicated char block
        let g = word
            .as_bytes()
            .chunk_by(|a, b| a == b)
            .map(|c| c.len() as i64)
            .collect::<Vec<_>>();
        const M: i64 = 1_000_000_007;
        let mut all = 1;
        // calculate all possible combinations
        for &c in &g {
            all = (all * c) % M
        }
        // return directly
        if k as usize <= g.len() {
            return all as i32;
        };
        // how many duplicated chars to base
        let n = k as usize - g.len();
        // remove base chars and count -1
        let g = g
            .iter()
            .filter(|&&c| c > 1)
            .map(|&c| c - 1)
            .collect::<Vec<_>>();

        let mut dp = vec![vec![0; n]; 2];
        dp[0][0] = 1i64;
        let mut ps = vec![0; n + 1];

        // === calculate dp ===
        // duplicated group length
        for i in 0..g.len() {
            // additional chars length
            for kk in 0..n {
                // ???
                ps[kk + 1] = (ps[kk] + dp[i % 2][kk]) % M;
                dp[1 - (i % 2)][kk] = (ps[kk + 1] - ps[(0.max(kk as i64 - g[i])) as usize]) % M;
            }
        }

        let mut bad = 0;
        for i in 0..n {
            bad = (bad + dp[g.len() % 2][i]) % M
        }

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
