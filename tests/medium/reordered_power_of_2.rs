// 869. Reordered Power of 2
// https://leetcode.com/problems/reordered-power-of-2/

struct Solution;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let len = n.to_string().len();
        println!("n: {}, len: {}", n, len);

        // scope the range of numbers
        let mut min = 10_i64.pow(len as u32 - 1);
        let mut max = 10_i64.pow(len as u32);

        println!("max: {}, min: {}", max, min);

        // scope the pow of 10
        let mut max_p: i32 = -1;
        let mut min_p: i32 = -1;

        while min > 0 {
            min = min >> 1;
            min_p += 1;
        }

        while max > 0 {
            max = max >> 1;
            max_p += 1;
        }

        println!("max_p: {}, min_p: {}", max_p, min_p);

        // convert to vec for compare numbers
        let to_vec = |n: i64| {
            let mut p = n;
            let mut v: Vec<i64> = vec![];
            while p >= 10 {
                v.push(p % 10);
                p = p / 10;
            }
            v.push(p);
            v.sort_unstable();
            v
        };

        // compare all possible numbers
        let vo = to_vec(n as i64);
        for i in min_p..=max_p {
            let pn = 2_i64.pow(i as u32);
            let vp = to_vec(pn);
            println!("compare {:?} : {:?}", &vo, &vp);
            if vp.eq(&vo) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::reordered_power_of_2::Solution;

    #[test]
    fn test_reordered_power_of2_1() {
        let n = 1;
        assert_eq!(true, Solution::reordered_power_of2(n));
    }

    #[test]
    fn test_reordered_power_of2_2() {
        let n = 10;
        assert_eq!(false, Solution::reordered_power_of2(n));
    }

    #[test]
    fn test_reordered_power_of2_3() {
        let n = 124387842;
        assert_eq!(false, Solution::reordered_power_of2(n));
    }

    #[test]
    fn test_reordered_power_of2_4() {
        let n = 4150876;
        assert_eq!(true, Solution::reordered_power_of2(n));
    }

    #[test]
    fn test_reordered_power_of2_5() {
        let n = 218;
        assert_eq!(true, Solution::reordered_power_of2(n));
    }
}
