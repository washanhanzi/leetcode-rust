use std::cmp::max;
use std::collections::HashMap;

pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut l = 0;
    let mut longest_len = 0;
    let mut frequency: HashMap<char, usize> = HashMap::with_capacity(s.len());
    let mut map: HashMap<usize, char> = HashMap::with_capacity(s.len());

    for (i, c) in s.chars().enumerate() {
        map.insert(i, c);
        *frequency.entry(c).or_insert(0) += 1;

        let max_frequency = frequency.values().max().unwrap();

        // If `window_length - max_frequency <= k`, there are 0 or more replacements
        // can be made so we can continue widen the window by incrementing `r`, and
        // update the longest length. Otherwise, slide the window by incrementing `l`.
        if i - l + 1 - max_frequency > k as usize {
            // update the frequency of the character that is about to move out of the
            // window.
            *frequency.entry(*map.get(&l).unwrap()).or_default() -= 1;
            l += 1;
        }

        longest_len = max(i - l + 1, longest_len);
    }

    longest_len as i32
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(character_replacement("ABAA".to_string(), 0), 2);
    }
}
