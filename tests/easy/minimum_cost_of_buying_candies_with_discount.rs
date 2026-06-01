// 2144. Minimum Cost of Buying Candies With Discount
// https://leetcode.com/problems/minimum-cost-of-buying-candies-with-discount/

struct Solution;

impl Solution {
    pub fn minimum_cost(cost: Vec<i32>) -> i32 {
        let mut cost = cost;
        cost.sort_unstable();
        let mut spend = 0;
        while !cost.is_empty() {
            if cost.len() > 1 {
                let low_price = cost.remove(cost.len() - 1);
                spend += low_price;
                let low_price = cost.remove(cost.len() - 1);
                spend += low_price;

                if !cost.is_empty() {
                    let mut len = cost.len();
                    while len > 0 {
                        if cost[len - 1] <= low_price {
                            cost.remove(len - 1);
                            break;
                        } else {
                            len -= 1;
                        }
                    }
                }
            } else {
                spend += cost.remove(cost.len() - 1);
            }
        }
        spend
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::minimum_cost_of_buying_candies_with_discount::Solution;

    #[test]
    fn test_minimum_cost_1() {
        let cost = [1, 2, 3].to_vec();
        assert_eq!(5, Solution::minimum_cost(cost));
    }

    #[test]
    fn test_minimum_cost_2() {
        let cost = [6, 5, 7, 9, 2, 2].to_vec();
        assert_eq!(23, Solution::minimum_cost(cost));
    }

    #[test]
    fn test_minimum_cost_3() {
        let cost = [5, 5].to_vec();
        assert_eq!(10, Solution::minimum_cost(cost));
    }

    #[test]
    fn test_minimum_cost_4() {
        let cost = [1].to_vec();
        assert_eq!(1, Solution::minimum_cost(cost));
    }
}
