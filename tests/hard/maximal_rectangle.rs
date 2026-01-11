// 85. Maximal Rectangle
// https://leetcode.com/problems/maximal-rectangle/

struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximal_rectangle::Solution;

    #[test]
    fn test_maximal_rectangle_1() {
        let matrix = [
            ["1", "0", "1", "0", "0"],
            ["1", "0", "1", "1", "1"],
            ["1", "1", "1", "1", "1"],
            ["1", "0", "0", "1", "0"],
        ]
        .iter()
        .map(|l| {
            l.iter()
                .map(|s| s.chars().next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
        assert_eq!(6, Solution::maximal_rectangle(matrix));
    }

    #[test]
    fn test_maximal_rectangle_2() {
        let matrix = [["0"]]
            .iter()
            .map(|l| {
                l.iter()
                    .map(|s| s.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert_eq!(0, Solution::maximal_rectangle(matrix));
    }

    #[test]
    fn test_maximal_rectangle_3() {
        let matrix = [["1"]]
            .iter()
            .map(|l| {
                l.iter()
                    .map(|s| s.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::maximal_rectangle(matrix));
    }
}
