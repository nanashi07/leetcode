// 2977. Minimum Cost to Convert String II
// https://leetcode.com/problems/minimum-cost-to-convert-string-ii/

struct Solution;

const INF: i32 = i32::MAX / 2;

struct TrieNode {
    child: [Option<Box<TrieNode>>; 26],
    id: i32,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            child: Default::default(),
            id: -1,
        }
    }

    fn add(node: &mut Box<TrieNode>, word: &str, index: &mut i32) -> i32 {
        let mut current = node;
        for ch in word.chars() {
            let i = (ch as u8 - b'a') as usize;
            if current.child[i].is_none() {
                current.child[i] = Some(Box::new(TrieNode::new()));
            }
            current = current.child[i].as_mut().unwrap();
        }
        if current.id == -1 {
            *index += 1;
            current.id = *index;
        }
        current.id
    }
}

impl Solution {
    // https://leetcode.com/problems/minimum-cost-to-convert-string-ii/editorial/
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        let n = source.len();
        let m = original.len();
        let mut root = Box::new(TrieNode::new());

        let mut p = -1;
        let node_count = m * 2;
        let mut g = vec![vec![INF; node_count]; node_count];
        for i in 0..node_count {
            g[i][i] = 0;
        }

        for i in 0..m {
            let x = TrieNode::add(&mut root, &original[i], &mut p);
            let y = TrieNode::add(&mut root, &changed[i], &mut p);
            g[x as usize][y as usize] = g[x as usize][y as usize].min(cost[i]);
        }

        let size = (p + 1) as usize;
        for k in 0..size {
            for i in 0..size {
                for j in 0..size {
                    g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
                }
            }
        }

        let mut f = vec![-1i64; n];
        let source_chars: Vec<char> = source.chars().collect();
        let target_chars: Vec<char> = target.chars().collect();
        for j in 0..n {
            if j > 0 && f[j - 1] == -1 {
                continue;
            }
            let base = if j == 0 { 0 } else { f[j - 1] };
            if source_chars[j] == target_chars[j] {
                if f[j] == -1 || base < f[j] {
                    f[j] = base;
                }
            }

            let mut u = &root;
            let mut v = &root;
            for i in j..n {
                let u_idx = (source_chars[i] as u8 - b'a') as usize;
                let v_idx = (target_chars[i] as u8 - b'a') as usize;

                u = match u.child[u_idx].as_ref() {
                    Some(node) => node,
                    None => break,
                };
                v = match v.child[v_idx].as_ref() {
                    Some(node) => node,
                    None => break,
                };

                if u.id != -1 && v.id != -1 && g[u.id as usize][v.id as usize] != INF {
                    let new_val = base + g[u.id as usize][v.id as usize] as i64;
                    if f[i] == -1 || new_val < f[i] {
                        f[i] = new_val;
                    }
                }
            }
        }

