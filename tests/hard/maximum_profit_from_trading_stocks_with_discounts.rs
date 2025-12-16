// 3562. Maximum Profit from Trading Stocks with Discounts
// https://leetcode.com/problems/maximum-profit-from-trading-stocks-with-discounts/

struct Solution;

impl Solution {
    pub fn max_profit(
        n: i32,
        present: Vec<i32>,
        future: Vec<i32>,
        hierarchy: Vec<Vec<i32>>,
        budget: i32,
    ) -> i32 {
        let n = n as usize;
        let budget = budget as usize;
        let mut adj = vec![vec![]; n];
        let mut has_parent = vec![false; n];

        for edge in hierarchy {
            let u = (edge[0] - 1) as usize;
            let v = (edge[1] - 1) as usize;
            adj[u].push(v);
            has_parent[v] = true;
        }

        let mut roots = vec![];
        for i in 0..n {
            if !has_parent[i] {
                roots.push(i);
            }
        }

        const INF: i32 = -1_000_000_000;
        let mut dp = vec![INF; budget + 1];
        dp[0] = 0;

        let mut memo = vec![vec![None; 2]; n];

        for root in roots {
            let tree_dp = Self::dfs(root, false, &adj, &present, &future, budget, &mut memo);
            let mut next_dp = vec![INF; budget + 1];

            let max_c_dp = dp.iter().rposition(|&x| x != INF).unwrap_or(0);
            let max_c_tree = tree_dp.iter().rposition(|&x| x != INF).unwrap_or(0);

            for c1 in 0..=max_c_dp {
                if dp[c1] == INF {
                    continue;
                }
                for c2 in 0..=max_c_tree {
                    if c1 + c2 > budget {
                        break;
                    }
                    if tree_dp[c2] == INF {
                        continue;
                    }
                    next_dp[c1 + c2] = next_dp[c1 + c2].max(dp[c1] + tree_dp[c2]);
                }
            }
            dp = next_dp;
        }

        *dp.iter().max().unwrap_or(&0)
    }

    fn dfs(
        u: usize,
        parent_bought: bool,
        adj: &Vec<Vec<usize>>,
        present: &Vec<i32>,
        future: &Vec<i32>,
        budget: usize,
        memo: &mut Vec<Vec<Option<Vec<i32>>>>,
    ) -> Vec<i32> {
        if let Some(ref res) = memo[u][parent_bought as usize] {
            return res.clone();
        }

        const INF: i32 = -1_000_000_000;

        // Option 1: Don't buy u
        let mut dp_not_bought = vec![INF; budget + 1];
        dp_not_bought[0] = 0;

        for &v in &adj[u] {
            let child_dp = Self::dfs(v, false, adj, present, future, budget, memo);
            dp_not_bought = Self::merge(&dp_not_bought, &child_dp, budget);
        }

        // Option 2: Buy u
        let cost = if parent_bought {
            present[u] / 2
        } else {
            present[u]
        } as usize;

        let profit = future[u]
            - (if parent_bought {
                present[u] / 2
            } else {
                present[u]
            });

        let mut dp_bought = vec![INF; budget + 1];

        if cost <= budget {
            let mut children_combined = vec![INF; budget + 1];
            children_combined[0] = 0;

            for &v in &adj[u] {
                let child_dp = Self::dfs(v, true, adj, present, future, budget, memo);
                children_combined = Self::merge(&children_combined, &child_dp, budget);
            }

            let max_c = children_combined
                .iter()
                .rposition(|&x| x != INF)
                .unwrap_or(0);
            for c in 0..=max_c {
                if children_combined[c] == INF {
                    continue;
                }
                if c + cost <= budget {
                    dp_bought[c + cost] = children_combined[c] + profit;
                }
            }
        }

        let mut res = dp_not_bought;
        for c in 0..=budget {
            if dp_bought[c] != INF {
                res[c] = res[c].max(dp_bought[c]);
            }
        }
        memo[u][parent_bought as usize] = Some(res.clone());
        res
    }

