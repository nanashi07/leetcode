// 2075. Decode the Slanted Ciphertext
// https://leetcode.com/problems/decode-the-slanted-ciphertext/

struct Solution;

impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        todo!()
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