        f[n - 1]
    }

    // Time exceed
    pub fn _minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        use std::collections::HashMap;

        let n = source.len();
        if n == 0 {
            return 0;
        }

        // Build string to id mapping
        let mut string_to_id: HashMap<&str, usize> = HashMap::new();
        let mut id = 0;

        for s in original.iter().chain(changed.iter()) {
            if !string_to_id.contains_key(s.as_str()) {
                string_to_id.insert(s.as_str(), id);
                id += 1;
            }
        }

        // Find max substring length and collect unique lengths for optimization
        let max_len = original
            .iter()
            .chain(changed.iter())
            .map(|s| s.len())
            .max()
            .unwrap_or(0)
            .min(n);

        // Floyd-Warshall algorithm to compute shortest paths
        const INF: i64 = i64::MAX / 2;
        let mut dist = vec![vec![INF; id]; id];
        for i in 0..id {
            dist[i][i] = 0;
        }

        for i in 0..original.len() {
            let from = string_to_id[original[i].as_str()];
            let to = string_to_id[changed[i].as_str()];
            dist[from][to] = dist[from][to].min(cost[i] as i64);
        }

        for k in 0..id {
            for i in 0..id {
                for j in 0..id {
                    if dist[i][k] != INF && dist[k][j] != INF {
                        dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                    }
                }
            }
        }

        // DP: dp[i] represents minimum cost to convert source[0..i]
        let mut dp = vec![INF; n + 1];
        dp[0] = 0;

        let source_bytes = source.as_bytes();
        let target_bytes = target.as_bytes();

        for i in 0..n {
            if dp[i] == INF {
                continue;
            }

            // Try replacing substrings of different lengths, limited by max_len
            let max_search_len = (n - i).min(max_len);

            for len in 1..=max_search_len {
                let end = i + len;

                // Quick check for single character equality
                if len == 1 && source_bytes[i] == target_bytes[i] {
                    dp[end] = dp[end].min(dp[i]);
                    continue;
                }

                // For small strings, check equality inline
                if len <= 10 {
                    let mut equal = true;
                    for j in 0..len {
                        if source_bytes[i + j] != target_bytes[i + j] {
                            equal = false;
                            break;
                        }
                    }
                    if equal {
                        dp[end] = dp[end].min(dp[i]);
                        continue;
                    }
                }

                let source_sub = &source[i..end];
                let target_sub = &target[i..end];

                if len > 10 && source_sub == target_sub {
                    // No conversion needed for longer strings
                    dp[end] = dp[end].min(dp[i]);
                    continue;
                }

                // Look up both substrings at once
                if let Some(&from_id) = string_to_id.get(source_sub) {
                    if let Some(&to_id) = string_to_id.get(target_sub) {
                        let conversion_cost = dist[from_id][to_id];
                        if conversion_cost < INF {
                            dp[end] = dp[end].min(dp[i] + conversion_cost);
                        }
                    }
                }
            }
        }

        if dp[n] >= INF {
            -1
        } else {
            dp[n]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_cost_to_convert_string_ii::Solution;

    #[test]
    fn test_minimum_cost_1() {
        let source = "abcd".to_string();
        let target = "acbe".to_string();
        let original = ["a", "b", "c", "c", "e", "d"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let changed = ["b", "c", "b", "e", "b", "e"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let cost = [2, 5, 5, 1, 2, 20].to_vec();
        assert_eq!(
            28,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }

    #[test]
    fn test_minimum_cost_2() {
        let source = "abcdefgh".to_string();
        let target = "acdeeghh".to_string();
        let original = ["bcd", "fgh", "thh"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let changed = ["cde", "thh", "ghh"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let cost = [1, 3, 5].to_vec();
        assert_eq!(
            9,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }

    #[test]
    fn test_minimum_cost_3() {
        let source = "abcdefgh".to_string();
        let target = "addddddd".to_string();
        let original = ["bcd", "defgh"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let changed = ["ddd", "ddddd"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let cost = [100, 1578].to_vec();
        assert_eq!(
            -1,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }

    #[test]
    fn test_minimum_cost_4() {
        let source = "cfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrr".to_string();
        let target = "fjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegx".to_string();
        let original = ["cfojngjcrr","cfoj","ngjcrrc","fojngj","crr","cfojngjcrr","cfojng","jc","rrcfojngjc","rrcf","ojngjcrrcf","ojng","jc","rrcf","ojn","gjcrrcfoj","ngjcrrcfoj","ngjc","rrcfojng","jcrrcf","ojngjcrrcf","ojngjcrrcf","ojng","jcrrcfojng","jcrrcfo","jngjcrrcf","ojngjc","rrcf","ojng","jcrr","cfojngj","crrc","fojngj","cr","rcfoj","ngjc","rrcfojngj","cr","rcfojngj","crrcfojn","gjcrrcfo","jngj","crrcfojn","gjcrrcfo","jn","gj","crrcfo","jngjcrrcfo","jn","gj","crrcfojng","jcrrcfoj","ngjc","rrcf","ojngjcr","rc","fojngj","cr","rcfojn","gjcrrcf","ojn","gjcr","rcfojngjc","rrcfojngjc","rr","cfo","jngjcrrcfo","jngjcrrcf","oj","ngjcr","rcfojngjcr","rcfojngj","crr","cfojngj","crrc","fo","jngjcrrcfo","jngjc","rrc","fojngjcrr","cfoj","ngjcrrcf","ojngjc","rrcfojngj","crr","cfojn","gjcrrcfo","jngjcrrcf","ojngjcrr","cfojn","gjcrrcfo","jngj","crrcfojng","jcrrcf","oj","ngjcr","rc","fojngjcrr","cfojn","cfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrrcfojngjcrr"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let changed = ["fjglppnegx","fjgl","ppnegxf","jglppn","egx","fjglppnegx","fjglpp","ne","gxfjglppne","gxfj","glppnegxfj","glpp","ne","gxfj","glp","pnegxfjgl","ppnegxfjgl","ppne","gxfjglpp","negxfj","glppnegxfj","glppnegxfj","glpp","negxfjglpp","negxfjg","lppnegxfj","glppne","gxfj","glpp","negx","fjglppn","egxf","jglppn","eg","xfjgl","ppne","gxfjglppn","eg","xfjglppn","egxfjglp","pnegxfjg","lppn","egxfjglp","pnegxfjg","lp","pn","egxfjg","lppnegxfjg","lp","pn","egxfjglpp","negxfjgl","ppne","gxfj","glppneg","xf","jglppn","eg","xfjglp","pnegxfj","glp","pneg","xfjglppne","gxfjglppne","gx","fjg","lppnegxfjg","lppnegxfj","gl","ppneg","xfjglppneg","xfjglppn","egx","fjglppn","egxf","jg","lppnegxfjg","lppne","gxf","jglppnegx","fjgl","ppnegxfj","glppne","gxfjglppn","egx","fjglp","pnegxfjg","lppnegxfj","glppnegx","fjglp","pnegxfjg","lppn","egxfjglpp","negxfj","gl","ppneg","xf","jglppnegx","fjglp","fjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegxfjglppnegx"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let cost = [
            343, 846, 540, 434, 660, 256, 759, 710, 106, 763, 707, 124, 713, 846, 37, 225, 599,
            447, 961, 222, 385, 377, 329, 193, 232, 877, 258, 482, 338, 746, 591, 280, 29, 895,
            252, 685, 301, 861, 543, 975, 630, 308, 850, 911, 620, 925, 26, 189, 274, 886, 658,
            485, 961, 769, 868, 973, 955, 876, 414, 648, 292, 605, 966, 2, 291, 235, 824, 715, 72,
            452, 744, 974, 166, 767, 327, 265, 436, 873, 91, 5, 252, 520, 216, 55, 528, 956, 201,
            42, 873, 152, 111, 629, 902, 470, 382, 424, 767, 393, 193, 92495,
        ]
        .to_vec();
        assert_eq!(
            783,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }

    #[test]
    fn test_minimum_cost_5() {
        let source = "hdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygc".to_string();
        let target = "qmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnh".to_string();
        let original = ["hd","yg","ch","dy","gch","dy","gc","hdy","gc","hd","yg","ch","dyg","ch","dy","gc","hdy","gc","hd","yg","ch","dy","gch","dy","gc","hdyg","ch","dy","gc","hdyg","ch","dy","gch","dyg","chd","yg","chd","yg","chdy","gc","hdy","gc","hdyg","chd","yg","ch","dyg","ch","dy","gch","dy","gch","dy","gch","dygc","hdyg","chdy","gc","hd","yg","chd","yg","ch","dy","gch","dy","gc","hdy","gc","hd","yg","ch","dy","gc","hd","ygc","hd","yg","ch","dyg","ch","dy","gch","dyg","ch","dy","gch","dy","gc","hd","ygch","dy","gc","hd","yg","chdy","gc","hd","ygch","hdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygchdygc"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let changed = ["qm","mn","hq","mm","nhq","mm","nh","qmm","nh","qm","mn","hq","mmn","hq","mm","nh","qmm","nh","qm","mn","hq","mm","nhq","mm","nh","qmmn","hq","mm","nh","qmmn","hq","mm","nhq","mmn","hqm","mn","hqm","mn","hqmm","nh","qmm","nh","qmmn","hqm","mn","hq","mmn","hq","mm","nhq","mm","nhq","mm","nhq","mmnh","qmmn","hqmm","nh","qm","mn","hqm","mn","hq","mm","nhq","mm","nh","qmm","nh","qm","mn","hq","mm","nh","qm","mnh","qm","mn","hq","mmn","hq","mm","nhq","mmn","hq","mm","nhq","mm","nh","qm","mnhq","mm","nh","qm","mn","hqmm","nh","qm","mnhq","qmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnhqmmnh"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let cost = [
            687, 83, 248, 546, 932, 286, 497, 884, 380, 854, 21, 770, 538, 344, 69, 150, 641, 169,
            484, 734, 411, 603, 601, 348, 232, 308, 515, 295, 637, 566, 70, 159, 866, 457, 504,
            593, 401, 766, 707, 567, 330, 483, 667, 220, 430, 927, 940, 655, 876, 920, 418, 139,
            436, 869, 631, 365, 799, 927, 418, 158, 506, 534, 358, 59, 264, 847, 661, 572, 748,
            239, 584, 177, 620, 117, 301, 34, 4, 981, 918, 462, 997, 70, 132, 625, 142, 80, 281,
            70, 929, 809, 51, 60, 333, 151, 927, 978, 577, 873, 397, 70416,
        ]
        .to_vec();
        assert_eq!(
            7600,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }

    #[test]
    fn test_minimum_cost_6() {
        let source = "hdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuq".to_string();
        let target = "orjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcj".to_string();
        let original = ["hd","rm","zjj","yuq","hdrmz","jjyu","qhdrmz","jjy","uqh","drm","zjjy","uqh","drm","zjjy","uqh","drmzjj","yuqh","drmz","jjyuqh","drmzj","jyu","qh","drmz","jj","yuqh","drm","zj","jy","uq","hdrm","zj","jy","uqh","drmz","jj","yuqhd","rmzj","jy","uq","hdrmz","jjyu","qhd","rm","zjj","yuqhd","rmzj","jyuq","hdrmz","jjyuqh","drmzj","jyuq","hd","rmzj","jyu","qhdr","mzjjyu","qhdr","mzjjy","uq","hdrmz","jjyuqh","drm","zjj","yuqhd","rm","zjj","yu","qhd","rm","zj","jyu","qhdrmz","jjyuqh","drmzj","jy","uq","hdrmzj","jyuq","hdrmz","jjy","uqhd","rmzjjy","uqh","drmz","jj","yuqh","dr","mzjjy","uqhd","rm","zjj","yuq","hd","rmzj","jyuqhd","rmzjj","yu","qhdrm","zjjy","hdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuqhdrmzjjyuq"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let changed = ["or","ju","iek","hcj","orjui","ekhc","jorjui","ekh","cjo","rju","iekh","cjo","rju","iekh","cjo","rjuiek","hcjo","rjui","ekhcjo","rjuie","khc","jo","rjui","ek","hcjo","rju","ie","kh","cj","orju","ie","kh","cjo","rjui","ek","hcjor","juie","kh","cj","orjui","ekhc","jor","ju","iek","hcjor","juie","khcj","orjui","ekhcjo","rjuie","khcj","or","juie","khc","jorj","uiekhc","jorj","uiekh","cj","orjui","ekhcjo","rju","iek","hcjor","ju","iek","hc","jor","ju","ie","khc","jorjui","ekhcjo","rjuie","kh","cj","orjuie","khcj","orjui","ekh","cjor","juiekh","cjo","rjui","ek","hcjo","rj","uiekh","cjor","ju","iek","hcj","or","juie","khcjor","juiek","hc","jorju","iekh","orjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcjorjuiekhcj"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let cost = [
            996, 474, 714, 65, 84, 242, 327, 526, 65, 979, 254, 537, 885, 150, 896, 673, 980, 318,
            813, 482, 470, 784, 285, 400, 131, 633, 492, 322, 342, 235, 330, 515, 584, 637, 880,
            609, 639, 686, 167, 501, 460, 40, 762, 612, 367, 694, 267, 949, 253, 378, 572, 284,
            880, 365, 755, 128, 328, 988, 756, 163, 949, 38, 617, 96, 493, 550, 357, 398, 706, 401,
            333, 780, 411, 152, 401, 377, 147, 875, 594, 157, 869, 93, 398, 694, 296, 681, 63, 503,
            813, 753, 216, 459, 656, 77, 293, 253, 765, 963, 299, 1457,
        ]
        .to_vec();
        assert_eq!(
            1457,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }
}
