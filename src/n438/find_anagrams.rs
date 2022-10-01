use std::collections::HashMap;

pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut target_string: HashMap<char, usize> = HashMap::with_capacity(26);
    let mut sliding_window: HashMap<char, usize> = HashMap::with_capacity(26);
    for s in p.chars() {
        *target_string.entry(s).or_insert(0) += 1;
    }
    for (i, c) in s.chars().enumerate() {
        *sliding_window.entry(c).or_insert(0) += 1;
        if i >= p.len() {
            let c = s.chars().nth(i - p.len()).unwrap();
            if let Some(v) = sliding_window.get_mut(&c) {
                *v -= 1;
                if *v == 0 {
                    sliding_window.remove(&c);
                }
            }
        }
        if target_string == sliding_window {
            res.push(i as i32 - p.len() as i32 + 1);
        }
    }
    res
}
