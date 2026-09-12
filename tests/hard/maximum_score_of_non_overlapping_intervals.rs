// 3414. Maximum Score of Non-overlapping Intervals
// https://leetcode.com/problems/maximum-score-of-non-overlapping-intervals/

struct Solution;

/// Best score of a non-overlapping set plus its indices in ascending order.
#[derive(Clone)]
pub struct Best {
    score: i64,
    picks: Vec<i32>,
}

impl Best {
    pub(crate) fn better(&self, target: &Best) -> bool {
        self.score > target.score || (self.score == target.score && self.picks < target.picks)
    }
}

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();
        // (start, end, weight, index) sorted by start ascending.
        let mut sorted: Vec<(i32, i32, i64, usize)> = intervals
            .iter()
            .enumerate()
            .map(|(i, v)| (v[0], v[1], v[2] as i64, i))
            .collect();
        sorted.sort_unstable();

        // next[i]: first position whose start is past the end of interval i.
        let next: Vec<usize> = (0..n)
            .map(|i| {
                let end = sorted[i].1;
                sorted.partition_point(|s| s.0 <= end)
            })
            .collect();

        // dp[i][k]: best set of at most k intervals among positions i..n.
        let empty = Best {
            score: 0,
            picks: Vec::new(),
        };
        let mut dp = vec![vec![empty; 5]; n + 1];
        for i in (0..n).rev() {
            for k in 1..=4 {
                let sub = &dp[next[i]][k - 1];
                let mut picks = sub.picks.clone();
                let at = picks.partition_point(|&p| p < sorted[i].3 as i32);
                picks.insert(at, sorted[i].3 as i32);
                let take = Best {
                    score: sorted[i].2 + sub.score,
                    picks,
                };
                dp[i][k] = if take.better(&dp[i + 1][k]) {
                    take
                } else {
                    dp[i + 1][k].clone()
                };
            }
        }

        std::mem::take(&mut dp[0][4].picks)
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_score_of_non_overlapping_intervals::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_maximum_weight_1() {
        let intervals = to_vec2d([
            [1, 3, 2],
            [4, 5, 2],
            [1, 5, 5],
            [6, 9, 3],
            [6, 7, 1],
            [8, 9, 1],
        ]);
        assert_eq!([2, 3].to_vec(), Solution::maximum_weight(intervals));
    }

    #[test]
    fn test_maximum_weight_2() {
        let intervals = to_vec2d([
            [5, 8, 1],
            [6, 7, 7],
            [4, 7, 3],
            [9, 10, 6],
            [7, 8, 2],
            [11, 14, 3],
            [3, 5, 5],
        ]);
        assert_eq!([1, 3, 5, 6].to_vec(), Solution::maximum_weight(intervals));
    }
}
