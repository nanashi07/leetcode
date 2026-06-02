// 3633. Earliest Finish Time for Land and Water Rides I
// https://leetcode.com/problems/earliest-finish-time-for-land-and-water-rides-i/

struct Solution;

impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        fn build(start: &[i32], duration: &[i32]) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
            let mut rides: Vec<(i32, i32)> = start
                .iter()
                .copied()
                .zip(duration.iter().copied())
                .collect();
            rides.sort_unstable_by_key(|&(s, _)| s);

            let n = rides.len();
            let mut starts = Vec::with_capacity(n);
            let mut pref_min_d = vec![i32::MAX; n + 1];
            let mut suf_min_sd = vec![i32::MAX; n + 1];

            for (i, &(s, d)) in rides.iter().enumerate() {
                starts.push(s);
                pref_min_d[i + 1] = pref_min_d[i].min(d);
            }
            for i in (0..n).rev() {
                let (s, d) = rides[i];
                suf_min_sd[i] = suf_min_sd[i + 1].min(s + d);
            }

            (starts, pref_min_d, suf_min_sd)
        }

        fn best_finish_after(
            arrival: i32,
            starts: &[i32],
            pref_min_d: &[i32],
            suf_min_sd: &[i32],
        ) -> i32 {
            let idx = starts.partition_point(|&s| s <= arrival);
            let mut best = i32::MAX;
            if idx > 0 {
                best = best.min(arrival + pref_min_d[idx]);
            }
            if idx < starts.len() {
                best = best.min(suf_min_sd[idx]);
            }
            best
        }

        let (water_starts, water_pref_min_d, water_suf_min_sd) =
            build(&water_start_time, &water_duration);
        let (land_starts, land_pref_min_d, land_suf_min_sd) = build(&land_start_time, &land_duration);

        let mut ans = i32::MAX;

        for (&s, &d) in land_start_time.iter().zip(land_duration.iter()) {
            let land_end = s + d;
            ans = ans.min(best_finish_after(
                land_end,
                &water_starts,
                &water_pref_min_d,
                &water_suf_min_sd,
            ));
        }

        for (&s, &d) in water_start_time.iter().zip(water_duration.iter()) {
            let water_end = s + d;
            ans = ans.min(best_finish_after(
                water_end,
                &land_starts,
                &land_pref_min_d,
                &land_suf_min_sd,
            ));
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::earliest_finish_time_for_land_and_water_rides_i::Solution;

    #[test]
    fn test_earliest_finish_time_1() {
        let land_start_time = [2, 8].to_vec();
        let land_duration = [4, 1].to_vec();
        let water_start_time = [6].to_vec();
        let water_duration = [3].to_vec();
        assert_eq!(
            9,
            Solution::earliest_finish_time(
                land_start_time,
                land_duration,
                water_start_time,
                water_duration
            )
        );
    }

    #[test]
    fn test_earliest_finish_time_2() {
        let land_start_time = [5].to_vec();
        let land_duration = [3].to_vec();
        let water_start_time = [1].to_vec();
        let water_duration = [10].to_vec();
        assert_eq!(
            14,
            Solution::earliest_finish_time(
                land_start_time,
                land_duration,
                water_start_time,
                water_duration
            )
        );
    }
}
