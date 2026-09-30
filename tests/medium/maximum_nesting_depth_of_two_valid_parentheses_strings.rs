// 1111. Maximum Nesting Depth of Two Valid Parentheses Strings
// https://leetcode.com/problems/maximum-nesting-depth-of-two-valid-parentheses-strings/

struct Solution;

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut ans = vec![0i32; seq.len()];

        // Within one primitive group, parens on odd nesting levels go to A and even levels to B,
        // which keeps each part at most ceil(depth / 2) deep. Flipping every label of a whole
        // primitive leaves both parts valid, so pick the orientation that balances their lengths.
        let (mut depth, mut start, mut count_a, mut count_b) = (0usize, 0usize, 0usize, 0usize);
        let (mut total_a, mut total_b) = (0usize, 0usize);

        for (i, bracket) in seq.bytes().enumerate() {
            if bracket == b'(' {
                depth += 1;
            }

            let group = ((depth - 1) & 1) as i32;
            ans[i] = group;
            if group == 0 {
                count_a += 1;
            } else {
                count_b += 1;
            }

            if bracket == b')' {
                depth -= 1;
                if depth == 0 {
                    // End of a primitive group: keep the orientation unless flipping it brings the
                    // two lengths closer together.
                    let flipped = (total_a + count_b).abs_diff(total_b + count_a)
                        < (total_a + count_a).abs_diff(total_b + count_b);
                    if flipped {
                        for slot in &mut ans[start..=i] {
                            *slot ^= 1;
                        }
                        total_a += count_b;
                        total_b += count_a;
                    } else {
                        total_a += count_a;
                        total_b += count_b;
                    }
                    start = i + 1;
                    count_a = 0;
                    count_b = 0;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_nesting_depth_of_two_valid_parentheses_strings::Solution;

    #[test]
    fn test_max_depth_after_split_1() {
        let seq = "(()())".to_string();
        assert_eq!(
            [0, 1, 1, 1, 1, 0].to_vec(),
            Solution::max_depth_after_split(seq)
        );
    }

    #[test]
    fn test_max_depth_after_split_2() {
        let seq = "()(())()".to_string();
        assert_eq!(
            [0, 0, 0, 1, 1, 0, 1, 1].to_vec(),
            Solution::max_depth_after_split(seq)
        );
    }
}
