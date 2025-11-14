// 2536. Increment Submatrices by One
// https://leetcode.com/problems/increment-submatrices-by-one/

struct Solution;

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::increment_submatrices_by_one::Solution;

    #[test]
    fn test_range_add_queries_1() {
        let n = 3;
        let queries = [[1, 1, 2, 2], [0, 0, 1, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let output = [[1, 1, 0], [1, 2, 1], [0, 1, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(output, Solution::range_add_queries(n, queries));
    }

    #[test]
    fn test_range_add_queries_2() {
        let n = 2;
        let queries = [[0, 0, 1, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let output = [[1, 1], [1, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(output, Solution::range_add_queries(n, queries));
    }
}
