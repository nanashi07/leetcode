// 1653. Minimum Deletions to Make String Balanced
// https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/

struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        // A balanced string has all 'a's before all 'b's
        // We track:
        // - b_count: number of 'b's seen so far
        // - deletions: minimum deletions needed to make the string balanced up to current position

        let mut b_count = 0;
        let mut deletions = 0;

        for ch in s.chars() {
            if ch == 'b' {
                b_count += 1;
            } else {
                // ch == 'a'
                // We have two choices:
                // 1. Delete this 'a' (cost: deletions + 1)
                // 2. Delete all previous 'b's (cost: b_count)
                // We choose the minimum
                deletions = (deletions + 1).min(b_count);
            }
        }

        deletions
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_deletions_to_make_string_balanced::Solution;

    #[test]
    fn test_minimum_deletions_1() {
        let s = "aababbab".to_string();
        assert_eq!(2, Solution::minimum_deletions(s));
    }

    #[test]
    fn test_minimum_deletions_2() {
        let s = "bbaaaaabb".to_string();
        assert_eq!(2, Solution::minimum_deletions(s));
    }
}
