// 955. Delete Columns to Make Sorted II
// https://leetcode.com/problems/delete-columns-to-make-sorted-ii/

struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        if strs.is_empty() || strs[0].is_empty() {
            return 0;
        }

        let n = strs.len();
        let m = strs[0].len();
        let strs_bytes: Vec<Vec<u8>> = strs.iter().map(|s| s.as_bytes().to_vec()).collect();

        // Track which rows are already sorted (lexicographically less than the next row)
        let mut sorted = vec![false; n - 1];
        let mut deletions = 0;

        for col in 0..m {
            // Check if deleting this column would break the sorted property
            let mut would_break = false;

            for row in 0..n - 1 {
                if !sorted[row] && strs_bytes[row][col] > strs_bytes[row + 1][col] {
                    // This column makes row[i] > row[i+1], must delete
                    would_break = true;
                    break;
                }
            }

            if would_break {
                deletions += 1;
            } else {
                // Keep this column, update sorted status
                for row in 0..n - 1 {
                    if !sorted[row] && strs_bytes[row][col] < strs_bytes[row + 1][col] {
                        sorted[row] = true;
                    }
                }
            }
        }

        deletions
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::delete_columns_to_make_sorted_ii::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_min_deletion_size_1() {
        let strs = to_string_vec(["ca", "bb", "ac"]);
        assert_eq!(1, Solution::min_deletion_size(strs));
    }

    #[test]
    fn test_min_deletion_size_2() {
        let strs = to_string_vec(["xc", "yb", "za"]);
        assert_eq!(0, Solution::min_deletion_size(strs));
    }

    #[test]
    fn test_min_deletion_size_3() {
        let strs = to_string_vec(["zyx", "wvu", "tsr"]);
        assert_eq!(3, Solution::min_deletion_size(strs));
    }
}
