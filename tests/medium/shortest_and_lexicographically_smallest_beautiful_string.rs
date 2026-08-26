// 2904. Shortest and Lexicographically Smallest Beautiful String
// https://leetcode.com/problems/shortest-and-lexicographically-smallest-beautiful-string/

struct Solution;

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::shortest_and_lexicographically_smallest_beautiful_string::Solution;

    #[test]
    fn test_shortest_beautiful_substring_1() {
        let s = "100011001".to_string();
        let k = 3;
        assert_eq!(
            "11001".to_string(),
            Solution::shortest_beautiful_substring(s, k)
        );
    }

    #[test]
    fn test_shortest_beautiful_substring_2() {
        let s = "1011".to_string();
        let k = 2;
        assert_eq!(
            "11".to_string(),
            Solution::shortest_beautiful_substring(s, k)
        );
    }

    #[test]
    fn test_shortest_beautiful_substring_3() {
        let s = "000".to_string();
        let k = 1;
        assert_eq!("".to_string(), Solution::shortest_beautiful_substring(s, k));
    }
}
