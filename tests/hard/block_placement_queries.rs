// 3161. Block Placement Queries
// https://leetcode.com/problems/block-placement-queries/

struct Solution;

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        use std::collections::BTreeSet;

        struct SegTree {
            n: usize,
            data: Vec<i32>,
        }

        impl SegTree {
            fn new(size: usize) -> Self {
                let mut n = 1usize;
                while n < size {
                    n <<= 1;
                }
                Self {
                    n,
                    data: vec![0; n << 1],
                }
            }

            fn set(&mut self, idx: usize, val: i32) {
                let mut p = idx + self.n;
                self.data[p] = val;
                p >>= 1;
                while p > 0 {
                    self.data[p] = self.data[p << 1].max(self.data[(p << 1) | 1]);
                    p >>= 1;
                }
            }

            fn query_max(&self, mut l: usize, mut r: usize) -> i32 {
                if l > r {
                    return 0;
                }
                l += self.n;
                r += self.n;
                let mut ans = 0;
                while l <= r {
                    if (l & 1) == 1 {
                        ans = ans.max(self.data[l]);
                        l += 1;
                    }
                    if (r & 1) == 0 {
                        ans = ans.max(self.data[r]);
                        if r == 0 {
                            break;
                        }
                        r -= 1;
                    }
                    l >>= 1;
                    r >>= 1;
                }
                ans
            }
        }

        let mut max_x = 0usize;
        let mut inserted = Vec::new();
        for q in &queries {
            max_x = max_x.max(q[1] as usize);
            if q[0] == 1 {
                inserted.push(q[1] as usize);
            }
        }

        // Sentinel to model an obstacle beyond every queried x.
        let sentinel = max_x + 1;

        let mut active = BTreeSet::new();
        active.insert(0usize);
        active.insert(sentinel);
        for &x in &inserted {
            active.insert(x);
        }

        let mut seg = SegTree::new(sentinel + 1);
        let mut prev = 0usize;
        let mut first = true;
        for &x in &active {
            if first {
                first = false;
                prev = x;
                continue;
            }
            seg.set(x, (x - prev) as i32);
            prev = x;
        }

        let mut rev_ans = Vec::new();

        for q in queries.iter().rev() {
            if q[0] == 1 {
                let x = q[1] as usize;
                if !active.contains(&x) {
                    continue;
                }
                let l = *active.range(..x).next_back().unwrap();
                let r_opt = active.range((x + 1)..).next().copied();

                seg.set(x, 0);
                if let Some(r) = r_opt {
                    seg.set(r, (r - l) as i32);
                }
                active.remove(&x);
            } else {
                let x = q[1] as usize;
                let sz = q[2];
                let prev_obs = *active.range(..=x).next_back().unwrap();
                // Must fit entirely in [0, x], and one end has to be at/left of x.
                // Gaps ending at obstacle <= x contribute fully;
                // for the first obstacle > x we can use only [prev_obs, x].
                let best_full = seg.query_max(0, x);
                let tail = (x - prev_obs) as i32;
                rev_ans.push(best_full.max(tail) >= sz);
            }
        }

        rev_ans.reverse();
        rev_ans
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::block_placement_queries::Solution;

    #[test]
    fn test_get_results_1() {
        let queries = [
            [1, 2].to_vec(),
            [2, 3, 3].to_vec(),
            [2, 3, 1].to_vec(),
            [2, 2, 2].to_vec(),
        ]
        .to_vec();
        assert_eq!([false, true, true].to_vec(), Solution::get_results(queries));
    }

    #[test]
    fn test_get_results_2() {
        let queries = [
            [1, 7].to_vec(),
            [2, 7, 6].to_vec(),
            [1, 2].to_vec(),
            [2, 7, 5].to_vec(),
            [2, 7, 6].to_vec(),
        ]
        .to_vec();
        assert_eq!([false, true, true].to_vec(), Solution::get_results(queries));
    }
}
