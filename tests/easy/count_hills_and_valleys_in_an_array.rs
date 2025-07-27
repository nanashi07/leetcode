// # 2210. Count Hills and Valleys in an Array
// https://leetcode.com/problems/count-hills-and-valleys-in-an-array/

struct Solution;

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::count_hills_and_valleys_in_an_array::Solution;

    #[test]
    fn test_count_hill_valley_1() {
        let nums = [2, 4, 1, 1, 6, 5].to_vec();
        assert_eq!(3, Solution::count_hill_valley(nums));
    }

    #[test]
    fn test_count_hill_valley_2() {
        let nums = [6, 6, 5, 5, 4, 1].to_vec();
        assert_eq!(0, Solution::count_hill_valley(nums));
    }
}
