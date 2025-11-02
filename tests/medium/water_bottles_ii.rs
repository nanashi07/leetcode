// 3100. Water Bottles II
// https://leetcode.com/problems/water-bottles-ii/

struct Solution;

impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut total_drunk = 0;
        let mut empty_bottles = 0;
        let mut full_bottles = num_bottles;
        let mut exchange_cost = num_exchange;

        loop {
            // Drink all full bottles
            total_drunk += full_bottles;
            empty_bottles += full_bottles;

            // Try to exchange empty bottles for full ones
            if empty_bottles >= exchange_cost {
                empty_bottles -= exchange_cost;
                full_bottles = 1;
                exchange_cost += 1;
            } else {
                // Can't exchange anymore, we're done
                break;
            }
        }

        total_drunk
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::water_bottles_ii::Solution;

    #[test]
    fn test_max_bottles_drunk_1() {
        let num_bottles = 13;
        let num_exchange = 6;
        assert_eq!(15, Solution::max_bottles_drunk(num_bottles, num_exchange));
    }

    #[test]
    fn test_max_bottles_drunk_2() {
        let num_bottles = 10;
        let num_exchange = 3;
        assert_eq!(13, Solution::max_bottles_drunk(num_bottles, num_exchange));
    }
}
