// # 3487. Maximum Unique Subarray Sum After Deletion
// https://leetcode.com/problems/maximum-unique-subarray-sum-after-deletion/

struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        println!("nums: {:?}", &nums);
        let mut nums = nums;
        nums.sort_unstable();
        let mut sum = 0;
        let mut prev = 0;
        let mut minus = i32::MIN;
        for n in nums {
            if n > 0 {
                if n != prev {
                    sum += n;
                    prev = n;
                }
            } else {
                minus = minus.max(n);
            }
        }

        if sum > 0 {
            sum
        } else {
            minus
        }
    }
}

#[cfg(test)]
mod test {
    use crate::easy::maximum_unique_subarray_sum_after_deletion::Solution;

    #[test]
    fn test_max_sum_1() {
        let nums = [1, 2, 3, 4, 5].to_vec();
        assert_eq!(15, Solution::max_sum(nums));
    }

    #[test]
    fn test_max_sum_2() {
        let nums = [1, 1, 0, 1, 1].to_vec();
        assert_eq!(1, Solution::max_sum(nums));
    }

    #[test]
    fn test_max_sum_3() {
        let nums = [1, 2, -1, -2, 1, 0, -1].to_vec();
        assert_eq!(3, Solution::max_sum(nums));
    }

    #[test]
    fn test_max_sum_4() {
        let nums = [-100].to_vec();
        assert_eq!(-100, Solution::max_sum(nums));
    }
}
