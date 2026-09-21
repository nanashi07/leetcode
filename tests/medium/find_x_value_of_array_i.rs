// 3524. Find X Value of Array I
// https://leetcode.com/problems/find-x-value-of-array-i/

struct Solution;

impl Solution {
    // Count subarrays (remaining segments after removing a prefix + suffix) by product mod k.
    // `end[r]` = number of subarrays ending at the previous index with product % k == r. O(n * k).
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let ku = k as usize;
        let mut ans = vec![0i64; ku];
        let mut end = vec![0i64; ku];
        let mut next = vec![0i64; ku];

        for num in nums {
            let v = (num % k) as usize;
            next.fill(0);
            for r in 0..ku {
                next[(r * v) % ku] += end[r];
            }
            next[v] += 1; // subarray starting at the current element
            for r in 0..ku {
                ans[r] += next[r];
            }
            std::mem::swap(&mut end, &mut next);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_x_value_of_array_i::Solution;

    #[test]
    fn test_result_array_1() {
        let nums = [1, 2, 3, 4, 5].to_vec();
        let k = 3;
        assert_eq!([9, 2, 4].to_vec(), Solution::result_array(nums, k));
    }

    #[test]
    fn test_result_array_2() {
        let nums = [1, 2, 4, 8, 16, 32].to_vec();
        let k = 4;
        assert_eq!([18, 1, 2, 0].to_vec(), Solution::result_array(nums, k));
    }

    #[test]
    fn test_result_array_3() {
        let nums = [1, 1, 2, 1, 1].to_vec();
        let k = 2;
        assert_eq!([9, 6].to_vec(), Solution::result_array(nums, k));
    }
}
