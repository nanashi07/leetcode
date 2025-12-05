// 3432. Count Partitions with Even Sum Difference
// https://leetcode.com/problems/count-partitions-with-even-sum-difference/

struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        println!("nums: {:?}", &nums);

        let sum = nums.iter().sum::<i32>();
        let mut l = 0;
        let mut c = 0;

        for i in 0..nums.len() - 1 {
            let n = nums[i];
            l = l + n;
            let r = sum - l;
            if (r - l).abs() % 2 == 0 {
                c += 1;
            }
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::count_partitions_with_even_sum_difference::Solution;

    #[test]
    fn test_count_partitions_1() {
        let nums = [10, 10, 3, 7, 6].to_vec();
        assert_eq!(4, Solution::count_partitions(nums));
    }

    #[test]
    fn test_count_partitions_2() {
        let nums = [1, 2, 2].to_vec();
        assert_eq!(0, Solution::count_partitions(nums));
    }

    #[test]
    fn test_count_partitions_3() {
        let nums = [2, 4, 6, 8].to_vec();
        assert_eq!(3, Solution::count_partitions(nums));
    }
}
