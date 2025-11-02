// 1865. Finding Pairs With a Certain Sum
// https://leetcode.com/problems/finding-pairs-with-a-certain-sum/

use std::collections::HashMap;

struct FindSumPairs {
    hash_nums1: HashMap<i32, i32>,
    hash_nums2: HashMap<i32, i32>,
    nums2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut hash_nums1 = HashMap::new();
        let mut hash_nums2 = HashMap::new();
        for x in nums1 {
            Self::put(&mut hash_nums1, x, 1)
        }
        for x in &nums2 {
            Self::put(&mut hash_nums2, *x, 1)
        }

        Self {
            hash_nums1,
            hash_nums2,
            nums2,
        }
    }

    fn put(map: &mut HashMap<i32, i32>, key: i32, incremental: i32) {
        if let Some(value) = map.get_mut(&key) {
            *value += incremental;
        } else {
            map.insert(key, incremental);
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        let v = self.nums2[index as usize];
        Self::put(&mut self.hash_nums2, v, -1);
        Self::put(&mut self.hash_nums2, v + val, 1);
        self.nums2[index as usize] += val;
    }

    fn count(&self, tot: i32) -> i32 {
        self.hash_nums1.iter().fold(0, |acc, (&k1, &v1)| {
            acc + if let Some(&v2) = self.hash_nums2.get(&(tot - k1)) {
                v2 * v1
            } else {
                0
            }
        })
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */
#[cfg(test)]
mod tests {
    use crate::medium::finding_pairs_with_a_certain_sum::FindSumPairs;

    #[test]
    fn test_find_sum_pairs() {
        // ["FindSumPairs", "count", "add", "count", "count", "add", "add", "count"]
        // [[[1, 1, 2, 2, 2, 3], [1, 4, 5, 2, 5, 4]], [7], [3, 2], [8], [4], [0, 1], [1, 1], [7]]
        // output:
        // [null, 8, null, 2, 1, null, null, 11]

        let mut results: Vec<i32> = Vec::new();

        let actions = [
            "FindSumPairs",
            "count",
            "add",
            "count",
            "count",
            "add",
            "add",
            "count",
        ]
        .to_vec();
        let args = [
            // [[1, 1, 2, 2, 2, 3], [1, 4, 5, 2, 5, 4]],
            [7].to_vec(),
            [3, 2].to_vec(),
            [8].to_vec(),
            [4].to_vec(),
            [0, 1].to_vec(),
            [1, 1].to_vec(),
            [7].to_vec(),
        ];

        let mut find_sum_pairs =
            FindSumPairs::new([1, 1, 2, 2, 2, 3].to_vec(), [1, 4, 5, 2, 5, 4].to_vec());

        for i in 1..actions.len() {
            match actions[i] {
                "count" => {
                    results.push(find_sum_pairs.count(args[i - 1][0]));
                }
                "add" => {
                    find_sum_pairs.add(args[i - 1][0], args[i - 1][1]);
                }
                _ => {}
            }
        }

        assert_eq!([8, 2, 1, 11].to_vec(), results);
    }
}
