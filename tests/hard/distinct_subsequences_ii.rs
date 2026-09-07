// 940. Distinct Subsequences II
// https://leetcode.com/problems/distinct-subsequences-ii/

struct Solution;

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::distinct_subsequences_ii::Solution;

    #[test]
    fn test_distinct_subseq_ii_1() {
        let s = "abc".to_string();
        assert_eq!(7, Solution::distinct_subseq_ii(s));
    }

    #[test]
    fn test_distinct_subseq_ii_2() {
        let s = "aba".to_string();
        assert_eq!(6, Solution::distinct_subseq_ii(s));
    }

    #[test]
    fn test_distinct_subseq_ii_3() {
        let s = "aaa".to_string();
        assert_eq!(3, Solution::distinct_subseq_ii(s));
    }
}
