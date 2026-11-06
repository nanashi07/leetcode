// 2747. Count Zero Request Servers
// https://leetcode.com/problems/count-zero-request-servers/

struct Solution;

impl Solution {
    pub fn count_servers(n: i32, logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        todo!()
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
