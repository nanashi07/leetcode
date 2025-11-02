// 976. Largest Perimeter Triangle
// https://leetcode.com/problems/largest-perimeter-triangle/

struct Solution;

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        println!("nums: {:?}", &nums);

        let len = nums.len();
        let mut nums = nums;
        nums.sort_unstable();

        // Check from the largest possible combinations first
        // We need at least 3 elements, so start from index 2 and go backwards
        for i in (2..len).rev() {
            let a = nums[i - 2]; // smallest of the three
            let b = nums[i - 1]; // middle
            let c = nums[i]; // largest of the three

            // Triangle inequality: sum of two smaller sides > largest side
            if a + b > c {
                return a + b + c; // Return the perimeter
            }
        }

        0 // No valid triangle found
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::largest_perimeter_triangle::Solution;

    #[test]
    fn test_largest_perimeter_1() {
        let nums = [2, 1, 2].to_vec();
        assert_eq!(5, Solution::largest_perimeter(nums));
    }

    #[test]
    fn test_largest_perimeter_2() {
        let nums = [1, 2, 1, 10].to_vec();
        assert_eq!(0, Solution::largest_perimeter(nums));
    }
}
