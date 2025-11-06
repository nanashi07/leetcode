// 3607. Power Grid Maintenance
// https://leetcode.com/problems/power-grid-maintenance/

struct Solution;

impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::power_grid_maintenance::Solution;

    #[test]
    fn test_process_queries_1() {
        let c = 5;
        let connections = [[1, 2], [2, 3], [3, 4], [4, 5]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let queries = [[1, 3], [2, 1], [1, 1], [2, 2], [1, 2]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let expected = [3, 2, 3].to_vec();
        assert_eq!(expected, Solution::process_queries(c, connections, queries));
    }

    #[test]
    fn test_process_queries_2() {
        let c = 3;
        let connections: Vec<Vec<i32>> = vec![];
        let queries = [[1, 1], [2, 1], [1, 1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let expected = [1, -1].to_vec();
        assert_eq!(expected, Solution::process_queries(c, connections, queries));
    }
}
