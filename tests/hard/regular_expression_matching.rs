// # 10. Regular Expression Matching
// https://leetcode.com/problems/regular-expression-matching/
struct Solution;

impl Solution {
    fn next(p: &str, pos: usize) -> (char, bool) {
        if let Some(c) = p.chars().nth(pos) {
            // check pattern following *
            if let Some(n) = p.chars().nth(pos + 1) {
                if n == '*' {
                    return (c, true);
                }
            }
            // self is *, skip and go next
            if c == '*' {
                return Solution::next(p, pos + 1);
            }
            return (c, false);
        } else {
            // indicate no pattern
            return ('*', false);
        }
    }
    pub fn is_match(s: String, p: String) -> bool {
        let mut pos = 0;
        let mut si = 0;
        let slen = s.len();
        let plen = p.len();
        let ss: Vec<char> = s.chars().collect();
        loop {
            let mut ps = pos;
            let c = ss[si];

            let (pp, any) = Solution::next(&p, pos);

            // no next pattern
            if pp == '*' {
                println!("[a] {} -> {} : {}", &s, &p, false);
                return false;
            }

            if any {
                if pp == '.' {
                    // matched, check greedy
                    if pos + 2 < plen
                        && Solution::is_match(s[si..].to_owned(), p[(pos + 2)..].to_owned())
                    {
                        break;
                    }
                } else {
                    if pp != c {
                        // non-matched, try next pattern
                        pos = pos + 2; // skip next '*' char
                        continue;
                    } else {
                        // check check greedy
                        if pos + 2 < plen
                            && Solution::is_match(s[si..].to_owned(), p[(pos + 2)..].to_owned())
                        {
                            break;
                        }
                    }
                }
            } else {
                if pp == '.' {
                    // matched, next pattern
                    pos = pos + 1
                } else {
                    if pp == c {
                        // matched, next patter
                        pos = pos + 1
                    } else {
                        println!("[b] {} -> {} : {}", &s, &p, false);
                        return false;
                    }
                }
            }

            si = si + 1;
            if si >= slen {
                // check if last pattern
                loop {
                    let (pp, any) = Solution::next(&p, if any { ps + 2 } else { ps + 1 });
                    if pp == '*' {
                        println!("[c] {} -> {} : {}", &s, &p, true);
                        return true;
                    } else {
                        if any {
                            ps = ps + 1;
                        } else {
                            println!("[d] {} -> {} : {}", &s, &p, false);
                            return false;
                        }
                    }
                }
            }
        }
        println!("[e] {} -> {} : {}", &s, &p, true);
        true
    }
}

// from standard solution
struct Answer;

impl Answer {
    pub fn is_match(s: String, p: String) -> bool {
        Answer::find(&s, &p)
    }
    fn find(s: &str, p: &str) -> bool {
        if p.is_empty() {
            return s.is_empty();
        }

        let fist_match = !s.is_empty() && (&p[0..=0] == "." || p[0..=0] == s[0..=0]);

        if p.len() >= 2 && &p[1..=1] == "*" {
            Answer::find(s, &p[2..]) || (fist_match && Answer::find(&s[1..], p))
        } else {
            fist_match && Answer::find(&s[1..], &p[1..])
        }
    }
}

#[test]
fn test_is_match() {
    let s = "aa";
    let p = "a";
    let result = Solution::is_match(s.to_owned(), p.to_owned());
    assert_eq!(false, result, "input s: {}, p: {}", s, p);

    let s = "aa";
    let p = "a*";
    let result = Solution::is_match(s.to_owned(), p.to_owned());
    assert_eq!(true, result, "input s: {}, p: {}", s, p);

    let s = "ab";
    let p = ".*";
    let result = Solution::is_match(s.to_owned(), p.to_owned());
    assert_eq!(true, result, "input s: {}, p: {}", s, p);

    let s = "mississippi";
    let p = "mis*is*ip*.";
    let result = Solution::is_match(s.to_owned(), p.to_owned());
    assert_eq!(true, result, "input s: {}, p: {}", s, p);

    let s = "ab";
    let p = ".*c";
    let result = Solution::is_match(s.to_owned(), p.to_owned());
    assert_eq!(false, result, "input s: {}, p: {}", s, p);

    let s = "aab";
    let p = "c*a*b";
    let result = Solution::is_match(s.to_owned(), p.to_owned());
    assert_eq!(true, result, "input s: {}, p: {}", s, p);

    let s = "aaa";
    let p = "aaaa";
    let result = Solution::is_match(s.to_owned(), p.to_owned());
    assert_eq!(false, result, "input s: {}, p: {}", s, p);

    let s = "aaa";
    let p = "a*a";
    let result = Solution::is_match(s.to_owned(), p.to_owned());
    assert_eq!(true, result, "input s: {}, p: {}", s, p);

    let s = "a";
    let p = "ab*";
    let result = Solution::is_match(s.to_owned(), p.to_owned());
    assert_eq!(true, result, "input s: {}, p: {}", s, p);

    let s = "a";
    let p = "ab*a";
    let result = Solution::is_match(s.to_owned(), p.to_owned());
    assert_eq!(false, result, "input s: {}, p: {}", s, p);

    let s = "bbbba";
    let p = ".*a*a";
    let result = Solution::is_match(s.to_owned(), p.to_owned());
    assert_eq!(true, result, "input s: {}, p: {}", s, p);
}

#[test]
fn test_is_match_answer() {
    let s = "aa";
    let p = "a";
    let result = Answer::is_match(s.to_owned(), p.to_owned());
    assert_eq!(false, result, "input s: {}, p: {}", s, p);

    let s = "aa";
    let p = "a*";
    let result = Answer::is_match(s.to_owned(), p.to_owned());
    assert_eq!(true, result, "input s: {}, p: {}", s, p);

    let s = "ab";
    let p = ".*";
    let result = Answer::is_match(s.to_owned(), p.to_owned());
    assert_eq!(true, result, "input s: {}, p: {}", s, p);

    let s = "mississippi";
    let p = "mis*is*ip*.";
    let result = Answer::is_match(s.to_owned(), p.to_owned());
    assert_eq!(true, result, "input s: {}, p: {}", s, p);

    let s = "ab";
    let p = ".*c";
    let result = Answer::is_match(s.to_owned(), p.to_owned());
    assert_eq!(false, result, "input s: {}, p: {}", s, p);

    let s = "aab";
    let p = "c*a*b";
    let result = Answer::is_match(s.to_owned(), p.to_owned());
    assert_eq!(true, result, "input s: {}, p: {}", s, p);

    let s = "aaa";
    let p = "aaaa";
    let result = Answer::is_match(s.to_owned(), p.to_owned());
    assert_eq!(false, result, "input s: {}, p: {}", s, p);

    let s = "aaa";
    let p = "a*a";
    let result = Answer::is_match(s.to_owned(), p.to_owned());
    assert_eq!(true, result, "input s: {}, p: {}", s, p);

    let s = "a";
    let p = "ab*";
    let result = Answer::is_match(s.to_owned(), p.to_owned());
    assert_eq!(true, result, "input s: {}, p: {}", s, p);

    let s = "a";
    let p = "ab*a";
    let result = Answer::is_match(s.to_owned(), p.to_owned());
    assert_eq!(false, result, "input s: {}, p: {}", s, p);

    let s = "bbbba";
    let p = ".*a*a";
    let result = Answer::is_match(s.to_owned(), p.to_owned());
    assert_eq!(true, result, "input s: {}, p: {}", s, p);
}
