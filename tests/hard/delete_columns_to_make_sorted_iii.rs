// 960. Delete Columns to Make Sorted III
// https://leetcode.com/problems/delete-columns-to-make-sorted-iii/

struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::delete_columns_to_make_sorted_iii::Solution;

    #[test]
    fn test_min_deletion_size_1() {
        let strs = ["babca", "bbazb"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(3, Solution::min_deletion_size(strs));
    }

    #[test]
    fn test_min_deletion_size_2() {
        let strs = ["edcba"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(4, Solution::min_deletion_size(strs));
    }

    #[test]
    fn test_min_deletion_size_3() {
        let strs = ["ghi", "def", "abc"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(0, Solution::min_deletion_size(strs));
    }
}
