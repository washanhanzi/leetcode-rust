use std::collections::HashMap;

pub fn longest_palindrome(s: String) -> i32 {
    //initialize a dict
    let mut dict: HashMap<char, i32> = HashMap::with_capacity(s.chars().count());
    //count every char in the string
    for v in s.chars() {
        let c = dict.entry(v).or_insert(0);
        *c += 1;
    }
    //loop through the dict to calculate the final palindrome length
    let mut res = 0;
    for vv in dict.into_values() {
        res += vv / 2 * 2;
    }
    //if the calculated length is lower than the original s, it means a unique center exist
    //return the calculated length + 1
    return if (res as usize) < s.chars().count() {
        res + 1
    } else {
        res
    };
}
