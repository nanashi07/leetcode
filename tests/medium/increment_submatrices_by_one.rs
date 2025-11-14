// 2536. Increment Submatrices by One
// https://leetcode.com/problems/increment-submatrices-by-one/

struct Solution;

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut diff = vec![vec![0; n + 1]; n + 1];

        // Apply difference array technique
        for query in queries {
            let (r1, c1, r2, c2) = (
                query[0] as usize,
                query[1] as usize,
                query[2] as usize,
                query[3] as usize,
            );

            diff[r1][c1] += 1;
            diff[r1][c2 + 1] -= 1;
            diff[r2 + 1][c1] -= 1;
            diff[r2 + 1][c2 + 1] += 1;
        }

        // Compute 2D prefix sum to get the result
        let mut result = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                result[i][j] = diff[i][j];
                if i > 0 {
                    result[i][j] += result[i - 1][j];
                }
                if j > 0 {
                    result[i][j] += result[i][j - 1];
                }
                if i > 0 && j > 0 {
                    result[i][j] -= result[i - 1][j - 1];
                }
            }
        }

        result
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
