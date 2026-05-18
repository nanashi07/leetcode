// 1345. Jump Game IV
// https://leetcode.com/problems/jump-game-iv/

struct Solution;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        use std::collections::{HashMap, VecDeque};

        let n = arr.len();
        if n <= 1 {
            return 0;
        }

        let mut indices_by_value: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &v) in arr.iter().enumerate() {
            indices_by_value.entry(v).or_default().push(i);
        }

        let mut visited = vec![false; n];
        visited[0] = true;

        let mut queue: VecDeque<(usize, i32)> = VecDeque::new();
        queue.push_back((0, 0));

        while let Some((idx, steps)) = queue.pop_front() {
            if idx == n - 1 {
                return steps;
            }

            if idx + 1 < n && !visited[idx + 1] {
                visited[idx + 1] = true;
                queue.push_back((idx + 1, steps + 1));
            }

            if idx > 0 && !visited[idx - 1] {
                visited[idx - 1] = true;
                queue.push_back((idx - 1, steps + 1));
            }

            if let Some(same_value_indices) = indices_by_value.remove(&arr[idx]) {
                for next in same_value_indices {
                    if !visited[next] {
                        visited[next] = true;
                        queue.push_back((next, steps + 1));
                    }
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::jump_game_iv::Solution;

    #[test]
    fn test_min_jumps_1() {
        let arr = [100, -23, -23, 404, 100, 23, 23, 23, 3, 404].to_vec();
        assert_eq!(3, Solution::min_jumps(arr));
    }

    #[test]
    fn test_min_jumps_2() {
        let arr = [7].to_vec();
        assert_eq!(0, Solution::min_jumps(arr));
    }

    #[test]
    fn test_min_jumps_3() {
        let arr = [7, 6, 9, 6, 9, 6, 9, 7].to_vec();
        assert_eq!(1, Solution::min_jumps(arr));
    }
}
