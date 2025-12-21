// 955. Delete Columns to Make Sorted II
// https://leetcode.com/problems/delete-columns-to-make-sorted-ii/

struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::delete_columns_to_make_sorted_ii::Solution;

    #[test]
    fn test_min_deletion_size_1() {
        let strs = ["ca", "bb", "ac"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::min_deletion_size(strs));
    }

    #[test]
    fn test_min_deletion_size_2() {
        let strs = ["xc", "yb", "za"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(0, Solution::min_deletion_size(strs));
    }

    #[test]
    fn test_min_deletion_size_3() {
        let strs = ["zyx", "wvu", "tsr"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(3, Solution::min_deletion_size(strs));
    }
}
