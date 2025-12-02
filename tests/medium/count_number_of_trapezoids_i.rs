// 3623. Count Number of Trapezoids I
// https://leetcode.com/problems/count-number-of-trapezoids-i/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let mut point_num: HashMap<i32, i32> = HashMap::new();
        let mod_val: i64 = 1000000007;
        let mut ans: i64 = 0;
        let mut sum: i64 = 0;

        for point in points {
            let y = point[1];
            *point_num.entry(y).or_insert(0) += 1;
        }

        for &p_num in point_num.values() {
            let edge = p_num as i64 * (p_num as i64 - 1) / 2;
            ans = (ans + edge * sum) % mod_val;
            sum = (sum + edge) % mod_val;
        }

        ans as i32
    }

    // Time Exceed
    pub fn _count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        // Group points by y-coordinate and count them
        let mut y_counts: HashMap<i32, usize> = HashMap::new();

        for point in &points {
            let y = point[1];
            *y_counts.entry(y).or_insert(0) += 1;
        }

        let mut count: i64 = 0;
        let mut total_pairs: i64 = 0;

        // Convert to sorted vector for processing
        let mut y_data: Vec<(i32, usize)> = y_counts.into_iter().collect();
        y_data.sort_unstable_by_key(|&(y, _)| y);

        // For each y-level, calculate trapezoids with all previous y-levels
        for &(_, n) in &y_data {
            if n >= 2 {
                let pairs_at_current = (n * (n - 1) / 2) as i64;
                // Trapezoids = pairs at current level * total pairs from previous levels
                count += pairs_at_current * total_pairs;
                total_pairs += pairs_at_current;
            }
        }

        count as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_number_of_trapezoids_i::Solution;

    #[test]
    fn test_count_trapezoids_1() {
        let points = [[1, 0], [2, 0], [3, 0], [2, 2], [3, 2]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(3, Solution::count_trapezoids(points));
    }

    #[test]
    fn test_count_trapezoids_2() {
        let points = [[0, 0], [1, 0], [0, 1], [2, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::count_trapezoids(points));
    }
}