    fn merge(dp1: &Vec<i32>, dp2: &Vec<i32>, budget: usize) -> Vec<i32> {
        const INF: i32 = -1_000_000_000;
        let mut res = vec![INF; budget + 1];
        let max_c1 = dp1.iter().rposition(|&x| x != INF).unwrap_or(0);
        let max_c2 = dp2.iter().rposition(|&x| x != INF).unwrap_or(0);

        for c1 in 0..=max_c1 {
            if dp1[c1] == INF {
                continue;
            }
            for c2 in 0..=max_c2 {
                if c1 + c2 > budget {
                    break;
                }
                if dp2[c2] == INF {
                    continue;
                }
                res[c1 + c2] = res[c1 + c2].max(dp1[c1] + dp2[c2]);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_profit_from_trading_stocks_with_discounts::Solution;

    #[test]
    fn test_max_profit_1() {
        let n = 2;
        let present = [1, 2].to_vec();
        let future = [4, 3].to_vec();
        let hierarchy = [[1, 2]].iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        let budget = 3;
        assert_eq!(
            5,
            Solution::max_profit(n, present, future, hierarchy, budget)
        );
    }

    #[test]
    fn test_max_profit_2() {
        let n = 2;
        let present = [3, 4].to_vec();
        let future = [5, 8].to_vec();
        let hierarchy = [[1, 2]].iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        let budget = 4;
        assert_eq!(
            4,
            Solution::max_profit(n, present, future, hierarchy, budget)
        );
    }

    #[test]
    fn test_max_profit_3() {
        let n = 3;
        let present = [4, 6, 8].to_vec();
        let future = [7, 9, 11].to_vec();
        let hierarchy = [[1, 2], [1, 3]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let budget = 10;
        assert_eq!(
            10,
            Solution::max_profit(n, present, future, hierarchy, budget)
        );
    }

    #[test]
    fn test_max_profit_4() {
        let n = 3;
        let present = [5, 2, 3].to_vec();
        let future = [8, 5, 6].to_vec();
        let hierarchy = [[1, 2], [2, 3]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let budget = 7;
        assert_eq!(
            12,
            Solution::max_profit(n, present, future, hierarchy, budget)
        );
    }

    #[test]
    fn test_max_profit_5() {
        let n = 160;
        let present = [
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ]
        .to_vec();
        let future = [
            50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
            50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
            50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
            50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
            50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
            50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
            50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
            50, 50, 50, 50, 50, 50,
        ]
        .to_vec();
        let hierarchy = [
            [1, 2],
            [2, 3],
            [3, 4],
            [4, 5],
            [5, 6],
            [6, 7],
            [7, 8],
            [8, 9],
            [9, 10],
            [10, 11],
            [11, 12],
            [12, 13],
            [13, 14],
            [14, 15],
            [15, 16],
            [16, 17],
            [17, 18],
            [18, 19],
            [19, 20],
            [20, 21],
            [21, 22],
            [22, 23],
            [23, 24],
            [24, 25],
            [25, 26],
            [26, 27],
            [27, 28],
            [28, 29],
            [29, 30],
            [30, 31],
            [31, 32],
            [32, 33],
            [33, 34],
            [34, 35],
            [35, 36],
            [36, 37],
            [37, 38],
            [38, 39],
            [39, 40],
            [40, 41],
            [41, 42],
            [42, 43],
            [43, 44],
            [44, 45],
            [45, 46],
            [46, 47],
            [47, 48],
            [48, 49],
            [49, 50],
            [50, 51],
            [51, 52],
            [52, 53],
            [53, 54],
            [54, 55],
            [55, 56],
            [56, 57],
            [57, 58],
            [58, 59],
            [59, 60],
            [60, 61],
            [61, 62],
            [62, 63],
            [63, 64],
            [64, 65],
            [65, 66],
            [66, 67],
            [67, 68],
            [68, 69],
            [69, 70],
            [70, 71],
            [71, 72],
            [72, 73],
            [73, 74],
            [74, 75],
            [75, 76],
            [76, 77],
            [77, 78],
            [78, 79],
            [79, 80],
            [80, 81],
            [81, 82],
            [82, 83],
            [83, 84],
            [84, 85],
            [85, 86],
            [86, 87],
            [87, 88],
            [88, 89],
            [89, 90],
            [90, 91],
            [91, 92],
            [92, 93],
            [93, 94],
            [94, 95],
            [95, 96],
            [96, 97],
            [97, 98],
            [98, 99],
            [99, 100],
            [100, 101],
            [101, 102],
            [102, 103],
            [103, 104],
            [104, 105],
            [105, 106],
            [106, 107],
            [107, 108],
            [108, 109],
            [109, 110],
            [110, 111],
            [111, 112],
            [112, 113],
            [113, 114],
            [114, 115],
            [115, 116],
            [116, 117],
            [117, 118],
            [118, 119],
            [119, 120],
            [120, 121],
            [121, 122],
            [122, 123],
            [123, 124],
            [124, 125],
            [125, 126],
            [126, 127],
            [127, 128],
            [128, 129],
            [129, 130],
            [130, 131],
            [131, 132],
            [132, 133],
            [133, 134],
            [134, 135],
            [135, 136],
            [136, 137],
            [137, 138],
            [138, 139],
            [139, 140],
            [140, 141],
            [141, 142],
            [142, 143],
            [143, 144],
            [144, 145],
            [145, 146],
            [146, 147],
            [147, 148],
            [148, 149],
            [149, 150],
            [150, 151],
            [151, 152],
            [152, 153],
            [153, 154],
            [154, 155],
            [155, 156],
            [156, 157],
            [157, 158],
            [158, 159],
            [159, 160],
        ]
        .iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        let budget = 160;
        assert_eq!(
            7999,
            Solution::max_profit(n, present, future, hierarchy, budget)
        );
    }
}
