// # 35. Search Insert Position
// https://leetcode.com/problems/search-insert-position/
struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut i = nums.len() / 2;

        let mut start = 0;
        let mut end = nums.len() - 1;
        while start < end {
            if let Some(value) = nums.get(i) {
                if value == &target {
                    return i as i32;
                } else if value < &target {
                    start = i;
                } else {
                    end = i;
                }
                i = (start + end) as usize / 2;

                if start + 1 == end {
                    let n = nums.get(start).unwrap();
                    let m = nums.get(end).unwrap();

                    if &target == n {
                        return start as i32;
                    }
                    if &target == m {
                        return end as i32;
                    }

                    if &target < n {
                        end = start;
                    } else if &target > m {
                        start = end;
                    } else {
                        return end as i32;
                    }
                }
            }
        }

        let n = nums.get(start).unwrap();
        if &target < n {
            i = start;
        } else if &target > n {
            i = end + 1;
        }

        return i as i32;
    }
}

#[test]
pub fn test_search_insert() {
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    let result = Solution::search_insert(nums, target);
    assert_eq!(2, result);

    let nums = vec![1, 3, 5, 6];
    let target = 2;
    let result = Solution::search_insert(nums, target);
    assert_eq!(1, result);

    let nums = vec![1, 3, 5, 6];
    let target = 7;
    let result = Solution::search_insert(nums, target);
    assert_eq!(4, result);

    let nums = vec![1];
    let target = 0;
    let result = Solution::search_insert(nums, target);
    assert_eq!(0, result);

    let nums = vec![1, 3];
    let target = 1;
    let result = Solution::search_insert(nums, target);
    assert_eq!(0, result);

    let nums = vec![1, 3, 5];
    let target = 5;
    let result = Solution::search_insert(nums, target);
    assert_eq!(2, result);
}
