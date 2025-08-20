// # 2210. Count Hills and Valleys in an Array
// https://leetcode.com/problems/count-hills-and-valleys-in-an-array/

struct Solution;

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        println!("nums: {:?}", &nums);

        let mut count = 0;
        let mut last = nums[0];
        let mut i = 1;
        while i < nums.len() - 1 {
            let n = nums[i];

            // find next not equals
            let mut mv: i32 = 0;
            let mut next = n;
            while n == next && (1 + i + mv as usize) < nums.len() {
                mv += 1;
                next = nums[i + mv as usize];
            }

            if last > n && next > n {
                println!("{} {} {}", last, n, next);
                count += 1;
                last = n;
            }
            if last < n && next < n {
                println!("{} {} {}", last, n, next);
                count += 1;
                last = n;
            }

            i += 1;
        }

        count
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

    #[test]
    fn test_count_hill_valley_3() {
        let nums = [8, 2, 5, 7, 7, 2, 10, 3, 6, 2].to_vec();
        assert_eq!(6, Solution::count_hill_valley(nums));
    }

    #[test]
    fn test_count_hill_valley_4() {
        let nums = [
            57, 57, 57, 57, 57, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90,
            85, 85, 85, 86, 86, 86,
        ]
        .to_vec();
        assert_eq!(2, Solution::count_hill_valley(nums));
    }
}
