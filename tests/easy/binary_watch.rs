// 401. Binary Watch
// https://leetcode.com/problems/binary-watch/

struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::binary_watch::Solution;

    #[test]
    fn test_read_binary_watch_1() {
        let turned_on = 1;
        let output = [
            "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        assert_eq!(output, Solution::read_binary_watch(turned_on));
    }

    #[test]
    fn test_read_binary_watch_2() {
        let turned_on = 9;
        let output: Vec<String> = vec![];
        assert_eq!(output, Solution::read_binary_watch(turned_on));
    }
}
