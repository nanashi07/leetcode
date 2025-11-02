// 16. 3Sum Closest
// https://leetcode.com/problems/3sum-closest/
struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut closest_sum: Option<i32> = None;

        for (i1, v1) in nums[..(nums.len() - 2)].iter().enumerate() {
            let mut i2 = i1 + 1;
            let mut i3 = nums.len() - 1;

            let mut last_sum = v1 + nums[i2] + nums[i3];

            while i2 < i3 {
                let v2 = nums[i2];
                let v3 = nums[i3];

                let sum = v1 + v2 + v3;

                if target == sum {
                    return sum;
                } else if target < sum {
                    i3 = i3 - 1;
                } else {
                    i2 = i2 + 1;
                }

                if let Some(s) = closest_sum {
                    if (target - s).abs() > (target - sum).abs() {
                        closest_sum = Some(sum);
                    }
                } else {
                    closest_sum = Some(sum);
                }

                if sum * last_sum < 0 {
                    break;
                }

                last_sum = sum;
            }

            // ignore same number
            if *v1 == nums[i1 + 1] {
                continue;
            }
        }

        closest_sum.unwrap()
    }
}

#[test]
fn test_three_sum_closest() {
    let nums: Vec<i32> = vec![-1, 2, 1, -4];
    let target = 1;
    let result = Solution::three_sum_closest(nums.clone(), target);
    assert_eq!(2, result, "nums: {:?}, target: {}", &nums, target);

    let nums: Vec<i32> = vec![0, 0, 0];
    let target = 1;
    let result = Solution::three_sum_closest(nums.clone(), target);
    assert_eq!(0, result, "nums: {:?}, target: {}", &nums, target);

    let nums: Vec<i32> = vec![4, 0, 5, -5, 3, 3, 0, -4, -5];
    let target = -2;
    let result = Solution::three_sum_closest(nums.clone(), target);
    assert_eq!(-2, result, "nums: {:?}, target: {}", &nums, target);
}
