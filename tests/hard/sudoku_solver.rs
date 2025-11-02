// 37. Sudoku Solver
// https://leetcode.com/problems/sudoku-solver/

struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::backtrack(board);
    }

    fn backtrack(board: &mut Vec<Vec<char>>) -> bool {
        // Find the first empty cell
        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] == '.' {
                    // Try digits 1-9
                    for digit in '1'..='9' {
                        if Self::is_valid(board, row, col, digit) {
                            // Place the digit
                            board[row][col] = digit;

                            // Recursively solve the rest
                            if Self::backtrack(board) {
                                return true;
                            }

                            // Backtrack if solution not found
                            board[row][col] = '.';
                        }
                    }
                    // No valid digit found for this cell
                    return false;
                }
            }
        }
        // All cells filled successfully
        true
    }

    fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, digit: char) -> bool {
        // Check row
        for c in 0..9 {
            if board[row][c] == digit {
                return false;
            }
        }

        // Check column
        for r in 0..9 {
            if board[r][col] == digit {
                return false;
            }
        }

        // Check 3x3 box
        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;
        for r in box_row..box_row + 3 {
            for c in box_col..box_col + 3 {
                if board[r][c] == digit {
                    return false;
                }
            }
        }

        true
    }

    // failed
    pub fn _solve_sudoku(board: &mut Vec<Vec<char>>) {
        use std::collections::HashSet;

        let h = board.len();
        let w = board[0].len();
        let all: HashSet<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
            .into_iter()
            .collect();
        let mut empties = 0;
        let mut rows: Vec<HashSet<char>> = vec![all.clone(); 9];
        let mut cols: Vec<HashSet<char>> = vec![all.clone(); 9];
        let mut chunks: Vec<Vec<HashSet<char>>> = vec![vec![all.clone(); 3]; 3];

        for y in 0..h {
            for x in 0..w {
                let c = board[y][x];
                if c != '.' {
                    rows[y].remove(&c);
                    cols[x].remove(&c);
                    // println!("{:?} -> {:?} = {}", &(y, x), &(y / 3, x / 3), &c);
                    chunks[y / 3][x / 3].remove(&c);
                    empties += 1;
                }
            }
        }

        while empties > 0 {
            for y in 0..h {
                let ry = &rows[y];
                if ry.is_empty() {
                    continue;
                }
                for x in 0..w {
                    let ry = &rows[y]; // re-borrow
                    let rx = &cols[x];
                    if rx.is_empty() {
                        continue;
                    }

                    let xy = rx.intersection(ry).map(|s| *s).collect::<HashSet<_>>();
                    let matching = if !xy.is_empty() {
                        let rc = &chunks[y / 3][x / 3];
                        let xyc = xy.intersection(&rc).collect::<HashSet<_>>();
                        if xyc.len() == 1 {
                            Some(**xyc.iter().next().unwrap())
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    if let Some(c) = matching {
                        board[y][x] = c;
                        rows[y].remove(&c);
                        cols[x].remove(&c);
                        chunks[y / 3][x / 3].remove(&c);
                        empties -= 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::sudoku_solver::Solution;

    #[test]
    fn test_solve_sudoku_1() {
        let mut board = [
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ]
        .into_iter()
        .map(|l| l.map(|c| c.chars().next().unwrap()).to_vec())
        .collect::<Vec<Vec<char>>>();
        let output = [
            ["5", "3", "4", "6", "7", "8", "9", "1", "2"],
            ["6", "7", "2", "1", "9", "5", "3", "4", "8"],
            ["1", "9", "8", "3", "4", "2", "5", "6", "7"],
            ["8", "5", "9", "7", "6", "1", "4", "2", "3"],
            ["4", "2", "6", "8", "5", "3", "7", "9", "1"],
            ["7", "1", "3", "9", "2", "4", "8", "5", "6"],
            ["9", "6", "1", "5", "3", "7", "2", "8", "4"],
            ["2", "8", "7", "4", "1", "9", "6", "3", "5"],
            ["3", "4", "5", "2", "8", "6", "1", "7", "9"],
        ]
        .into_iter()
        .map(|l| l.map(|c| c.chars().next().unwrap()).to_vec())
        .collect::<Vec<Vec<char>>>();
        Solution::solve_sudoku(&mut board);
        assert_eq!(output, board);
    }
}
