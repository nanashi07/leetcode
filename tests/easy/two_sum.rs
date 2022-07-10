// # 1. Two Sum
struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i1, v1) in nums.iter().enumerate() {
            for (i2, v2) in nums[(i1 + 1)..].iter().enumerate() {
                if v1 + v2 == target {
                    return vec![i1 as i32, (i1 + i2 + 1) as i32];
                }
            }
        }
        vec![]
    }
}

#[test]
fn test_two_sum() {
    let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
    assert_eq!(vec![0, 1], result);
    let result = Solution::two_sum(vec![0, 4, 3, 0], 0);
    assert_eq!(vec![0, 3], result);
    let result = Solution::two_sum(vec![3, 2, 4], 6);
    assert_eq!(vec![1, 2], result);
    let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
    assert_eq!(vec![0, 1], result);
}
