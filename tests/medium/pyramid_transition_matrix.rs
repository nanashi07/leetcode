// 756. Pyramid Transition Matrix
// https://leetcode.com/problems/pyramid-transition-matrix/

struct Solution;

impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        use std::collections::HashMap;

        // Build a mapping from (char1, char2) -> Vec<char3>
        // where char1 and char2 are adjacent blocks, and char3 can be placed on top
        let mut map: HashMap<(char, char), Vec<char>> = HashMap::new();

        for s in allowed.iter() {
            let chars: Vec<char> = s.chars().collect();
            if chars.len() == 3 {
                let key = (chars[0], chars[1]);
                map.entry(key).or_insert_with(Vec::new).push(chars[2]);
            }
        }

        Self::backtrack(bottom, &map)
    }

    fn backtrack(
        current: String,
        map: &std::collections::HashMap<(char, char), Vec<char>>,
    ) -> bool {
        // Base case: if we reach a single block, we've built the pyramid
        if current.len() == 1 {
            return true;
        }

        // Try to build the next level
        Self::build_next_level(&current, 0, String::new(), map)
    }

    fn build_next_level(
        current: &str,
        pos: usize,
        next_level: String,
        map: &std::collections::HashMap<(char, char), Vec<char>>,
    ) -> bool {
        // If we've processed all adjacent pairs, move to the next level
        if pos == current.len() - 1 {
            return Self::backtrack(next_level, map);
        }

        let chars: Vec<char> = current.chars().collect();
        let key = (chars[pos], chars[pos + 1]);

        // Try all possible blocks that can be placed on top of this pair
        if let Some(options) = map.get(&key) {
            for &c in options {
                let mut new_next = next_level.clone();
                new_next.push(c);
                if Self::build_next_level(current, pos + 1, new_next, map) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::pyramid_transition_matrix::Solution;

    #[test]
    fn test_pyramid_transition_1() {
        let bottom = "BCD".to_string();
        let allowed = ["BCC", "CDE", "CEA", "FFF"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(true, Solution::pyramid_transition(bottom, allowed));
    }

    #[test]
    fn test_pyramid_transition_2() {
        let bottom = "AAAA".to_string();
        let allowed = ["AAB", "AAC", "BCD", "BBE", "DEF"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(false, Solution::pyramid_transition(bottom, allowed));
    }
}
