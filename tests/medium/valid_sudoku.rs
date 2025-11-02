// 36. Valid Sudoku
// https://leetcode.com/problems/valid-sudoku/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let len = board.len();

        for x in 0..len {
            let fills = &board[x]
                .iter()
                .filter(|ele| **ele != '.')
                .collect::<Vec<&char>>();
            let uniq: HashSet<_> = fills.iter().cloned().collect();
            if fills.len() != uniq.len() {
                println!("row {} has duplicate", x);
                return false;
            }

            let mut uniq: HashSet<&char> = HashSet::new();
            for y in 0..len {
                let c = &board[y][x];
                if *c != '.' && !uniq.insert(c) {
                    println!("column {} has duplicate", y);
                    return false;
                }
            }
        }

        for ox in 0..(len / 3) {
            for oy in 0..(len / 3) {
                let mut uniq: HashSet<&char> = HashSet::new();
                for x in 0..3 {
                    for y in 0..3 {
                        let c = &board[ox * 3 + x][oy * 3 + y];
                        if *c != '.' && !uniq.insert(c) {
                            println!("block [{},{}] [{},{}] has duplicate {}", ox, oy, x, y, c);
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::valid_sudoku::Solution;

    #[test]
    fn test_is_valid_sudoku_1() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(true, Solution::is_valid_sudoku(board));
    }

    #[test]
    fn test_is_valid_sudoku_2() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(false, Solution::is_valid_sudoku(board));
    }
}
