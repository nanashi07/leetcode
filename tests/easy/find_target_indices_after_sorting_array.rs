// 2089. Find Target Indices After Sorting Array
// https://leetcode.com/problems/find-target-indices-after-sorting-array/

struct Solution;

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // In the sorted array every element smaller than `target` sits before all
        // occurrences of `target`, so the target indices form one contiguous run.
        let mut smaller = 0usize;
        let mut equal = 0usize;

        for num in nums {
            if num < target {
                smaller += 1;
            } else if num == target {
                equal += 1;
            }
        }

        (smaller..smaller + equal).map(|i| i as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_target_indices_after_sorting_array::Solution;

    #[test]
    fn test_target_indices_1() {
        let nums = [1, 2, 5, 2, 3].to_vec();
        let target = 2;
        assert_eq!([1, 2].to_vec(), Solution::target_indices(nums, target));
    }

    #[test]
    fn test_target_indices_2() {
        let nums = [1, 2, 5, 2, 3].to_vec();
        let target = 3;
        assert_eq!([3].to_vec(), Solution::target_indices(nums, target));
    }

    #[test]
    fn test_target_indices_3() {
        let nums = [1, 2, 5, 2, 3].to_vec();
        let target = 5;
        assert_eq!([4].to_vec(), Solution::target_indices(nums, target));
    }
}
