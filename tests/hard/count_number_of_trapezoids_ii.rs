// 3625. Count Number of Trapezoids II
// https://leetcode.com/problems/count-number-of-trapezoids-ii/

use std::collections::HashMap;

struct Solution;

impl Solution {
    // https://leetcode.com/problems/count-number-of-trapezoids-ii/editorial/
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let inf = 1e9 as i32;
        let mut slope_to_intercept: HashMap<String, Vec<String>> = HashMap::new();
        let mut mid_to_slope: HashMap<i64, Vec<String>> = HashMap::new();
        let mut ans = 0;

        for i in 0..n {
            let x1 = points[i][0];
            let y1 = points[i][1];
            for j in i + 1..n {
                let x2 = points[j][0];
                let y2 = points[j][1];
                let dx = x1 - x2;
                let dy = y1 - y2;

                let (k, b) = if x2 == x1 {
                    (inf.to_string(), x1.to_string())
                } else {
                    let mut k_val = (y2 - y1) as f64 / (x2 - x1) as f64;
                    let mut b_val =
                        (y1 as i64 * dx as i64 - x1 as i64 * dy as i64) as f64 / dx as f64;
                    if k_val == -0.0 {
                        k_val = 0.0;
                    }
                    if b_val == -0.0 {
                        b_val = 0.0;
                    }
                    (format!("{:.10}", k_val), format!("{:.10}", b_val))
                };

                let mid = (x1 + x2) as i64 * 10000 + (y1 + y2) as i64;
                slope_to_intercept
                    .entry(k.clone())
                    .or_insert(Vec::new())
                    .push(b.clone());
                mid_to_slope.entry(mid).or_insert(Vec::new()).push(k);
            }
        }

        for sti in slope_to_intercept.values() {
            if sti.len() == 1 {
                continue;
            }
            let mut cnt: HashMap<&String, i32> = HashMap::new();
            for b_val in sti {
                *cnt.entry(b_val).or_insert(0) += 1;
            }
            let mut total_sum = 0;
            for &count in cnt.values() {
                ans += total_sum * count;
                total_sum += count;
            }
        }

        for mts in mid_to_slope.values() {
            if mts.len() == 1 {
                continue;
            }

            let mut cnt: HashMap<&String, i32> = HashMap::new();
            for k_val in mts {
                *cnt.entry(k_val).or_insert(0) += 1;
            }

            let mut total_sum = 0;
            for &count in cnt.values() {
                ans -= total_sum * count;
                total_sum += count;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::count_number_of_trapezoids_ii::Solution;

    #[test]
    fn test_count_trapezoids_1() {
        let points = [[-3, 2], [3, 0], [2, 3], [3, 2], [2, -3]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(2, Solution::count_trapezoids(points));
    }

    #[test]
    fn test_count_trapezoids_2() {
        let points = [[0, 0], [1, 0], [0, 1], [2, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::count_trapezoids(points));
    }

    #[test]
    fn test_count_trapezoids_3() {
        let points = [[82, 7], [82, -9], [82, -52], [82, 78]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(0, Solution::count_trapezoids(points));
    }

    #[test]
    fn test_count_trapezoids_4() {
        let points = [
            [71, -89],
            [-75, -89],
            [-9, 11],
            [-24, -89],
            [-51, -89],
            [-77, -89],
            [42, 11],
        ]
        .iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        assert_eq!(10, Solution::count_trapezoids(points));
    }
}
