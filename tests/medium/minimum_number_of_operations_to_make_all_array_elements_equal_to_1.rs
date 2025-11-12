// 2654. Minimum Number of Operations to Make All Array Elements Equal to 1
// https://leetcode.com/problems/minimum-number-of-operations-to-make-all-array-elements-equal-to-1/

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        println!("nums: {:?}", &nums);

        let n = nums.len();

        // Count existing 1s
        let ones = nums.iter().filter(|&&x| x == 1).count();

        // If we already have 1s, we just need to convert the rest
        if ones > 0 {
            return (n - ones) as i32;
        }

        // Find the minimum length subarray with GCD = 1
        let mut min_len = i32::MAX;

        for i in 0..n {
            let mut g = nums[i];
            for j in i..n {
                g = Self::gcd(g, nums[j]);

                // If GCD becomes 1, we found a valid subarray
                if g == 1 {
                    min_len = min_len.min((j - i) as i32);
                    break; // No need to extend further from i
                }
            }
        }

        // If no subarray has GCD = 1, impossible
        if min_len == i32::MAX {
            return -1;
        }

        // Operations needed:
        // - min_len operations to create one 1 from the subarray
        // - (n - 1) operations to convert all other elements to 1
        min_len + (n as i32 - 1)
    }

    fn gcd(n: i32, m: i32) -> i32 {
        if m == 0 {
            n
        } else {
            Self::gcd(m, n % m)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_number_of_operations_to_make_all_array_elements_equal_to_1::Solution;

    #[test]
    fn test_min_operations_1() {
        let nums = [2, 6, 3, 4].to_vec();
        assert_eq!(4, Solution::min_operations(nums));
    }

    #[test]
    fn test_min_operations_2() {
        let nums = [2, 10, 6, 14].to_vec();
        assert_eq!(-1, Solution::min_operations(nums));
    }
}
