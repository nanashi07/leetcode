// 944. Delete Columns to Make Sorted
// https://leetcode.com/problems/delete-columns-to-make-sorted/

struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        println!("strs: {:?}", &strs);

        let mut del = 0;
        let size = strs[0].len();

        for j in 0..size {
            let mut last_char = ' ';
            for i in 0..strs.len() {
                let str = &strs[i][..];
                if let Some(curr_char) = str.chars().nth(j) {
                    if curr_char < last_char {
                        del += 1;
                        break;
                    }
                    last_char = curr_char;
                }
            }
        }

        del
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::delete_columns_to_make_sorted::Solution;

    #[test]
    fn test_min_deletion_size_1() {
        let strs = ["cba", "daf", "ghi"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::min_deletion_size(strs));
    }

    #[test]
    fn test_min_deletion_size_2() {
        let strs = ["a", "b"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
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
