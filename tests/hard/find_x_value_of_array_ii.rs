// 3525. Find X Value of Array II
// https://leetcode.com/problems/find-x-value-of-array-ii/

struct Solution;

/// Segment node: `cnt[r]` = number of non-empty prefixes with product ≡ r (mod k),
/// `prod` = whole segment product mod k. Concatenation is an associative monoid.
#[derive(Clone, Copy)]
struct Node {
    cnt: [i32; 5],
    prod: usize,
}

impl Solution {
    fn merge(a: &Node, b: &Node, k: usize) -> Node {
        let mut cnt = a.cnt;
        for q in 0..k {
            if b.cnt[q] != 0 {
                cnt[(a.prod * q) % k] += b.cnt[q];
            }
        }
        Node {
            cnt,
            prod: (a.prod * b.prod) % k,
        }
    }

    fn leaf(v: i32, k: usize) -> Node {
        let mut cnt = [0; 5];
        let p = (v % k as i32) as usize;
        cnt[p] = 1;
        Node { cnt, prod: p }
    }

    pub fn result_array(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let id = Node {
            cnt: [0; 5],
            prod: 1 % k,
        };
        let size = n.next_power_of_two();
        let mut tree = vec![id; 2 * size];
        for (i, v) in nums.iter().enumerate() {
            tree[size + i] = Self::leaf(*v, k);
        }
        for i in (1..size).rev() {
            tree[i] = Self::merge(&tree[2 * i], &tree[2 * i + 1], k);
        }

        queries
            .iter()
            .map(|q| {
                let (idx, value, start, x) = (q[0] as usize, q[1], q[2] as usize, q[3] as usize);

                let mut i = size + idx;
                tree[i] = Self::leaf(value, k);
                while i > 1 {
                    i >>= 1;
                    tree[i] = Self::merge(&tree[2 * i], &tree[2 * i + 1], k);
                }

                let (mut left, mut right) = (id, id);
                let (mut l, mut r) = (size + start, size + n);
                while l < r {
                    if l & 1 == 1 {
                        left = Self::merge(&left, &tree[l], k);
                        l += 1;
                    }
                    if r & 1 == 1 {
                        r -= 1;
                        right = Self::merge(&tree[r], &right, k);
                    }
                    l >>= 1;
                    r >>= 1;
                }
                Self::merge(&left, &right, k).cnt[x]
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_x_value_of_array_ii::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_result_array_1() {
        let nums = [1, 2, 3, 4, 5].to_vec();
        let k = 3;
        let queries = to_vec2d([[2, 2, 0, 2], [3, 3, 3, 0], [0, 1, 0, 1]]);
        assert_eq!([2, 2, 2].to_vec(), Solution::result_array(nums, k, queries));
    }

    #[test]
    fn test_result_array_2() {
        let nums = [1, 2, 4, 8, 16, 32].to_vec();
        let k = 4;
        let queries = to_vec2d([[0, 2, 0, 2], [0, 2, 0, 1]]);
        assert_eq!([1, 0].to_vec(), Solution::result_array(nums, k, queries));
    }

    #[test]
    fn test_result_array_3() {
        let nums = [1, 1, 2, 1, 1].to_vec();
        let k = 2;
        let queries = to_vec2d([[2, 1, 0, 1]]);
        assert_eq!([5].to_vec(), Solution::result_array(nums, k, queries));
    }
}
