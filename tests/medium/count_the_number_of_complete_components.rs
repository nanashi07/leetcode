// 2685. Count the Number of Complete Components
// https://leetcode.com/problems/count-the-number-of-complete-components/

struct Solution;

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }

        let mut visited = vec![false; n];
        let mut complete_count = 0;

        for i in 0..n {
            if !visited[i] {
                let mut component = Vec::new();
                let mut stack = vec![i];
                visited[i] = true;

                while let Some(u) = stack.pop() {
                    component.push(u);
                    for &v in &adj[u] {
                        if !visited[v] {
                            visited[v] = true;
                            stack.push(v);
                        }
                    }
                }

                let target_deg = component.len() - 1;
                let mut is_complete = true;
                for &u in &component {
                    if adj[u].len() != target_deg {
                        is_complete = false;
                        break;
                    }
                }

                if is_complete {
                    complete_count += 1;
                }
            }
        }

        complete_count as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_the_number_of_complete_components::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_count_complete_components_1() {
        let n = 6;
        let edges = to_vec2d([[0, 1], [0, 2], [1, 2], [3, 4]]);
        assert_eq!(3, Solution::count_complete_components(n, edges));
    }

    #[test]
    fn test_count_complete_components_2() {
        let n = 6;
        let edges = to_vec2d([[0, 1], [0, 2], [1, 2], [3, 4], [3, 5]]);
        assert_eq!(1, Solution::count_complete_components(n, edges));
    }
}
