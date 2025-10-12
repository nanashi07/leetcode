// # 3539. Find Sum of Array Product of Magical Sequences
// https://leetcode.com/problems/find-sum-of-array-product-of-magical-sequences/

struct Solution;

impl Solution {
    pub fn magical_sum(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_sum_of_array_product_of_magical_sequences::Solution;

    #[test]
    fn test_magical_sum_1() {
        let m = 5;
        let k = 5;
        let nums = [1, 10, 100, 10000, 1000000].to_vec();
        assert_eq!(991600007, Solution::magical_sum(m, k, nums));
    }

    #[test]
    fn test_magical_sum_2() {
        let m = 2;
        let k = 2;
        let nums = [5, 4, 3, 2, 1].to_vec();
        assert_eq!(170, Solution::magical_sum(m, k, nums));
    }

    #[test]
    fn test_magical_sum_3() {
        let m = 1;
        let k = 1;
        let nums = [28].to_vec();
        assert_eq!(28, Solution::magical_sum(m, k, nums));
    }
}
