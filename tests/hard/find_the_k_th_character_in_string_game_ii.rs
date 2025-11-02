// 3307. Find the K-th Character in String Game II
// https://leetcode.com/problems/find-the-k-th-character-in-string-game-ii/

struct Solution;

impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut pow = operations.len() as i64;
        let mut position = k;
        let mut char_move: i32 = 0;

        let get_first_half_max_value = |position: i64| {
            let mut p = position.clone();
            let mut c: i64 = 0;
            while p > 1 {
                p = p >> 1;
                c = c + 1;
            }
            c
        };

        while pow > 0 {
            println!(
                "pow: {}, position: {}, previous pow: {}, previous max: {}",
                pow,
                position,
                get_first_half_max_value(position),
                2_u64.pow(get_first_half_max_value(position) as u32)
            );
            if get_first_half_max_value(position) >= pow - 1 && position > 2_i64.pow(pow as u32 - 1)
            {
                // in second-half word range
                if operations[pow as usize - 1] == 0 {
                    // char not changed
                } else {
                    // char changed
                    char_move -= 1;
                    char_move %= 26;
                }
                // find the previous position
                position = position - 2_i64.pow(pow as u32 - 1);
                println!("previous position: {}, char_move: {}", position, char_move);
            }
            pow -= 1;
        }

        let a = 'a' as u32;
        char::from_u32(a + (-1 * char_move) as u32).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_the_k_th_character_in_string_game_ii::Solution;

    #[test]
    fn test_kth_character_1() {
        let k = 5;
        let operations = [0, 0, 0].to_vec();
        assert_eq!('a', Solution::kth_character(k, operations));
    }

    #[test]
    fn test_kth_character_2() {
        let k = 10;
        let operations = [0, 1, 0, 1].to_vec();
        assert_eq!('b', Solution::kth_character(k, operations));
    }

    #[test]
    fn test_kth_character_3() {
        let k = 26170712;
        let operations = [
            0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0,
        ]
        .to_vec();
        assert_eq!('h', Solution::kth_character(k, operations));
    }

    #[test]
    fn test_kth_character_4() {
        let k = 26170712;
        let operations = [
            0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0,
            1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0,
            0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0,
            0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0,
        ]
        .to_vec();
        assert_eq!('f', Solution::kth_character(k, operations));
    }

    #[test]
    fn test_kth_character_5() {
        let k = 100000000000000;
        let operations = [
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ]
        .to_vec();
        assert_eq!('e', Solution::kth_character(k, operations));
    }
}
