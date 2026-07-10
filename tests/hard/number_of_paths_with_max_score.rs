// 1301. Number of Paths with Max Score
// https://leetcode.com/problems/number-of-paths-with-max-score/

struct Solution;

impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        let n = board.len();
        let board_bytes: Vec<&[u8]> = board.iter().map(|s| s.as_bytes()).collect();

        let mut dp_curr = vec![-1; n + 1];
        let mut dp_next = vec![-1; n + 1];
        let mut ways_curr = vec![0; n + 1];
        let mut ways_next = vec![0; n + 1];

        let modulo = 1_000_000_007;

        for r in (0..n).rev() {
            dp_curr.fill(-1);
            ways_curr.fill(0);

            for c in (0..n).rev() {
                if r == n - 1 && c == n - 1 {
                    dp_curr[c] = 0;
                    ways_curr[c] = 1;
                    continue;
                }
                if board_bytes[r][c] == b'X' {
                    continue;
                }

                let mut max_val = -1;

                if dp_next[c] > max_val {
                    max_val = dp_next[c];
                }
                if dp_curr[c + 1] > max_val {
                    max_val = dp_curr[c + 1];
                }
                if dp_next[c + 1] > max_val {
                    max_val = dp_next[c + 1];
                }

                if max_val == -1 {
                    continue;
                }

                let mut way = 0;
                if dp_next[c] == max_val {
                    way += ways_next[c];
                    if way >= modulo {
                        way -= modulo;
                    }
                }
                if dp_curr[c + 1] == max_val {
                    way += ways_curr[c + 1];
                    if way >= modulo {
                        way -= modulo;
                    }
                }
                if dp_next[c + 1] == max_val {
                    way += ways_next[c + 1];
                    if way >= modulo {
                        way -= modulo;
                    }
                }

                dp_curr[c] = max_val
                    + if board_bytes[r][c] == b'E' {
                        0
                    } else {
                        (board_bytes[r][c] - b'0') as i32
                    };
                ways_curr[c] = way;
            }

            dp_next.copy_from_slice(&dp_curr);
            ways_next.copy_from_slice(&ways_curr);
        }

        if dp_curr[0] == -1 {
            vec![0, 0]
        } else {
            vec![dp_curr[0], ways_curr[0]]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::number_of_paths_with_max_score::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_paths_with_max_score_1() {
        let board = to_string_vec(["E23", "2X2", "12S"]);
        assert_eq!([7, 1].to_vec(), Solution::paths_with_max_score(board));
    }

    #[test]
    fn test_paths_with_max_score_2() {
        let board = to_string_vec(["E12", "1X1", "21S"]);
        assert_eq!([4, 2].to_vec(), Solution::paths_with_max_score(board));
    }

    #[test]
    fn test_paths_with_max_score_3() {
        let board = to_string_vec(["E11", "XXX", "11S"]);
        assert_eq!([0, 0].to_vec(), Solution::paths_with_max_score(board));
    }
}
