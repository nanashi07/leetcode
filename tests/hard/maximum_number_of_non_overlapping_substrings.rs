// 1520. Maximum Number of Non-Overlapping Substrings
// https://leetcode.com/problems/maximum-number-of-non-overlapping-substrings/

struct Solution;

impl Solution {
    // O(26 * n): for each letter take the minimal closed block starting at its first
    // occurrence, drop blocks polluted by an earlier-appearing letter, then greedily
    // select non-overlapping blocks by earliest end.
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let b = s.as_bytes();
        let mut first = [usize::MAX; 26];
        let mut last = [0; 26];
        for (i, &c) in b.iter().enumerate() {
            let c = (c - b'a') as usize;
            if first[c] == usize::MAX {
                first[c] = i;
            }
            last[c] = i;
        }

        let mut blocks: Vec<(usize, usize)> = Vec::new();
        for c in 0..26 {
            let start = match first[c] {
                usize::MAX => continue,
                f => f,
            };
            let mut end = last[c];
            let mut i = start;
            let mut ok = true;
            while i <= end {
                let k = (b[i] - b'a') as usize;
                if first[k] < start {
                    ok = false;
                    break;
                }
                end = end.max(last[k]);
                i += 1;
            }
            if ok {
                blocks.push((start, end));
            }
        }

        // Interval scheduling: earliest finish first (shorter wins on ties).
        blocks.sort_unstable_by(|x, y| x.1.cmp(&y.1).then(y.0.cmp(&x.0)));

        let mut ans: Vec<String> = Vec::new();
        let mut next_free = 0;
        for (start, end) in blocks {
            if start >= next_free {
                ans.push(s[start..=end].to_string());
                next_free = end + 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_number_of_non_overlapping_substrings::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_max_num_of_substrings_1() {
        let s = "adefaddaccc".to_string();
        let output = to_string_vec(["e", "f", "ccc"]);
        assert_eq!(output, Solution::max_num_of_substrings(s));
    }

    #[test]
    fn test_max_num_of_substrings_2() {
        let s = "adefaddaccc".to_string();
        let output = to_string_vec(["e", "f", "ccc"]);
        assert_eq!(output, Solution::max_num_of_substrings(s));
    }
}
