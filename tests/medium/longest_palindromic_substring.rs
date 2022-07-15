// # 5. Longest Palindromic Substring
// https://leetcode.com/problems/longest-palindromic-substring/
struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();

        for size in (3..(len + 1)).rev() {
            for i in 0..(len - size + 1) {
                let m = &s[i..(i + size)];
                let n = Solution::palindromic(m);
                if n > 0 {
                    return m.to_owned();
                }
            }
        }

        for i in 0..len {
            if i + 2 <= len && &s[i..(i + 1)] == &s[(i + 1)..(i + 2)] {
                return s[i..(i + 2)].to_owned();
            }
        }

        s[0..1].to_owned()
    }

    fn palindromic(s: &str) -> i32 {
        let len = s.len();

        let mut i: usize = 0;
        let mut j = len - 1;

        loop {
            if s[i..(i + 1)] != s[j..(j + 1)] {
                return -1;
            }
            i = i + 1;
            j = j - 1;

            if i >= j {
                break;
            }
        }

        len as i32
    }
}

#[test]
fn test_longest_palindrome() {
    let input = "babad";
    let result = Solution::longest_palindrome(input.to_owned());
    assert_eq!("bab".to_owned(), result, "input = {}", input);

    let input = "cbbd";
    let result = Solution::longest_palindrome(input.to_owned());
    assert_eq!("bb".to_owned(), result, "input = {}", input);

    let input = "a";
    let result = Solution::longest_palindrome(input.to_owned());
    assert_eq!("a".to_owned(), result, "input = {}", input);

    let input = "ac";
    let result = Solution::longest_palindrome(input.to_owned());
    assert_eq!("a".to_owned(), result, "input = {}", input);

    let input = "abb";
    let result = Solution::longest_palindrome(input.to_owned());
    assert_eq!("bb".to_owned(), result, "input = {}", input);

    let input = "caba";
    let result = Solution::longest_palindrome(input.to_owned());
    assert_eq!("aba".to_owned(), result, "input = {}", input);

    let input = "aacabdkacaa";
    let result = Solution::longest_palindrome(input.to_owned());
    assert_eq!("aca".to_owned(), result, "input = {}", input);

    let input = "jglknendplocymmvwtoxvebkekzfdhykknufqdkntnqvgfbahsljkobhbxkvyictzkqjqydczuxjkgecdyhixdttxfqmgksrkyvopwprsgoszftuhawflzjyuyrujrxluhzjvbflxgcovilthvuihzttzithnsqbdxtafxrfrblulsakrahulwthhbjcslceewxfxtavljpimaqqlcbrdgtgjryjytgxljxtravwdlnrrauxplempnbfeusgtqzjtzshwieutxdytlrrqvyemlyzolhbkzhyfyttevqnfvmpqjngcnazmaagwihxrhmcibyfkccyrqwnzlzqeuenhwlzhbxqxerfifzncimwqsfatudjihtumrtjtggzleovihifxufvwqeimbxvzlxwcsknksogsbwwdlwulnetdysvsfkonggeedtshxqkgbhoscjgpiel";
    let result = Solution::longest_palindrome(input.to_owned());
    assert_eq!("sknks".to_owned(), result, "input = {}", input);

    let input = "ccc";
    let result = Solution::longest_palindrome(input.to_owned());
    assert_eq!("ccc".to_owned(), result, "input = {}", input);
}
