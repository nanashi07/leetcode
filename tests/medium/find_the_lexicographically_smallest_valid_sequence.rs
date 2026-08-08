// 3302. Find the Lexicographically Smallest Valid Sequence
// https://leetcode.com/problems/find-the-lexicographically-smallest-valid-sequence/

struct Solution;

impl Solution {
    pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();
        let n = w1.len();
        let m = w2.len();

        let mut last = vec![-1i32; m + 1];
        last[m] = n as i32;
        let mut k = (n as i32) - 1;
        for j in (0..m).rev() {
            while k >= 0 && w1[k as usize] != w2[j] {
                k -= 1;
            }
            if k >= 0 {
                last[j] = k;
                k -= 1;
            }
        }

        let mut ans = Vec::with_capacity(m);
        let mut curr_i = 0;
        let mut changed = false;

        for j in 0..m {
            if curr_i >= n {
                return vec![];
            }
            if w1[curr_i] == w2[j] {
                ans.push(curr_i as i32);
                curr_i += 1;
            } else if !changed && (curr_i as i32) < last[j + 1] {
                ans.push(curr_i as i32);
                changed = true;
                curr_i += 1;
            } else if let Some(pos) = w1[curr_i..].iter().position(|&x| x == w2[j]) {
                let idx = curr_i + pos;
                ans.push(idx as i32);
                curr_i = idx + 1;
            } else {
                return vec![];
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_the_lexicographically_smallest_valid_sequence::Solution;

    #[test]
    fn test_valid_sequence_1() {
        let word1 = "vbcca".to_string();
        let word2 = "abc".to_string();
        assert_eq!([0, 1, 2].to_vec(), Solution::valid_sequence(word1, word2));
    }

    #[test]
    fn test_valid_sequence_2() {
        let word1 = "bacdc".to_string();
        let word2 = "abc".to_string();
        assert_eq!([1, 2, 4].to_vec(), Solution::valid_sequence(word1, word2));
    }

    #[test]
    fn test_valid_sequence_3() {
        let word1 = "aaaaaa".to_string();
        let word2 = "aaabc".to_string();
        assert_eq!([0; 0].to_vec(), Solution::valid_sequence(word1, word2));
    }

    #[test]
    fn test_valid_sequence_4() {
        let word1 = "abc".to_string();
        let word2 = "ab".to_string();
        assert_eq!([0, 1].to_vec(), Solution::valid_sequence(word1, word2));
    }
}
