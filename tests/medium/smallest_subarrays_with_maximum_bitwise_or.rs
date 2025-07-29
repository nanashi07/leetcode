// # 2411. Smallest Subarrays With Maximum Bitwise OR
// https://leetcode.com/problems/smallest-subarrays-with-maximum-bitwise-or/

struct Solution;

impl Solution {
    // https://leetcode.com/problems/smallest-subarrays-with-maximum-bitwise-or/editorial/
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut pos = vec![-1; 31];
        let mut ans = vec![0; n];
        for i in (0..n).rev() {
            let mut j = i;
            for bit in 0..31 {
                if (nums[i] & (1 << bit)) == 0 {
                    if pos[bit] != -1 {
                        j = j.max(pos[bit] as usize);
                    }
                } else {
                    pos[bit] = i as i32;
                }
            }
            ans[i] = (j - i + 1) as i32;
        }
        ans
    }

    // Time Limit Exceeded
    pub fn _smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        println!("nums: {:?}", &nums);

        let mut xor = 0;
        for n in &nums {
            xor |= *n;
        }

        println!("xor: {}", xor);

        let mut counter = vec![];
        for i in 0..nums.len() {
            counter.push(1.max(Self::count(&nums, i, 0, 0, xor).0 - i as i32));
        }

        counter
    }

    fn count(
        nums: &Vec<i32>,
        index: usize,
        skip: i32,
        current_xor: i32,
        target_xor: i32,
    ) -> (i32, i32) {
        if index == nums.len() || current_xor == target_xor {
            return (index as i32 - skip, current_xor);
        }

        let (next_count, next_xor) =
            Self::count(nums, index + 1, skip, current_xor | nums[index], target_xor);

        if current_xor < next_xor {
            return (next_count, next_xor);
        }

        let (next_count, next_xor) =
            Self::count(nums, index + 1, skip + 1, current_xor, target_xor);

        if current_xor < next_xor {
            return (next_count, next_xor);
        }

        (index as i32 - skip, current_xor)
    }
}

#[cfg(test)]
mod test {
    use crate::medium::smallest_subarrays_with_maximum_bitwise_or::Solution;

    #[test]
    fn test_smallest_subarrays_1() {
        let nums = [1, 0, 2, 1, 3].to_vec();
        assert_eq!([3, 3, 2, 2, 1].to_vec(), Solution::smallest_subarrays(nums));
    }

    #[test]
    fn test_smallest_subarrays_2() {
        let nums = [1, 2].to_vec();
        assert_eq!([2, 1].to_vec(), Solution::smallest_subarrays(nums));
    }

    #[test]
    fn test_smallest_subarrays_3() {
        let nums = [4, 0, 5, 6, 3, 2].to_vec();
        assert_eq!(
            [4, 3, 2, 2, 1, 1].to_vec(),
            Solution::smallest_subarrays(nums)
        );
    }

    #[test]
    fn test_smallest_subarrays_4() {
        let nums = [0].to_vec();
        assert_eq!([1].to_vec(), Solution::smallest_subarrays(nums));
    }
}
