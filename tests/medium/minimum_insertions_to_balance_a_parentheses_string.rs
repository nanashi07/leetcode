// 1541. Minimum Insertions to Balance a Parentheses String
// https://leetcode.com/problems/minimum-insertions-to-balance-a-parentheses-string/

struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        // Greedy single pass. `need_close` is how many ')' are still required to
        // balance every '(' seen so far, knowing that one '(' consumes two ')'.
        let mut insertions = 0;
        let mut need_close = 0;

        for byte in s.bytes() {
            if byte == b'(' {
                need_close += 2;
                // An odd requirement means a lone ')' is still pending from an earlier
                // '('; inserting that ')' now keeps every pair contiguous.
                if (need_close & 1) == 1 {
                    need_close -= 1;
                    insertions += 1;
                }
            } else {
                need_close -= 1;
                if need_close < 0 {
                    // This ')' has no '(' in front of it, so insert one '('. It still
                    // needs a single ')' because the current char covers the other half.
                    insertions += 1;
                    need_close = 1;
                }
            }
        }

        insertions + need_close
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_insertions_to_balance_a_parentheses_string::Solution;

    #[test]
    fn test_min_insertions_1() {
        let s = "(()))".to_string();
        assert_eq!(1, Solution::min_insertions(s));
    }

    #[test]
    fn test_min_insertions_2() {
        let s = "())".to_string();
        assert_eq!(0, Solution::min_insertions(s));
    }

    #[test]
    fn test_min_insertions_3() {
        let s = "))())(".to_string();
        assert_eq!(3, Solution::min_insertions(s));
    }
}
