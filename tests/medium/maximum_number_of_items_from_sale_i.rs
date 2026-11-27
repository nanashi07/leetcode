// 3946. Maximum Number of Items From Sale I
// https://leetcode.com/problems/maximum-number-of-items-from-sale-i/

struct Solution;

const MAX_FACTOR: usize = 1500;

impl Solution {
    // Knapsack over the budget. Activating an item type (first copy) is worth
    // 1 + bonus_i where bonus_i counts other items whose factor is a multiple of
    // factor_i; every further copy of that type is worth exactly 1.
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        let budget = budget as usize;
        let mut freq = vec![0i32; MAX_FACTOR + 1];
        for it in &items {
            freq[it[0] as usize] += 1;
        }

        // div_cnt[f] = number of items whose factor is divisible by f.
        let mut div_cnt = vec![0i32; MAX_FACTOR + 1];
        for f in 1..=MAX_FACTOR {
            if freq[f] > 0 {
                let mut count = 0;
                let mut multiple = f;
                while multiple <= MAX_FACTOR {
                    count += freq[multiple];
                    multiple += f;
                }
                div_cnt[f] = count;
            }
        }

        let neg = i32::MIN / 2;
        let mut dp = vec![neg; budget + 1];
        dp[0] = 0;

        for it in &items {
            let price = it[1] as usize;
            if price > budget {
                continue;
            }
            let bonus = div_cnt[it[0] as usize] - 1;

            // First copy of this type (0/1 transition, descending keeps dp[b - price]
            // at its pre-item value).
            for b in (price..=budget).rev() {
                if dp[b - price] != neg {
                    dp[b] = dp[b].max(dp[b - price] + 1 + bonus);
                }
            }

            // Extra copies of an already-activated type (complete knapsack).
            for b in price..=budget {
                if dp[b - price] != neg {
                    dp[b] = dp[b].max(dp[b - price] + 1);
                }
            }
        }

        dp.iter().copied().max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_number_of_items_from_sale_i::Solution;

    #[test]
    fn test_maximum_sale_items_1() {
        let items: Vec<Vec<i32>> = vec![vec![6, 2], vec![2, 6], vec![3, 4]];
        let budget: i32 = 9;
        assert_eq!(4, Solution::maximum_sale_items(items, budget));
    }

    #[test]
    fn test_maximum_sale_items_2() {
        let items: Vec<Vec<i32>> =
            vec![vec![2, 4], vec![3, 2], vec![4, 1], vec![6, 4], vec![12, 4]];
        let budget: i32 = 8;
        assert_eq!(10, Solution::maximum_sale_items(items, budget));
    }
}
