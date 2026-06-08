// 2161. Partition Array According to Given Pivot
// https://leetcode.com/problems/partition-array-according-to-given-pivot/

struct Solution;

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        let mut pivot_count = 0;

        for &num in &nums {
            if num < pivot {
                result.push(num);
            } else if num == pivot {
                pivot_count += 1;
            }
        }

        result.extend(std::iter::repeat_n(pivot, pivot_count));

        for num in nums {
            if num > pivot {
                result.push(num);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::partition_array_according_to_given_pivot::Solution;

    #[test]
    fn test_pivot_array_1() {
        let nums = [9, 12, 5, 10, 14, 3, 10].to_vec();
        let pivot = 10;
        assert_eq!(
            [9, 5, 3, 10, 10, 12, 14].to_vec(),
            Solution::pivot_array(nums, pivot)
        );
    }

    #[test]
    fn test_pivot_array_2() {
        let nums = [-3, 4, 3, 2].to_vec();
        let pivot = 2;
        assert_eq!([-3, 2, 4, 3].to_vec(), Solution::pivot_array(nums, pivot));
    }
}
