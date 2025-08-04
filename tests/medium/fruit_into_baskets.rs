// # 904. Fruit Into Baskets
// https://leetcode.com/problems/fruit-into-baskets/description/

struct Solution;

impl Solution {
    // Time Limit Exceeded
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        println!("fruits: {:?}", &fruits);

        if fruits.len() < 3 {
            return fruits.len() as i32;
        }

        let mut max = 0;
        let mut i = 0;

        while i < fruits.len() - 1 {
            let mut a = fruits[i];
            let mut b = fruits[i];
            let mut j = i + 1;
            let mut last_change = j;
            while j < fruits.len() {
                let c = fruits[j];
                if c == a || c == b {
                    if b == c {
                        // nothing
                    } else {
                        a = b;
                        b = c;
                        last_change = j;
                    }
                } else {
                    if a == b {
                        b = c;
                        last_change = j;
                    } else {
                        // stop here
                        break;
                    }
                }
                j += 1;
            }

            max = max.max(j - i);

            i = last_change;
        }

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::fruit_into_baskets::Solution;

    #[test]
    fn test_total_fruit_1() {
        let fruits = [1, 2, 1].to_vec();
        assert_eq!(3, Solution::total_fruit(fruits));
    }

    #[test]
    fn test_total_fruit_2() {
        let fruits = [0, 1, 2, 2].to_vec();
        assert_eq!(3, Solution::total_fruit(fruits));
    }

    #[test]
    fn test_total_fruit_3() {
        let fruits = [1, 2, 3, 2, 2].to_vec();
        assert_eq!(4, Solution::total_fruit(fruits));
    }

    #[test]
    fn test_total_fruit_4() {
        let fruits = [3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4].to_vec();
        assert_eq!(5, Solution::total_fruit(fruits));
    }

    #[test]
    fn test_total_fruit_5() {
        let fruits = [0, 1, 1, 2].to_vec();
        assert_eq!(3, Solution::total_fruit(fruits));
    }

    #[test]
    fn test_total_fruit_6() {
        let fruits = [0, 1, 6, 6, 4, 4, 6].to_vec();
        assert_eq!(5, Solution::total_fruit(fruits));
    }

    #[test]
    fn test_total_fruit_7() {
        let fruits = [6, 2, 1, 1, 3, 6, 6].to_vec();
        assert_eq!(3, Solution::total_fruit(fruits));
    }
}
