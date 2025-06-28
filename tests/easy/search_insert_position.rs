// # 35. Search Insert Position
// https://leetcode.com/problems/search-insert-position/
struct Solution;

impl Solution {
    // ref: https://leetcode.com/problems/search-insert-position/discuss/15080/My-8-line-Java-solution
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len();
        while low < high {
            let mid = low + (high - low) / 2; // low<=mid, mid<high
            if nums[mid] >= target {
                high = mid; // high always decreases (even high-low==1)
            } else {
                low = mid + 1; // low always increases
            }
        }
        return low as i32;
    }

    pub fn _search_insert2(nums: Vec<i32>, target: i32) -> i32 {
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
