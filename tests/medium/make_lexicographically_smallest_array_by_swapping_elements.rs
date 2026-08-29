// 2948. Make Lexicographically Smallest Array by Swapping Elements
// https://leetcode.com/problems/make-lexicographically-smallest-array-by-swapping-elements/

struct Solution;

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let n = nums.len();
        // sort indices by value
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_unstable_by_key(|&i| nums[i]);

        let mut result = vec![0i32; n];
        let mut g_start = 0usize;
        while g_start < n {
            // extend group while consecutive sorted values are within limit
            let mut g_end = g_start + 1;
            while g_end < n && nums[order[g_end]] - nums[order[g_end - 1]] <= limit {
                g_end += 1;
            }
            // collect original indices in this group, sort them, assign sorted values
            let group = &order[g_start..g_end];
            let mut positions: Vec<usize> = group.to_vec();
            positions.sort_unstable();
            for (rank, &pos) in positions.iter().enumerate() {
                result[pos] = nums[group[rank]];
            }
            g_start = g_end;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::make_lexicographically_smallest_array_by_swapping_elements::Solution;

    #[test]
    fn test_lexicographically_smallest_array_1() {
        let nums = [1, 5, 3, 9, 8].to_vec();
        let limit = 2;
        assert_eq!(
            [1, 3, 5, 8, 9].to_vec(),
            Solution::lexicographically_smallest_array(nums, limit)
        );
    }

    #[test]
    fn test_lexicographically_smallest_array_2() {
        let nums = [1, 7, 6, 18, 2, 1].to_vec();
        let limit = 3;
        assert_eq!(
            [1, 6, 7, 18, 1, 2].to_vec(),
            Solution::lexicographically_smallest_array(nums, limit)
        );
    }

    #[test]
    fn test_lexicographically_smallest_array_3() {
        let nums = [1, 7, 28, 19, 10].to_vec();
        let limit = 3;
        assert_eq!(
            [1, 7, 28, 19, 10].to_vec(),
            Solution::lexicographically_smallest_array(nums, limit)
        );
    }
}
