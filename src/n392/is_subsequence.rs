//https://leetcode.com/problems/is-subsequence/
//TODO recursive and dynamic programming
//time complexity: O(N), space complexity: O(N)
//chars().nth(index) is a linear operation, when doing the operation in a loop, it ends up with a quadratic time complexity
pub fn is_subsequence_naive(s: String, t: String) -> bool {
    if s.is_empty() {
        return true;
    }
    let mut arr: Vec<char> = s.chars().collect();
    let mut test = arr.pop().unwrap();
    for d in t.chars().rev() {
        if d == test {
            match arr.pop() {
                Some(v) => test = v,
                _ => return true,
            }
        }
    }
    false
}
