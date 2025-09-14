// # 966. Vowel Spellchecker
// https://leetcode.com/problems/vowel-spellchecker/

struct Solution;

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::vowel_spellchecker::Solution;

    #[test]
    fn test_spellchecker_1() {
        let wordlist = ["KiTe", "kite", "hare", "Hare"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let queries = [
            "kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
        let output = [
            "kite", "KiTe", "KiTe", "Hare", "hare", "", "", "KiTe", "", "KiTe",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
        assert_eq!(output, Solution::spellchecker(wordlist, queries));
    }

    #[test]
    fn test_spellchecker_2() {
        let wordlist = ["yellow"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let queries = ["YellOw"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let output = ["yellow"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(output, Solution::spellchecker(wordlist, queries));
    }
}
