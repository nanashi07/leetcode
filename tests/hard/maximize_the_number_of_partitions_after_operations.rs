// # 3003. Maximize the Number of Partitions After Operations
// https://leetcode.com/problems/maximize-the-number-of-partitions-after-operations/

struct Solution;

impl Solution {
    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximize_the_number_of_partitions_after_operations::Solution;

    #[test]
    fn test_max_partitions_after_operations_1() {
        let s = "accca".to_string();
        let k = 2;
        assert_eq!(3, Solution::max_partitions_after_operations(s, k));
    }

    #[test]
    fn test_max_partitions_after_operations_2() {
        let s = "aabaab".to_string();
        let k = 3;
        assert_eq!(1, Solution::max_partitions_after_operations(s, k));
    }

    #[test]
    fn test_max_partitions_after_operations_3() {
        let s = "xxyz".to_string();
        let k = 1;
        assert_eq!(4, Solution::max_partitions_after_operations(s, k));
    }
}
