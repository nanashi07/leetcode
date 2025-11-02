// 3479. Fruits Into Baskets III
// https://leetcode.com/problems/fruits-into-baskets-iii/

struct Solution;

impl Solution {
    // https://leetcode.com/problems/fruits-into-baskets-iii/editorial/
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        use std::cmp::max;

        let n = baskets.len();
        let mut baskets = baskets;
        let m = (n as f64).sqrt() as usize;
        let section = (n + m - 1) / m;
        let mut count = 0;
        let mut max_v = vec![0; section];

        for i in 0..n {
            let sec = i / m;
            max_v[sec] = max(max_v[sec], baskets[i])
        }

        for &fruit in &fruits {
            let mut unset = 1;
            for sec in 0..section {
                if max_v[sec] < fruit {
                    continue;
                }

                let mut choose = false;
                max_v[sec] = 0;
                for i in 0..m {
                    let pos = sec * m + i;
                    if pos < n && baskets[pos] >= fruit && !choose {
                        baskets[pos] = 0;
                        choose = true;
                    }
                    if pos < n {
                        max_v[sec] = max(max_v[sec], baskets[pos]);
                    }
                }
                unset = 0;
                break;
            }
            count += unset;
        }
        count
    }

    // Time Limit Exceeded
    pub fn _num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        println!("fruits: {:?}, baskets: {:?}", &fruits, &baskets);

        let mut remained = 0;
        let mut baskets = baskets;

        for i in 0..fruits.len() {
            let fruit = fruits[i];
            let len = baskets.len();
            for j in 0..len {
                if fruit <= baskets[j] {
                    baskets.remove(j);
                    break;
                }
            }
            if len == baskets.len() {
                remained += 1;
            }
        }

        remained
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::fruits_into_baskets_iii::Solution;

    #[test]
    fn test_num_of_unplaced_fruits_1() {
        let fruits = [4, 2, 5].to_vec();
        let baskets = [3, 5, 4].to_vec();
        assert_eq!(1, Solution::num_of_unplaced_fruits(fruits, baskets));
    }

    #[test]
    fn test_num_of_unplaced_fruits_2() {
        let fruits = [3, 6, 1].to_vec();
        let baskets = [6, 4, 7].to_vec();
        assert_eq!(0, Solution::num_of_unplaced_fruits(fruits, baskets));
    }

    #[test]
    fn test_num_of_unplaced_fruits_3() {
        let fruits = [4].to_vec();
        let baskets = [4].to_vec();
        assert_eq!(0, Solution::num_of_unplaced_fruits(fruits, baskets));
    }
}
