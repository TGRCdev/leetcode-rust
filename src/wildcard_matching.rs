use std::iter::Peekable;

#[derive(Debug)]
enum Pattern<'a> {
    Fragment(&'a str),
    MultiWild(usize),
    AnyWild,
}

pub struct WildcardMatching;

impl WildcardMatching {
    pub fn is_match(s: String, p: String) -> bool {
        // Split pattern into Pattern enums
        let mut pattern: Vec<Pattern> = Vec::new();
        let mut pattern_iter = p.chars().enumerate().peekable();
        while let Some((index, next_char)) = pattern_iter.next() {
            match next_char {
                '*' => {
                    // Truncate extra wildcards
                    while let Some((_, next_char)) = pattern_iter.peek() {
                        if matches!(next_char, '*') {
                            pattern_iter.next();
                        }
                        else {
                            break;
                        }
                    }
                    pattern.push(Pattern::AnyWild);
                },
                '?' => {
                    // Combine multiple question wildcards
                    let mut wildcard_len = 1;
                    while let Some((_, next_char)) = pattern_iter.peek() {
                        if matches!(next_char, '?') {
                            wildcard_len += 1;
                            pattern_iter.next();
                        }
                        else {
                            break;
                        }
                    }
                    pattern.push(Pattern::MultiWild(wildcard_len));
                },
                _ => {
                    // Construct a Fragment
                    let mut end_index = index+1;
                    while let Some((index, next_char)) = pattern_iter.peek() {
                        if !matches!(next_char, '?' | '*') {
                            end_index = index+1;
                            pattern_iter.next();
                        }
                        else {
                            break;
                        }
                    }
                    pattern.push(Pattern::Fragment(&p[index..end_index]));
                }
            }
        }

        //println!("Pattern: {pattern:?}");

        // Step through s
        let mut string_idx = 0;
        let mut rewind_iter: Option<(Peekable<_>, Option<usize>)> = None;
        let mut pattern_iter = pattern.iter().peekable();
        let mut last_fragment: &Pattern<'_> = pattern_iter.peek().unwrap();
        loop {
            let mut frag = pattern_iter.next();
            if frag.is_none() {
                if matches!(last_fragment, Pattern::AnyWild) {
                    return true;
                }
                else if string_idx < s.len() {
                    // There is still unprocessed string
                    // Attempt to rewind to wildcard
                    if let Some((rewind_iter, skip_len)) = &rewind_iter {
                        pattern_iter = rewind_iter.clone();
                        if let Some(skip_len) = skip_len {
                            string_idx += skip_len;
                        }
                        frag = pattern_iter.next();
                    }
                    else {
                        return false;
                    }
                }
                else {
                    return true;
                }
            }
            let frag = frag.unwrap();
            last_fragment = frag;

            match frag {
                &Pattern::Fragment(frag) => {
                    if string_idx + frag.len() > s.len() {
                        return false;
                    }

                    if frag != &s[string_idx..string_idx+frag.len()] {
                        // If the fragment doesn't match, assume we are still matching
                        // the last encountered wildcard
                        if let Some((rewind_iter, skip_len)) = &rewind_iter {
                            pattern_iter = rewind_iter.clone();
                            if let Some(skip_len) = skip_len {
                                string_idx += skip_len;
                            }
                            string_idx += 1;
                        }
                        else {
                            // There was no wildcard, so we cant match anymore
                            return false;
                        }
                    }
                    else {
                        string_idx += frag.len();
                    }
                },
                &Pattern::MultiWild(len) => {
                    rewind_iter = Some((pattern_iter.clone(), Some(len)));
                    string_idx += 1;
                },
                Pattern::AnyWild => {
                    rewind_iter = Some((pattern_iter.clone(), None));
                },
            }

            if string_idx > s.len() {
                return pattern_iter.peek().is_none();
            }
        }
    }
}

#[test]
fn wildcard_test() {
    fn wildcard_test(string: &str, pattern: &str) -> bool {
        let result = WildcardMatching::is_match(string.to_string(), pattern.to_string());
        println!("({string}) & ({pattern}) = {result}");
        result
    }
    wildcard_test("aa", "abab*aabb*?a??a");
    wildcard_test("aaaaaaa", "a*");
    wildcard_test("aaaaaaa", "a*a");
    wildcard_test("aaaabaaaaab", "a*b");
    wildcard_test("adceb", "*a*b");
    wildcard_test("", "*****");
    wildcard_test("abcabczzzde", "*abc???de*");
}