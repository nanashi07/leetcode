// 2528. Maximize the Minimum Powered City
// https://leetcode.com/problems/maximize-the-minimum-powered-city/

struct Solution;

impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let n = stations.len();
        let r = r as usize;

        // Calculate initial power for each city using prefix sum
        let mut power = vec![0i64; n];
        let mut prefix = vec![0i64; n + 1];

        for i in 0..n {
            prefix[i + 1] = prefix[i] + stations[i] as i64;
        }

        for i in 0..n {
            let left = i.saturating_sub(r);
            let right = (i + r).min(n - 1) + 1;
            power[i] = prefix[right] - prefix[left];
        }

        // Binary search on the answer
        let mut lo = *power.iter().min().unwrap();
        let mut hi = lo + k as i64;

        while lo < hi {
            let mid = lo + (hi - lo + 1) / 2;
            if Self::can_achieve(mid, &stations, r, k as i64) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }

        lo
    }

    fn can_achieve(target: i64, stations: &[i32], r: usize, k: i64) -> bool {
        let n = stations.len();
        let mut added = vec![0i64; n];
        let mut used = 0i64;

        // Calculate power using sliding window
        let mut window_sum = 0i64;

        // Initialize window for city 0 (original stations only)
        for j in 0..=r.min(n - 1) {
            window_sum += stations[j] as i64;
        }

        // Track cumulative added stations as we slide
        let mut added_in_window = 0i64;

        for i in 0..n {
            let current_power = window_sum + added_in_window;

            if current_power < target {
                let needed = target - current_power;
                used += needed;

                if used > k {
                    return false;
                }

                // Add stations at rightmost position in current window
                let right = (i + r).min(n - 1);
                added[right] += needed;
                added_in_window += needed;
            }

            // Slide window for next city
            if i + 1 < n {
                let next_left = (i + 1).saturating_sub(r);
                let curr_left = i.saturating_sub(r);
                let next_right = ((i + 1) + r).min(n - 1);
                let curr_right = (i + r).min(n - 1);

                // Remove what's going out on the left
                if next_left > curr_left {
                    window_sum -= stations[curr_left] as i64;
                    added_in_window -= added[curr_left];
                }

                // Add what's coming in on the right
                if next_right > curr_right {
                    window_sum += stations[next_right] as i64;
                    added_in_window += added[next_right];
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximize_the_minimum_powered_city::Solution;

    #[test]
    fn test_max_power_1() {
        let stations = [1, 2, 4, 5, 0].to_vec();
        let r = 1;
        let k = 2;
        assert_eq!(5, Solution::max_power(stations, r, k));
    }

    #[test]
    fn test_max_power_2() {
        let stations = [4, 4, 4, 4].to_vec();
        let r = 0;
        let k = 3;
        assert_eq!(4, Solution::max_power(stations, r, k));
    }

    #[test]
    fn test_max_power_3() {
        let stations = [57, 70, 35, 30, 29, 13, 17, 88, 89, 49].to_vec();
        let r = 1;
        let k = 90;
        assert_eq!(138, Solution::max_power(stations, r, k));
    }
}
