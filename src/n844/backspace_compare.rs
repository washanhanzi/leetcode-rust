pub fn backspace_compare(s: String, t: String) -> bool {
    let mut s_arr = Vec::with_capacity(s.len());
    let mut t_arr = Vec::with_capacity(t.len());
    for ss in s.chars() {
        if ss == '#' {
            s_arr.pop();
        } else {
            s_arr.push(ss);
        }
    }
    for tt in t.chars() {
        if tt == '#' {
            t_arr.pop();
        } else {
            t_arr.push(tt);
        }
    }
    s_arr == t_arr
}
