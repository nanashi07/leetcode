// 1689. Partitioning Into Minimum Number Of Deci-Binary Numbers
// https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/

struct Solution;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::partitioning_into_minimum_number_of_deci_binary_numbers::Solution;

    #[test]
    fn test_min_partitions_1() {
        let n = "32".to_string();
        assert_eq!(3, Solution::min_partitions(n));
    }

    #[test]
    fn test_min_partitions_2() {
        let n = "82734".to_string();
        assert_eq!(8, Solution::min_partitions(n));
    }

    #[test]
    fn test_min_partitions_3() {
        let n = "27346209830709182346".to_string();
        assert_eq!(9, Solution::min_partitions(n));
    }
}
