// 3310. Remove Methods From Project
// https://leetcode.com/problems/remove-methods-from-project/

struct Solution;

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let k = k as usize;

        let mut head = vec![usize::MAX; n];
        let mut next = vec![0; invocations.len()];
        let mut to = vec![0; invocations.len()];

        for (i, edge) in invocations.iter().enumerate() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            to[i] = v;
            next[i] = head[u];
            head[u] = i;
        }

        let mut suspicious = vec![false; n];
        let mut stack = vec![k];
        suspicious[k] = true;

        while let Some(u) = stack.pop() {
            let mut idx = head[u];
            while idx != usize::MAX {
                let v = to[idx];
                if !suspicious[v] {
                    suspicious[v] = true;
                    stack.push(v);
                }
                idx = next[idx];
            }
        }

        for edge in &invocations {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            if !suspicious[u] && suspicious[v] {
                return (0..n as i32).collect();
            }
        }

        (0..n as i32).filter(|&i| !suspicious[i as usize]).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::remove_methods_from_project::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_remaining_methods_1() {
        let n = 4;
        let k = 1;
        let invocations = to_vec2d([[1, 2], [0, 1], [3, 2]]);
        let output = [0, 1, 2, 3].to_vec();
        assert_eq!(output, Solution::remaining_methods(n, k, invocations));
    }

    #[test]
    fn test_remaining_methods_2() {
        let n = 5;
        let k = 0;
        let invocations = to_vec2d([[1, 2], [0, 2], [0, 1], [3, 4]]);
        let output = [3, 4].to_vec();
        assert_eq!(output, Solution::remaining_methods(n, k, invocations));
    }

    #[test]
    fn test_remaining_methods_3() {
        let n = 3;
        let k = 2;
        let invocations = to_vec2d([[1, 2], [0, 1], [2, 0]]);
        let output = [0; 0].to_vec();
        assert_eq!(output, Solution::remaining_methods(n, k, invocations));
    }
}
