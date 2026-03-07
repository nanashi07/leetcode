// 1888. Minimum Number of Flips to Make the Binary String Alternating
// https://leetcode.com/problems/minimum-number-of-flips-to-make-the-binary-string-alternating/

struct Solution;

impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let n = s.len();
        let bytes = s.as_bytes();

        // diff0: mismatches vs pattern starting with '0' ("010101...")
        // diff1: mismatches vs pattern starting with '1' ("101010...")
        let mut diff0 = 0i32;
        let mut diff1 = 0i32;

        for i in 0..n {
            let c = bytes[i];
            if (c == b'0') != (i % 2 == 0) { diff0 += 1; }
            if (c == b'1') != (i % 2 == 0) { diff1 += 1; }
        }

        let mut result = diff0.min(diff1);

        // Slide window over doubled string (s+s), window size n
        for i in 0..n {
            // Remove position i (absolute)
            let c_out = bytes[i];
            if (c_out == b'0') != (i % 2 == 0) { diff0 -= 1; }
            if (c_out == b'1') != (i % 2 == 0) { diff1 -= 1; }

            // Add position i+n (in s+s, index i+n maps to bytes[i])
            let c_in = bytes[i];
            let abs_pos = i + n;
            if (c_in == b'0') != (abs_pos % 2 == 0) { diff0 += 1; }
            if (c_in == b'1') != (abs_pos % 2 == 0) { diff1 += 1; }

            result = result.min(diff0).min(diff1);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_number_of_flips_to_make_the_binary_string_alternating::Solution;

    #[test]
    fn test_min_flips_1() {
        let s = "111000".to_string();
        assert_eq!(2, Solution::min_flips(s));
    }

    #[test]
    fn test_min_flips_2() {
        let s = "010".to_string();
        assert_eq!(0, Solution::min_flips(s));
    }

    #[test]
    fn test_min_flips_3() {
        let s = "1110".to_string();
        assert_eq!(1, Solution::min_flips(s));
    }
}
