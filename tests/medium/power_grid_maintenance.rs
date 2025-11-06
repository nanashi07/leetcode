// 3607. Power Grid Maintenance
// https://leetcode.com/problems/power-grid-maintenance/

use std::collections::HashMap;

struct Solution;

struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    fn new(size: usize) -> Self {
        let mut parent = vec![0; size];
        for i in 0..size {
            parent[i] = i;
        }
        Self { parent }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn join(&mut self, u: usize, v: usize) {
        let root_u = self.find(u);
        let root_v = self.find(v);
        if root_u != root_v {
            self.parent[root_v] = root_u;
        }
    }
}

impl Solution {
    // https://leetcode.com/problems/power-grid-maintenance/editorial/
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let c = c as usize;
        let mut dsu = DSU::new(c + 1);

        for p in connections {
            dsu.join(p[0] as usize, p[1] as usize);
        }

        let mut online = vec![true; c + 1];
        let mut offline_counts = vec![0; c + 1];
        let mut minimum_online_stations: HashMap<usize, i32> = HashMap::new();

        for q in &queries {
            let op = q[0];
            let x = q[1] as usize;
            if op == 2 {
                online[x] = false;
                offline_counts[x] += 1;
            }
        }

        for i in 1..=c {
            let root = dsu.find(i);
            if !minimum_online_stations.contains_key(&root) {
                minimum_online_stations.insert(root, -1);
            }

            let station = *minimum_online_stations.get(&root).unwrap();
            if online[i] {
                if station == -1 || station > i as i32 {
                    minimum_online_stations.insert(root, i as i32);
                }
            }
        }

        let mut ans = Vec::new();

        for i in (0..queries.len()).rev() {
            let op = queries[i][0];
            let x = queries[i][1] as usize;
            let root = dsu.find(x);
            let station = *minimum_online_stations.get(&root).unwrap();

            if op == 1 {
                if online[x] {
                    ans.push(x as i32);
                } else {
                    ans.push(station);
                }
            }

            if op == 2 {
                if offline_counts[x] > 1 {
                    offline_counts[x] -= 1;
                } else {
                    online[x] = true;
                    if station == -1 || station > x as i32 {
                        minimum_online_stations.insert(root, x as i32);
                    }
                }
            }
        }

        ans.reverse();
        ans
    }

    // wrong
    pub fn _process_queries(
        c: i32,
        connections: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        println!(
            "c: {c}, connections: {:?}, queries: {:?}",
            &connections, &queries
        );

        let mut stations = vec![true; 1 + c as usize];
        let mut result = vec![];
        let mut topoligies: Vec<Vec<i32>> = vec![vec![]; 1 + c as usize];

        for connection in connections {
            topoligies[connection[0] as usize].push(connection[1]);
            topoligies[connection[1] as usize].push(connection[0]);
        }

        for i in 0..c as usize {
            topoligies[i].sort_unstable();
        }

        for query in queries {
            let action = query[0];
            match action {
                1 => {
                    if stations[query[1] as usize] {
                        result.push(query[1]);
                    } else {
                        let size = result.len();
                        for x in &topoligies[query[1] as usize] {
                            if stations[*x as usize] {
                                result.push(*x);
                                break;
                            }
                        }
                        if size == result.len() {
                            result.push(-1);
                        }
                    }
                }
                2 => stations[query[1] as usize] = false,
                _ => {}
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::power_grid_maintenance::Solution;

    #[test]
    fn test_process_queries_1() {
        let c = 5;
        let connections = [[1, 2], [2, 3], [3, 4], [4, 5]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let queries = [[1, 3], [2, 1], [1, 1], [2, 2], [1, 2]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let expected = [3, 2, 3].to_vec();
        assert_eq!(expected, Solution::process_queries(c, connections, queries));
    }

    #[test]
    fn test_process_queries_2() {
        let c = 3;
        let connections: Vec<Vec<i32>> = vec![];
        let queries = [[1, 1], [2, 1], [1, 1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let expected = [1, -1].to_vec();
        assert_eq!(expected, Solution::process_queries(c, connections, queries));
    }

    #[test]
    fn test_process_queries_3() {
        let c = 1;
        let connections: Vec<Vec<i32>> = vec![];
        let queries = [[1, 1], [2, 1], [2, 1], [2, 1], [2, 1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let expected = [1].to_vec();
        assert_eq!(expected, Solution::process_queries(c, connections, queries));
    }

    #[test]
    fn test_process_queries_4() {
        let c = 4;
        let connections = [[4, 3], [3, 1], [4, 2], [3, 2], [4, 1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let queries = [
            [2, 3],
            [1, 2],
            [2, 4],
            [1, 1],
            [2, 2],
            [1, 2],
            [1, 2],
            [2, 2],
            [1, 3],
            [2, 3],
            [2, 4],
            [2, 3],
            [2, 4],
            [1, 2],
            [1, 1],
        ]
        .into_iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        let expected = [2, 1, 1, 1, 1, 1, 1].to_vec();
        assert_eq!(expected, Solution::process_queries(c, connections, queries));
    }
}
