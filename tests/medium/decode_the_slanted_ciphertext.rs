// 2075. Decode the Slanted Ciphertext
// https://leetcode.com/problems/decode-the-slanted-ciphertext/

struct Solution;

impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let rows = rows as usize;
        let n = encoded_text.len();
        let cols = n / rows;
        let bytes = encoded_text.as_bytes();

        let mut result = Vec::with_capacity(n);

        for d in 0..(rows + cols - 1) {
            let (mut r, mut c) = if d < cols { (0, d) } else { (d - cols + 1, 0) };
            while r < rows && c < cols {
                result.push(bytes[r * cols + c]);
                r += 1;
                c += 1;
            }
        }

        while result.last() == Some(&b' ') {
            result.pop();
        }

        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::decode_the_slanted_ciphertext::Solution;

    #[test]
    fn test_decode_ciphertext_1() {
        let encoded_text = "ch   ie   pr".to_string();
        let rows = 3;
        assert_eq!(
            "cipher".to_string(),
            Solution::decode_ciphertext(encoded_text, rows)
        );
    }

    #[test]
    fn test_decode_ciphertext_2() {
        let encoded_text = "iveo    eed   l te   olc".to_string();
        let rows = 4;
        assert_eq!(
            "i love leetcode".to_string(),
            Solution::decode_ciphertext(encoded_text, rows)
        );
    }

    #[test]
    fn test_decode_ciphertext_3() {
        let encoded_text = "coding".to_string();
        let rows = 1;
        assert_eq!(
            "coding".to_string(),
            Solution::decode_ciphertext(encoded_text, rows)
        );
    }
}
