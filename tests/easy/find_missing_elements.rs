// 3731. Find Missing Elements
// https://leetcode.com/problems/find-missing-elements/

struct Solution;

impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut out = vec![];
        for (i, &n) in nums.iter().enumerate() {
            if i > 0 {
                if n - nums[i - 1] > 1 {
                    for j in nums[i - 1] + 1..n {
                        out.push(j);
                    }
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_missing_elements::Solution;

    #[test]
    fn test_find_missing_elements_1() {
        let nums = [1, 4, 2, 5].to_vec();
        assert_eq!([3].to_vec(), Solution::find_missing_elements(nums));
    }

    #[test]
    fn test_find_missing_elements_2() {
        let nums = [7, 8, 6, 9].to_vec();
        assert_eq!([0; 0].to_vec(), Solution::find_missing_elements(nums));
    }

    #[test]
    fn test_find_missing_elements_3() {
        let nums = [5, 1].to_vec();
        assert_eq!([2, 3, 4].to_vec(), Solution::find_missing_elements(nums));
    }
}
