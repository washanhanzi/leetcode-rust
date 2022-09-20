//https://leetcode.com/problems/isomorphic-strings/
use std::collections::HashMap;

//character mapping with dictionary
//time complexity: O(N), space complexity: O(1)
fn is_isomorphic(s: String, t: String) -> bool {
    let mut dict: HashMap<char, char> = HashMap::new();
    let mut reverse_dict: HashMap<char, char> = HashMap::new();
    for (ss, tt) in s.chars().zip(t.chars()) {
        //check for s->t mapping
        match dict.get(&ss) {
            Some(&v) => {
                if v != tt {
                    return false;
                }
            }
            _ => {
                dict.insert(ss, tt);
            }
        }
        //check for t->s mapping
        match reverse_dict.get(&tt) {
            Some(&v) => {
                if v != ss {
                    return false;
                }
            }
            _ => {
                reverse_dict.insert(tt, ss);
            }
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestData {
        input: [String; 2],
        output: bool,
    }
    #[test]
    fn test() {
        let data: Vec<TestData> = vec![
            TestData {
                input: [String::from("egg"), String::from("add")],
                output: true,
            },
            TestData {
                input: [String::from("foo"), String::from("bar")],
                output: false,
            },
            TestData {
                input: [String::from("paper"), String::from("title")],
                output: true,
            },
            TestData {
                input: [String::from("badc"), String::from("baba")],
                output: false,
            },
        ];
        for d in data {
            let [s, t] = d.input;
            assert_eq!(is_isomorphic(s, t), d.output)
        }
    }
}
