// 3577. Count the Number of Computer Unlocking Permutations
// https://leetcode.com/problems/count-the-number-of-computer-unlocking-permutations/

struct Solution;

impl Solution {
    // https://leetcode.com/problems/count-the-number-of-computer-unlocking-permutations/
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        let n = complexity.len();
        for i in 1..n {
            if complexity[i] <= complexity[0] {
                return 0;
            }
        }

        let mut ans: i64 = 1;
        let mod_val: i64 = 1_000_000_007;
        for i in 2..n {
            ans = (ans * i as i64) % mod_val;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_the_number_of_computer_unlocking_permutations::Solution;

    #[test]
    fn test_count_permutations_1() {
        let complexity = [1, 2, 3].to_vec();
        assert_eq!(2, Solution::count_permutations(complexity));
    }

    #[test]
    fn test_count_permutations_2() {
        let complexity = [3, 3, 3, 4, 4, 4].to_vec();
        assert_eq!(0, Solution::count_permutations(complexity));
    }

    #[test]
    fn test_count_permutations_3() {
        let complexity = [2, 204, 482].to_vec();
        assert_eq!(2, Solution::count_permutations(complexity));
    }

    #[test]
    fn test_count_permutations_4() {
        let complexity = [16, 51].to_vec();
        assert_eq!(1, Solution::count_permutations(complexity));
    }

    #[test]
    fn test_count_permutations_5() {
        let complexity = [58, 283, 52].to_vec();
        assert_eq!(0, Solution::count_permutations(complexity));
    }

    #[test]
    fn test_count_permutations_6() {
        let complexity = [59, 40].to_vec();
        assert_eq!(0, Solution::count_permutations(complexity));
    }
}
