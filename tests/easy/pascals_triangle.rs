// 118. Pascal's Triangle
// https://leetcode.com/problems/pascals-triangle/

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut rows: Vec<Vec<i32>> = Vec::new();

        for _ in 1..=num_rows {
            if rows.len() == 0 {
                rows.push(vec![1])
            } else {
                let last = &rows[rows.len() - 1];
                let len = last.len();
                let mut row: Vec<i32> = Vec::new();
                for i in 0..=len {
                    let v = if i == 0 {
                        last[i]
                    } else if i == len {
                        last[i - 1]
                    } else {
                        last[i - 1] + last[i]
                    };
                    row.push(v);
                }
                rows.push(row);
            }
        }

        rows
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::pascals_triangle::Solution;

    #[test]
    fn test_generate_1() {
        let num_rows = 5;
        let result = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(result, Solution::generate(num_rows));
    }

    #[test]
    fn test_generate_2() {
        let num_rows = 1;
        let result = vec![vec![1]];
        assert_eq!(result, Solution::generate(num_rows));
    }
}
