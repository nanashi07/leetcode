// 22. Generate Parentheses
// https://leetcode.com/problems/generate-parentheses/
struct Solution;

impl Solution {
    // wrong answer
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut r: std::collections::HashSet<String> = std::collections::HashSet::new();

        match n {
            0 => {}
            1 => {
                r.insert("()".to_owned());
            }
            _ => {
                let tmp = Solution::generate_parenthesis(n - 1);

                for ele in tmp {
                    let s = ele.as_str();
                    let s1 = String::new() + s + "()";
                    let s2 = String::new() + "()" + s;
                    let s3 = String::new() + "(" + s + ")";

                    println!("s: {}", s);
                    println!("s1: {}", s1);
                    println!("s2: {}", s2);
                    println!("s3: {}", s3);

                    r.insert(s3);
                    r.insert(s2);
                    r.insert(s1);
                }
                println!("r = {:?}", &r);
            }
        }

        // ugly approach
        r.into_iter().collect()
    }

    // https://leetcode.com/problems/generate-parentheses/discuss/10100/Easy-to-understand-Java-backtracking-solution
    pub fn generate_parenthesis_from_discussion(n: i32) -> Vec<String> {
        let mut list = Vec::new();
        Solution::backtrack(&mut list, String::new(), 0, 0, n);
        list
    }

    fn backtrack(list: &mut Vec<String>, str: String, open: i32, close: i32, max: i32) {
        println!("o={}, c={}, {}", open, close, &str);
        if str.len() == max as usize * 2 {
            list.push(str.clone());
        }

        if open < max {
            Solution::backtrack(list, str.clone() + "(", open + 1, close, max);
        }

        if close < open {
            Solution::backtrack(list, str.clone() + ")", open, close + 1, max);
        }
    }
}

#[test]
fn test_generate_parenthesis() {
    let n = 3;
    let mut result = Solution::generate_parenthesis(n);
    let mut expected: Vec<String> = ["((()))", "(()())", "(())()", "()(())", "()()()"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    result.sort();
    expected.sort();
    assert_eq!(expected, result, "n = {}", n);

    let n = 1;
    let mut result = Solution::generate_parenthesis(n);
    let mut expected: Vec<String> = ["()"].iter().map(|s| s.to_string()).collect();
    result.sort();
    expected.sort();
    assert_eq!(expected, result, "n = {}", n);

    let n = 4;
    let mut result = Solution::generate_parenthesis(n);
    let mut expected: Vec<String> = [
        "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
        "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    result.sort();
    expected.sort();
    // assert_eq!(expected, result, "n = {}", n);
}

#[test]
fn test_generate_parenthesis_from_discussion() {
    let n = 3;
    let mut result = Solution::generate_parenthesis_from_discussion(n);
    let mut expected: Vec<String> = ["((()))", "(()())", "(())()", "()(())", "()()()"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    result.sort();
    expected.sort();
    assert_eq!(expected, result, "n = {}", n);

    let n = 1;
    let mut result = Solution::generate_parenthesis_from_discussion(n);
    let mut expected: Vec<String> = ["()"].iter().map(|s| s.to_string()).collect();
    result.sort();
    expected.sort();
    assert_eq!(expected, result, "n = {}", n);

    let n = 4;
    let mut result = Solution::generate_parenthesis_from_discussion(n);
    let mut expected: Vec<String> = [
        "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
        "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    result.sort();
    expected.sort();
    assert_eq!(expected, result, "n = {}", n);
}
