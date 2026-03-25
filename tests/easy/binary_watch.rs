// 401. Binary Watch
// https://leetcode.com/problems/binary-watch/

struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut result = Vec::new();

        // Iterate through all valid hours (0-11) and minutes (0-59)
        for hour in 0u32..12 {
            for minute in 0u32..60 {
                // Count the number of 1s in the binary representation
                let bits = hour.count_ones() + minute.count_ones();

                if bits == turned_on as u32 {
                    // Format as "H:MM"
                    result.push(format!("{}:{:02}", hour, minute));
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::binary_watch::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_read_binary_watch_1() {
        let turned_on = 1;
        let output = to_string_vec([
            "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00",
        ]);
        assert_eq!(output, Solution::read_binary_watch(turned_on));
    }

    #[test]
    fn test_read_binary_watch_2() {
        let turned_on = 9;
        let output: Vec<String> = vec![];
        assert_eq!(output, Solution::read_binary_watch(turned_on));
    }
}
