// # 3304. Find the K-th Character in String Game I
// https://leetcode.com/problems/find-the-k-th-character-in-string-game-i/

struct Solution;

impl Solution {
    pub fn kth_character(k: i32) -> char {
        let mut workd = vec![0];
        loop {
            workd.extend(workd.iter().map(|x| *x + 1).collect::<Vec<i32>>());
            if workd.len() as i32 > k {
                break;
            }
        }

        let n = (workd[k as usize - 1] as u32) % 26;
        char::from_u32(n + ('a' as u32)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_the_k_th_character_in_string_game_i::Solution;

    #[test]
    fn test_kth_character_1() {
        let k = 5;
        assert_eq!('b', Solution::kth_character(k));
    }

    #[test]
    fn test_kth_character_2() {
        let k = 10;
        assert_eq!('c', Solution::kth_character(k));
    }
}
