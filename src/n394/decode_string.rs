pub fn decode_string(s: String) -> String {
    let mut count_arr: Vec<usize> = Vec::with_capacity(s.len());
    let mut str_arr: Vec<String> = Vec::with_capacity(s.len());
    let mut cur_str = String::new();
    let mut k = 0;
    for c in s.chars() {
        if c.is_numeric() {
            k = k * 10 + (c as u8 - b'0') as usize
        } else if c == '[' {
            count_arr.push(k);
            str_arr.push(cur_str.clone());
            cur_str.clear();
            k = 0;
        } else if c == ']' {
            let mut decoded_str = str_arr.pop().unwrap();
            let cur_k = count_arr.pop().unwrap();
            for _ in 0..cur_k {
                decoded_str = decoded_str + &cur_str;
            }
            cur_str = decoded_str;
        } else {
            cur_str.push(c);
        }
    }
    cur_str
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(
            decode_string("3[a]2[bc]".to_string()),
            "aaabcbc".to_string()
        );
    }
}
