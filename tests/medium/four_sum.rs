// # 18. 4Sum
// https://leetcode.com/problems/4sum/
struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return Vec::new();
        }

        let mut nums = nums;
        nums.sort();

        let mut r: Vec<Vec<i32>> = Vec::new();

        let len = nums.len();
        let mut i1 = 0;

        while i1 < len - 2 {
            // ignore duplicated & overflow
            if (i1 > 0 && nums[i1] == nums[i1 - 1]) || target.checked_sub(nums[i1]).is_none() {
                i1 = i1 + 1;
                continue;
            }
            let mut i2 = i1 + 1;
            while i2 < len - 1 {
                // ignore duplicated & overflow
                if (i2 > i1 + 1 && nums[i2] == nums[i2 - 1])
                    || (target - nums[i1]).checked_sub(nums[i2]).is_none()
                {
                    i2 = i2 + 1;
                    continue;
                }
                let two_sum = Solution::two_sum(&nums[i2 + 1..], target - nums[i1] - nums[i2]);
                for v in two_sum {
                    r.push(vec![nums[i1], nums[i2], v[0], v[1]]);
                }
                i2 = i2 + 1;
            }
            i1 = i1 + 1;
        }

        r
    }
    // first try
    pub fn _four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut r: Vec<Vec<i32>> = Vec::new();

        let mut next = false;
        let mut i1 = 0;
        let mut i4 = nums.len() - 1;

        while i4 - i1 > 2 {
            println!("i1: {}, i4: {}", i1, i4);

            let sum = target - nums[i1] - nums[i4];

            let two_set = Solution::two_sum(&nums[i1 + 1..i4], sum);

            for two in two_set {
                println!("{} {} {} {}", nums[i1], two[0], two[1], nums[i4]);
                r.push(vec![nums[i1], two[0], two[1], nums[i4]]);
            }

            if next {
                i4 = nums.len() - 1;
                i1 = i1 + 1;
                next = false;
            } else {
                i4 = i4 - 1;
                next = i4 - i1 <= 3;
            }
        }

        println!("------------------------------------------------");

        r
    }
    fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut r: Vec<Vec<i32>> = Vec::new();

        let mut i1 = 0;
        let mut i2 = nums.len() - 1;
        while i1 < i2 {
            let sum = nums[i1] + nums[i2];
            if sum == target {
                // println!("[{}, {}]", nums[i2], nums[i3]);
                r.push(vec![nums[i1], nums[i2]]);

                while i1 + 1 < i2 && nums[i1] == nums[i1 + 1] {
                    i1 = i1 + 1;
                }
                while i1 < i2 - 1 && nums[i2 - 1] == nums[i2] {
                    i2 = i2 - 1;
                }
                i1 = i1 + 1;
                i2 = i2 - 1;
            } else if sum - target > 0 {
                i2 = i2 - 1;
            } else {
                i1 = i1 + 1;
            }
        }

        r
    }
}

#[test]
fn test_four_sum() {
    let nums = vec![1, 0, -1, 0, -2, 2];
    let target = 0;
    let result = Solution::four_sum(nums.clone(), target);
    let expected = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
    assert_eq!(expected, result, "nums: {:?}, target: {}", &nums, &target);

    let nums = vec![2, 2, 2, 2, 2];
    let target = 8;
    let result = Solution::four_sum(nums.clone(), target);
    let expected = vec![vec![2, 2, 2, 2]];
    assert_eq!(expected, result, "nums: {:?}, target: {}", &nums, &target);

    let nums = vec![-3, -1, 0, 2, 4, 5];
    let target = 0;
    let result = Solution::four_sum(nums.clone(), target);
    let expected = vec![vec![-3, -1, 0, 4]];
    assert_eq!(expected, result, "nums: {:?}, target: {}", &nums, &target);

    let nums = vec![-2, -1, -1, 1, 1, 2, 2];
    let target = 0;
    let result = Solution::four_sum(nums.clone(), target);
    let expected = vec![vec![-2, -1, 1, 2], vec![-1, -1, 1, 1]];
    assert_eq!(expected, result, "nums: {:?}, target: {}", &nums, &target);

    let nums = vec![-3, -2, -1, 0, 0, 1, 2, 3];
    let target = 0;
    let result = Solution::four_sum(nums.clone(), target);
    let expected = vec![
        vec![-3, -2, 2, 3],
        vec![-3, -1, 1, 3],
        vec![-3, 0, 0, 3],
        vec![-3, 0, 1, 2],
        vec![-2, -1, 0, 3],
        vec![-2, -1, 1, 2],
        vec![-2, 0, 0, 2],
        vec![-1, 0, 0, 1],
    ];
    assert_eq!(expected, result, "nums: {:?}, target: {}", &nums, &target);

    let nums = vec![2, 0, 3, 0, 1, 2, 4];
    let target = 7;
    let result = Solution::four_sum(nums.clone(), target);
    let expected = vec![vec![0, 0, 3, 4], vec![0, 1, 2, 4], vec![0, 2, 2, 3]];
    assert_eq!(expected, result, "nums: {:?}, target: {}", &nums, &target);

    let nums = vec![1000000000, 1000000000, 1000000000, 1000000000];
    let target = -294967296;
    let result = Solution::four_sum(nums.clone(), target);
    let expected: Vec<Vec<i32>> = vec![];
    assert_eq!(expected, result, "nums: {:?}, target: {}", &nums, &target);
}
