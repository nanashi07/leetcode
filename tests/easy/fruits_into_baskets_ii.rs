// 3477. Fruits Into Baskets II
// https://leetcode.com/problems/fruits-into-baskets-ii/

struct Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        println!("fruits: {:?}, baskets: {:?}", &fruits, &baskets);

        let mut remains = 0;
        let mut baskets = baskets;

        for i in 0..fruits.len() {
            let fruit = fruits[i];
            let len = (&baskets).len();
            for j in 0..len {
                if fruit <= baskets[j] {
                    baskets.remove(j);
                    break;
                }
            }

            if len == baskets.len() {
                remains += 1;
            }
        }

        remains
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::fruits_into_baskets_ii::Solution;

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
}
