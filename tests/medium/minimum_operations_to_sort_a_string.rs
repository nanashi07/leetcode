// 3863. Minimum Operations to Sort a String
// https://leetcode.com/problems/minimum-operations-to-sort-a-string/

struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let b = s.into_bytes();
        let n = b.len();

        if b.windows(2).all(|w| w[0] <= w[1]) {
            return 0;
        }
        // With two characters every proper substring has length 1, so sorting one changes nothing.
        if n == 2 {
            return -1;
        }

        let mut sorted = b.clone();
        sorted.sort_unstable();

        // One operation is enough iff the window of misplaced positions is a proper substring:
        // outside that window `s` already equals the sorted string, so sorting the window itself
        // (same multiset, hence same letters in the same order) finishes the job.
        let prefix = b.iter().zip(&sorted).take_while(|(a, c)| a == c).count();
        if prefix > 0 {
            return 1;
        }
        let suffix = b
            .iter()
            .rev()
            .zip(sorted.iter().rev())
            .take_while(|(a, c)| a == c)
            .count();
        if suffix > 0 {
            return 1;
        }

        // Both ends are misplaced: s[0] is above the minimum and s[n - 1] is below the maximum.
        // A sort can only fix an end when the needed letter lies strictly inside the string, so
        // when the unique minimum sits at the very end and the unique maximum at the very front,
        // no first move can make progress on either end and three operations are required.
        let min = sorted[0];
        let max = sorted[n - 1];
        if b[..n - 1].iter().all(|&c| c != min) && b[1..].iter().all(|&c| c != max) {
            3
        } else {
            // Otherwise one operation brings the minimum to the front (or the maximum to the end),
            // leaving a proper window for the second one.
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_operations_to_sort_a_string::Solution;

    #[test]
    fn test_min_operations_1() {
        let s = "dog".to_string();
        assert_eq!(1, Solution::min_operations(s));
    }

    #[test]
    fn test_min_operations_2() {
        let s = "card".to_string();
        assert_eq!(2, Solution::min_operations(s));
    }

    #[test]
    fn test_min_operations_3() {
        let s = "gf".to_string();
        assert_eq!(-1, Solution::min_operations(s));
    }
}
