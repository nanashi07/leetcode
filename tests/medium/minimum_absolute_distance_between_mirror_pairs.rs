// 3761. Minimum Absolute Distance Between Mirror Pairs
// https://leetcode.com/problems/minimum-absolute-distance-between-mirror-pairs/

struct Solution;

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        fn reverse(mut n: i32) -> i32 {
            let mut rev = 0;
            while n > 0 {
                rev = rev * 10 + n % 10;
                n /= 10;
            }
            rev
        }

        // Map from reverse-value to the latest index where it was seen
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut min_dist = i32::MAX;

        for (j, &num) in nums.iter().enumerate() {
            if let Some(&i) = map.get(&num) {
                min_dist = min_dist.min((j - i) as i32);
            }
            map.insert(reverse(num), j);
        }

        if min_dist == i32::MAX { -1 } else { min_dist }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_absolute_distance_between_mirror_pairs::Solution;

    #[test]
    fn test_min_mirror_pair_distance_1() {
        let nums = [12, 21, 45, 33, 54].to_vec();
        assert_eq!(1, Solution::min_mirror_pair_distance(nums));
    }

    #[test]
    fn test_min_mirror_pair_distance_2() {
        let nums = [120, 21].to_vec();
        assert_eq!(1, Solution::min_mirror_pair_distance(nums));
    }

    #[test]
    fn test_min_mirror_pair_distance_3() {
        let nums = [21, 120].to_vec();
        assert_eq!(-1, Solution::min_mirror_pair_distance(nums));
    }
}
