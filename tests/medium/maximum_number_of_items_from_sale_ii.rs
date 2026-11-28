// 3947. Maximum Number of Items From Sale II
// https://leetcode.com/problems/maximum-number-of-items-from-sale-ii/

struct Solution;

impl Solution {
    // Buying c copies of item i yields c + min(c, gain[i]) copies, where gain[i]
    // counts the other items whose factor is a multiple of factor[i]: each copy
    // can hand out at most one free copy and each ordered pair (i, j) fires once.
    // So the first gain[i] copies of item i are worth 2 copies each and every
    // later copy is worth 1. Plain value-1 copies always come from the cheapest
    // item (price p_min), and a boosted copy at price p beats spending that same
    // money on cheap copies only while p < 2 * p_min; among those, buying the
    // cheapest prices first minimizes the cost of any chosen count.
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        let n = items.len();
        let mut freq = vec![0i64; n + 1];
        for it in &items {
            freq[it[0] as usize] += 1;
        }

        // div_cnt[f] = number of items whose factor is a multiple of f.
        let mut div_cnt = vec![0i64; n + 1];
        for (f, cnt) in div_cnt.iter_mut().enumerate().skip(1) {
            let mut multiple = f;
            while multiple <= n {
                *cnt += freq[multiple];
                multiple += f;
            }
        }

        // (price, boosted copies available) per item type, cheapest first.
        let mut boosts: Vec<(i64, i64)> = items
            .iter()
            .map(|it| (i64::from(it[1]), div_cnt[it[0] as usize] - 1))
            .collect();
        boosts.sort_unstable_by_key(|&(price, _)| price);

        let budget = i64::from(budget);
        let p_min = boosts[0].0;
        let mut boosted: i64 = 0;
        let mut spent: i64 = 0;
        for &(price, gain) in &boosts {
            if price >= 2 * p_min || spent + price > budget {
                break;
            }
            let take = gain.min((budget - spent) / price);
            boosted += take;
            spent += take * price;
        }

        // Leftover money buys plain copies of the cheapest item.
        (2 * boosted + (budget - spent) / p_min).min(i64::from(i32::MAX)) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_number_of_items_from_sale_ii::Solution;

    #[test]
    fn test_maximum_sale_items_1() {
        let items: Vec<Vec<i32>> = vec![vec![1, 6], vec![2, 4], vec![3, 5]];
        let budget: i32 = 19;
        assert_eq!(5, Solution::maximum_sale_items(items, budget));
    }

    #[test]
    fn test_maximum_sale_items_2() {
        let items: Vec<Vec<i32>> = vec![
            vec![2, 8],
            vec![1, 10],
            vec![6, 6],
            vec![4, 12],
            vec![5, 20],
            vec![5, 17],
        ];
        let budget: i32 = 35;
        assert_eq!(7, Solution::maximum_sale_items(items, budget));
    }
}
