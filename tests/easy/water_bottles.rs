// # 1518. Water Bottles
// https://leetcode.com/problems/water-bottles/

struct Solution;

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::water_bottles::Solution;

    #[test]
    fn test_num_water_bottles_1() {
        let num_bottles = 9;
        let num_exchange = 3;
        assert_eq!(13, Solution::num_water_bottles(num_bottles, num_exchange));
    }

    #[test]
    fn test_num_water_bottles_2() {
        let num_bottles = 15;
        let num_exchange = 4;
        assert_eq!(19, Solution::num_water_bottles(num_bottles, num_exchange));
    }
}
