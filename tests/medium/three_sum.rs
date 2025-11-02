// 15. 3Sum
// https://leetcode.com/problems/3sum/
struct Solution;

// sort nums
// take v1 from first
// take v3 from last
// check available v2

impl Solution {
    // from discuss
    pub fn _three_sum_from_discussion(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut r: Vec<Vec<i32>> = Vec::new();

        for i in 0..(nums.len() - 2) {
            if i == 0 || (i > 0 && nums[i] != nums[i - 1]) {
                let mut i2 = i + 1;
                let mut i3 = nums.len() - 1;
                let sum = 0 - nums[i];
                while i2 < i3 {
                    if nums[i2] + nums[i3] == sum {
                        r.push(vec![nums[i], nums[i2], nums[i3]]);

                        while i2 < i3 && nums[i2] == nums[i2 + 1] {
                            i2 = i2 + 1
                        }
                        while i2 < i3 && nums[i3] == nums[i3 - 1] {
                            i3 = i3 - 1
                        }
                        i2 = i2 + 1;
                        i3 = i3 - 1;
                    } else if nums[i2] + nums[i3] < sum {
                        i2 = i2 + 1;
                    } else {
                        i3 = i3 - 1
                    }
                }
            }
        }

        r
    }
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut r = Vec::new();

        let mut nums: Vec<i32> = nums;
        nums.sort();

        for (i1, v1) in nums[..(nums.len() - 2)].iter().enumerate() {
            let mut i2 = i1 + 1; // min
            let mut i3 = nums.len() - 1; // max

            // ignore same number
            if i1 > 0 && nums[i1] == nums[i1 - 1] {
                continue;
            }

            while i2 < i3 {
                let v2 = nums[i2];
                let v3 = nums[i3];

                if v1 + v2 + v3 == 0 {
                    r.push(vec![*v1, v2, v3]);

                    // ignore same number
                    while i2 < i3 && nums[i2] == nums[i2 + 1] {
                        i2 = i2 + 1
                    }
                    while i2 < i3 && nums[i3] == nums[i3 - 1] {
                        i3 = i3 - 1
                    }

                    i2 = i2 + 1;
                    i3 = i3 - 1;
                } else if v1 + v2 + v3 > 0 {
                    // move i3 then v3 is smaller
                    i3 = i3 - 1;
                } else {
                    // move i2 then v2 is bigger
                    i2 = i2 + 1;
                }
            }
        }

        r
    }
}

#[test]
fn test_three_sum() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let result = Solution::three_sum(nums.clone());
    assert_eq!(
        vec![vec![-1, -1, 2], vec![-1, 0, 1]],
        result,
        "nums = {:?}",
        &nums
    );

    let nums = vec![0, 1, 1];
    let result = Solution::three_sum(nums.clone());
    let expected: Vec<Vec<i32>> = vec![];
    assert_eq!(expected, result, "nums = {:?}", &nums);

    let nums = vec![0, 0, 0];
    let result = Solution::three_sum(nums.clone());
    assert_eq!(vec![vec![0, 0, 0]], result, "nums = {:?}", &nums);

    let nums = read_case_01();
    let _result = Solution::three_sum(nums.clone());
    // assert_eq!(vec![vec![0, 0, 0]], result);
}

fn read_case_01() -> Vec<i32> {
    let path = "testcase//medium/three_sum/testcase01.txt";
    let content = std::fs::read_to_string(path).unwrap();
    content
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}
