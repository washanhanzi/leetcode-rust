//https://leetcode.com/problems/top-k-frequent-words/description/
//TODO time complexity?
use std::collections::HashMap;

pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    let mut freq_map: HashMap<String, usize> = HashMap::with_capacity(words.len());
    for w in words {
        *freq_map.entry(w).or_default() += 1
    }
    let mut to_sort = Vec::new();
    for (k, v) in freq_map {
        to_sort.push((v, k));
    }
    to_sort.sort_by(|l, r| {
        if l.0 != r.0 {
            r.0.cmp(&l.0)
        } else {
            l.1.cmp(&r.1)
        }
    });
    let mut ans = Vec::new();
    for i in 0..k {
        ans.push(to_sort[i as usize].1.clone())
    }
    ans
}
