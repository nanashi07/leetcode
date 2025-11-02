// 3480. Maximize Subarrays After Removing One Conflicting Pair
// https://leetcode.com/problems/maximize-subarrays-after-removing-one-conflicting-pair/

struct Solution;

impl Solution {
    // https://leetcode.com/problems/maximize-subarrays-after-removing-one-conflicting-pair/editorial/

    /// Maximizes the number of subarrays after removing at most one conflicting pair.
    /// 在移除最多一個衝突對後，最大化子陣列的數量。
    ///
    /// A conflicting pair (a, b) prevents the formation of any subarray that contains both elements a and b.
    /// 衝突對 (a, b) 會阻止任何同時包含元素 a 和 b 的子陣列的形成。
    /// We can remove at most one such pair to maximize the total number of valid subarrays.
    /// 我們可以移除最多一個這樣的對來最大化有效子陣列的總數。
    ///
    /// Algorithm / 演算法流程:
    /// 1. For each left endpoint i, track the two smallest right endpoints of conflicting pairs
    ///    對於每個左端點 i，追蹤衝突對的兩個最小右端點
    /// 2. Calculate base score: sum of all possible subarrays without removing any pair
    ///    計算基礎分數：不移除任何對的情況下所有可能子陣列的總和
    /// 3. For each potential pair removal, calculate the additional subarrays gained
    ///    對於每個潛在的對移除，計算獲得的額外子陣列
    /// 4. Return base score + maximum additional gain
    ///    返回基礎分數 + 最大額外收益
    pub fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;

        // b_min1[i]: smallest right endpoint of conflicting pairs starting at i
        // b_min1[i]: 以 i 為起點的衝突對中最小的右端點
        let mut b_min1 = vec![i32::MAX; n + 1];

        // b_min2[i]: second smallest right endpoint of conflicting pairs starting at i
        // b_min2[i]: 以 i 為起點的衝突對中第二小的右端點
        let mut b_min2 = vec![i32::MAX; n + 1];

        // Process all conflicting pairs and group by left endpoint
        // 處理所有衝突對，按左端點分組
        for pair in conflicting_pairs {
            let a = pair[0].min(pair[1]) as usize; // left endpoint (smaller value) / 左端點（較小值）
            let b = pair[0].max(pair[1]); // right endpoint (larger value) / 右端點（較大值）

            // Update the two smallest right endpoints for left endpoint 'a'
            // 更新左端點 'a' 對應的兩個最小右端點
            if b_min1[a] > b {
                b_min2[a] = b_min1[a]; // previous min becomes second min / 之前的最小值變成第二小
                b_min1[a] = b; // new minimum / 新的最小值
            } else if b_min2[a] > b {
                b_min2[a] = b; // new second minimum / 新的第二小值
            }
        }

        let mut res: i64 = 0; // base score (no pairs removed) / 基礎分數（未移除任何對）
        let mut ib1 = n; // index of current best left endpoint / 當前最佳左端點的索引
        let mut b2 = i32::MAX; // second best right endpoint overall / 全局第二好的右端點
        let mut del_count = vec![0i64; n + 1]; // gain from removing each specific pair / 移除每個特定對的收益

        // Iterate from right to left to build the solution
        // 從右到左遍歷構建解決方案
        for i in (1..=n).rev() {
            // Update the best conflicting pair seen so far
            // 更新到目前為止看到的最佳衝突對
            if b_min1[ib1] > b_min1[i] {
                // Found a better left endpoint with smaller first right endpoint
                // 找到了具有更小首個右端點的更好左端點
                b2 = b2.min(b_min1[ib1]); // update second best / 更新第二好的
                ib1 = i; // update best left endpoint / 更新最佳左端點
            } else {
                // Current left endpoint is not better, but update second best
                // 當前左端點不是更好的，但更新第二好的
                b2 = b2.min(b_min1[i]);
            }

            // Add to base score: number of subarrays starting at i and ending before
            // the first conflicting right endpoint
            // 添加到基礎分數：從 i 開始並在第一個衝突右端點之前結束的子陣列數量
            res += (b_min1[ib1].min((n + 1) as i32) - i as i32) as i64;

            // Calculate additional gain if we remove the best conflicting pair at ib1
            // This allows subarrays to extend further (up to second best right endpoint)
            // 計算如果我們移除 ib1 處最佳衝突對的額外收益
            // 這允許子陣列延伸得更遠（直到第二好的右端點）
            del_count[ib1] +=
                (b2.min(b_min2[ib1]).min((n + 1) as i32) - b_min1[ib1].min((n + 1) as i32)) as i64;
        }

        // Return base score + maximum additional gain from removing one pair
        // 返回基礎分數 + 移除一個對的最大額外收益
        res + *del_count.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximize_subarrays_after_removing_one_conflicting_pair::Solution;

    #[test]
    fn test_max_subarrays_1() {
        let n = 4;
        let conflicting_pairs = [[2, 3], [1, 4]]
            .iter()
            .map(|&s| s.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(9, Solution::max_subarrays(n, conflicting_pairs));
    }

    #[test]
    fn test_max_subarrays_2() {
        let n = 5;
        let conflicting_pairs = [[1, 2], [2, 5], [3, 5]]
            .iter()
            .map(|&s| s.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(12, Solution::max_subarrays(n, conflicting_pairs));
    }
}
