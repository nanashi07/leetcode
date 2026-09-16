// 1621. Number of Sets of K Non-Overlapping Line Segments
// https://leetcode.com/problems/number-of-sets-of-k-non-overlapping-line-segments/

struct Solution;

impl Solution {
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_sets_of_k_non_overlapping_line_segments::Solution;

    #[test]
    fn test_number_of_sets_1() {
        let n = 4;
        let k = 2;
        assert_eq!(5, Solution::number_of_sets(n, k));
    }

    #[test]
    fn test_number_of_sets_2() {
        let n = 3;
        let k = 1;
        assert_eq!(3, Solution::number_of_sets(n, k));
    }

    #[test]
    fn test_number_of_sets_3() {
        let n = 30;
        let k = 7;
        assert_eq!(796297179, Solution::number_of_sets(n, k));
    }
}
