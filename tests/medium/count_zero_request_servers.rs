// 2747. Count Zero Request Servers
// https://leetcode.com/problems/count-zero-request-servers/

struct Solution;

impl Solution {
    pub fn count_servers(n: i32, mut logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        // Sort logs by request time and queries by query time (keeping their
        // original indices), then slide a window of logs inside [t - x, t].
        logs.sort_unstable_by_key(|log| log[1]);

        let mut order: Vec<usize> = (0..queries.len()).collect();
        order.sort_unstable_by_key(|&i| queries[i]);

        // Server ids reach up to n (see the examples), so keep a slot for each.
        let mut requests = vec![0usize; n as usize + 1];
        let mut busy = 0usize;
        let mut left = 0usize;
        let mut right = 0usize;
        let mut ans = vec![0; queries.len()];

        for &qi in &order {
            let t = queries[qi];
            while right < logs.len() && logs[right][1] <= t {
                let server = logs[right][0] as usize;
                if requests[server] == 0 {
                    busy += 1;
                }
                requests[server] += 1;
                right += 1;
            }
            while left < right && logs[left][1] < t - x {
                let server = logs[left][0] as usize;
                requests[server] -= 1;
                if requests[server] == 0 {
                    busy -= 1;
                }
                left += 1;
            }
            ans[qi] = n - busy as i32;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_zero_request_servers::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_count_servers_1() {
        let n = 3;
        let logs = to_vec2d([[1, 3], [2, 6], [1, 5]]);
        let x = 5;
        let queries = [10, 11].to_vec();
        assert_eq!(
            [1, 2].to_vec(),
            Solution::count_servers(n, logs, x, queries)
        );
    }

    #[test]
    fn test_count_servers_2() {
        let n = 3;
        let logs = to_vec2d([[2, 4], [2, 1], [1, 2], [3, 1]]);
        let x = 2;
        let queries = [3, 4].to_vec();
        assert_eq!(
            [0, 1].to_vec(),
            Solution::count_servers(n, logs, x, queries)
        );
    }
}
