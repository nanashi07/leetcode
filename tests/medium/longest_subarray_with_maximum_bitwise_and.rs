// 2419. Longest Subarray With Maximum Bitwise AND
// https://leetcode.com/problems/longest-subarray-with-maximum-bitwise-and/

struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        println!("nums: {:?}", &nums);

        let max_and = *nums.iter().max().unwrap();
        let mut max = 0;
        let mut count = 0;
        for n in &nums {
            if max_and & *n == max_and {
                count += 1;
                max = max.max(count);
            } else {
                count = 0;
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::longest_subarray_with_maximum_bitwise_and::Solution;

    #[test]
    fn test_longest_subarray_1() {
        let nums = [1, 2, 3, 3, 2, 2].to_vec();
        assert_eq!(2, Solution::longest_subarray(nums));
    }

    #[test]
    fn test_longest_subarray_2() {
        let nums = [1, 2, 3, 4].to_vec();
        assert_eq!(1, Solution::longest_subarray(nums));
    }

    #[test]
    fn test_longest_subarray_3() {
        let nums = [
            311155, 311155, 311155, 311155, 311155, 311155, 311155, 311155, 201191, 311155,
        ]
        .to_vec();
        assert_eq!(8, Solution::longest_subarray(nums));
    }
}
