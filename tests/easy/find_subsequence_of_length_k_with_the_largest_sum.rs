// 2099. Find Subsequence of Length K With the Largest Sum
// https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/
struct Solution;

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let l = k as usize;
        if nums.len() < l {
            panic!("The length of nums must be at least k");
        }

        let mut result: Vec<i32> = vec![];
        let mut min = i32::MAX;
        let mut p = 0;

        for i in 0..nums.len() {
            let num = nums[i];
            if i < l {
                result.push(num);
                if min > num {
                    min = num;
                    p = i;
                }
            } else {
                if min < num {
                    result.remove(p);
                    result.push(num);
                    min = i32::MAX;

                    for x in 0..result.len() {
                        if result[x] < min {
                            min = result[x];
                            p = x;
                        }
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_subsequence_of_length_k_with_the_largest_sum::Solution;

    #[test]
    fn test_max_subsequence_1() {
        let nums = vec![2, 1, 3, 3];
        let k = 2;
        assert_eq!(vec![3, 3], Solution::max_subsequence(nums, k))
    }
    #[test]
    fn test_max_subsequence_2() {
        let nums = vec![-1, -2, 3, 4];
        let k = 3;
        assert_eq!(vec![-1, 3, 4], Solution::max_subsequence(nums, k))
    }
    #[test]
    fn test_max_subsequence_3() {
        let nums = vec![3, 4, 3, 3];
        let k = 2;
        assert_eq!(vec![3, 4], Solution::max_subsequence(nums, k))
    }
}
