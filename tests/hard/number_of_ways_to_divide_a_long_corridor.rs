// 2147. Number of Ways to Divide a Long Corridor
// https://leetcode.com/problems/number-of-ways-to-divide-a-long-corridor/

struct Solution;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        todo!()
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
