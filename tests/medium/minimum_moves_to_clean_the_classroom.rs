// 3568. Minimum Moves to Clean the Classroom
// https://leetcode.com/problems/minimum-moves-to-clean-the-classroom/

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        let m = classroom.len();
        let n = classroom[0].len();

        let mut start = (0, 0);
        let mut litters = Vec::new();

        for (r, row) in classroom.iter().enumerate().take(m) {
            for (c, ch) in row.bytes().enumerate() {
                match ch {
                    b'S' => start = (r, c),
                    b'L' => litters.push((r, c)),
                    _ => {}
                }
            }
        }

        let k = litters.len();
        if k == 0 {
            return 0;
        }

        let target_mask = (1 << k) - 1;
        let mut initial_mask = 0;
        for (i, &(r, c)) in litters.iter().enumerate() {
            if (r, c) == start {
                initial_mask |= 1 << i;
            }
        }
        if initial_mask == target_mask {
            return 0;
        }

        // max_energy[r][c][mask]
        let mut max_energy = vec![vec![vec![-1i32; 1 << k]; n]; m];
        let mut q = VecDeque::new();

        max_energy[start.0][start.1][initial_mask] = energy;
        q.push_back((start.0, start.1, initial_mask, energy));

        let mut moves = 0;

        while !q.is_empty() {
            moves += 1;
            let sz = q.len();
            for _ in 0..sz {
                let (r, c, mask, e) = q.pop_front().unwrap();
                if e == 0 {
                    continue;
                }

                let next_e = e - 1;
                for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;

                    if nr < 0 || nr >= m as i32 || nc < 0 || nc >= n as i32 {
                        continue;
                    }

                    let nr = nr as usize;
                    let nc = nc as usize;

                    let ch = classroom[nr].as_bytes()[nc];
                    if ch == b'X' {
                        continue;
                    }

                    let mut next_mask = mask;
                    let mut tile_e = next_e;

                    if ch == b'R' {
                        tile_e = energy;
                    } else if ch == b'L' {
                        if let Some(idx) = litters.iter().position(|&p| p == (nr, nc)) {
                            next_mask |= 1 << idx;
                        }
                    }

                    if next_mask == target_mask {
                        return moves;
                    }

                    if tile_e > max_energy[nr][nc][next_mask] {
                        max_energy[nr][nc][next_mask] = tile_e;
                        q.push_back((nr, nc, next_mask, tile_e));
                    }
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_moves_to_clean_the_classroom::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_min_moves_1() {
        let classroom = to_string_vec(["S.", "XL"]);
        let energy = 2;
        assert_eq!(2, Solution::min_moves(classroom, energy));
    }

    #[test]
    fn test_min_moves_2() {
        let classroom = to_string_vec(["LS", "RL"]);
        let energy = 4;
        assert_eq!(3, Solution::min_moves(classroom, energy));
    }

    #[test]
    fn test_min_moves_3() {
        let classroom = to_string_vec(["L.S", "RXL"]);
        let energy = 3;
        assert_eq!(-1, Solution::min_moves(classroom, energy));
    }
}
