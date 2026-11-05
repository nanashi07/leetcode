// 1385. Find the Distance Value Between Two Arrays
// https://leetcode.com/problems/find-the-distance-value-between-two-arrays/

struct Solution;

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, mut arr2: Vec<i32>, d: i32) -> i32 {
        // Sort arr2 once, then for every x in arr1 the nearest value of arr2 is one of
        // the two neighbours of x's binary-search insertion point.
        arr2.sort_unstable();

        let mut count = 0;
        for &x in &arr1 {
            let i = arr2.partition_point(|&y| y < x);
            let left = if i > 0 {
                (x - arr2[i - 1]).abs()
            } else {
                i32::MAX
            };
            let right = if i < arr2.len() {
                (arr2[i] - x).abs()
            } else {
                i32::MAX
            };
            if left.min(right) > d {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_the_distance_value_between_two_arrays::Solution;

    #[test]
    fn test_find_the_distance_value_1() {
        let arr1 = [4, 5, 8].to_vec();
        let arr2 = [10, 9, 1, 8].to_vec();
        let d = 2;
        assert_eq!(2, Solution::find_the_distance_value(arr1, arr2, d));
    }

    #[test]
    fn test_find_the_distance_value_2() {
        let arr1 = [1, 4, 2, 3].to_vec();
        let arr2 = [-4, -3, 6, 10, 20, 30].to_vec();
        let d = 3;
        assert_eq!(2, Solution::find_the_distance_value(arr1, arr2, d));
    }

    #[test]
    fn test_find_the_distance_value_3() {
        let arr1 = [2, 1, 100, 3].to_vec();
        let arr2 = [-5, -2, 10, -3, 7].to_vec();
        let d = 6;
        assert_eq!(1, Solution::find_the_distance_value(arr1, arr2, d));
    }
}
