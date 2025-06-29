// # 912. Sort an Array
// https://leetcode.com/problems/sort-an-array/

struct Solution;

impl Solution {
    // counting sort (the best performance on leetcode submissions)
    pub fn _sort_array2(mut nums: Vec<i32>) -> Vec<i32> {
        let mut counts = [0; 100_001];

        for n in nums.drain(..) {
            let adjusted_n = (n + 50_000) as usize;

            counts[adjusted_n] += 1;
        }

        for adjusted_n in 0..=100_000 {
            let count = counts[adjusted_n];

            if count > 0 {
                let n = adjusted_n as i32 - 50_000;

                for _ in 0..count {
                    nums.push(n);
                }
            }
        }
        nums
    }

    // insertion sort, submissions
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let len = nums.len();
        println!("nums: {:?}", &nums);

        for i in 1..len {
            let key = nums[i];
            let mut j = i;

            while j > 0 && nums[j - 1] > key {
                println!("a {:?}, key = {}, i = {}, j = {}", &nums, key, i, j);
                nums[j] = nums[j - 1];
                println!("b {:?}, key = {}, i = {}, j = {}", &nums, key, i, j);
                j -= 1;
            }
            nums[j] = key;
            println!("c {:?}, key = {}, i = {}, j = {}", &nums, key, i, j);
        }

        nums
    }

    // bubble sort, too slow
    pub fn _bubble_sort_array(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut nums = nums;

        let mut run = |i| {
            let mut exchanged = false;
            for j in i + 1..len {
                if &nums[i] > &nums[j] {
                    nums.swap(i, j);
                    exchanged = true;
                }
            }
            exchanged
        };

        for i in 0..len {
            while run(i) {}
        }

        nums
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::sort_an_array::Solution;

    #[test]
    fn test_sort_array_1() {
        let nums = vec![5, 2, 3, 1];
        let expected = vec![1, 2, 3, 5];
        assert_eq!(expected, Solution::sort_array(nums));
    }

    #[test]
    fn test_sort_array_2() {
        let nums = vec![5, 1, 1, 2, 0, 0];
        let expected = vec![0, 0, 1, 1, 2, 5];
        assert_eq!(expected, Solution::sort_array(nums));
    }

    #[test]
    fn test_sort_array_3() {
        let nums = vec![9, 8, 7, 6, 5, 1, 1, 2, 0, 0];
        let expected = vec![0, 0, 1, 1, 2, 5, 6, 7, 8, 9];
        assert_eq!(expected, Solution::sort_array(nums));
    }
}
