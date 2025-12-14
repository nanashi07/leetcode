// 2147. Number of Ways to Divide a Long Corridor
// https://leetcode.com/problems/number-of-ways-to-divide-a-long-corridor/

struct Solution;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let s_indices: Vec<usize> = corridor
            .bytes()
            .enumerate()
            .filter(|&(_, b)| b == b'S')
            .map(|(i, _)| i)
            .collect();

        if s_indices.len() == 0 || s_indices.len() % 2 != 0 {
            return 0;
        }

        let mut ans: i64 = 1;
        let modulo: i64 = 1_000_000_007;

        for i in (2..s_indices.len()).step_by(2) {
            let prev_end = s_indices[i - 1];
            let curr_start = s_indices[i];
            let ways = (curr_start - prev_end) as i64;
            ans = (ans * ways) % modulo;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::number_of_ways_to_divide_a_long_corridor::Solution;

    #[test]
    fn test_number_of_ways_1() {
        let corridor = "SSPPSPS".to_string();
        assert_eq!(3, Solution::number_of_ways(corridor));
    }

    #[test]
    fn test_number_of_ways_2() {
        let corridor = "PPSPSP".to_string();
        assert_eq!(1, Solution::number_of_ways(corridor));
    }

    #[test]
    fn test_number_of_ways_3() {
        let corridor = "S".to_string();
        assert_eq!(0, Solution::number_of_ways(corridor));
    }
}
