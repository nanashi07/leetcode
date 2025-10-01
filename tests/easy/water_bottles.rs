// # 1518. Water Bottles
// https://leetcode.com/problems/water-bottles/

struct Solution;

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        println!("num_bottles: {num_bottles}, num_exchange: {num_exchange}");

        let mut drink = 0;
        let mut bottles = num_bottles;
        let mut empty = 0;

        while bottles + empty >= num_exchange {
            drink += bottles;

            let exchanged = (bottles + empty) / num_exchange;
            let remained = (bottles + empty) % num_exchange;

            bottles = exchanged;
            empty = remained;
        }

        drink + bottles
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
