use std::collections::HashMap;

pub fn get_hint(secret: String, guess: String) -> String {
    let mut bull: usize = 0;
    let mut cow: usize = 0;
    let mut map: HashMap<char, i32> = HashMap::with_capacity(secret.len());
    for v in secret.chars() {
        *map.entry(v).or_insert(0) += 1;
    }
    for (k, v) in guess.chars().enumerate() {
        if let Some(vv) = secret.chars().nth(k) {
            if map.contains_key(&v) {
                if vv == v {
                    bull += 1;
                    if *map.get(&v).unwrap() <= 0 {
                        cow -= 1;
                    }
                } else {
                    if *map.get(&v).unwrap() > 0 {
                        cow += 1;
                    }
                }
                map.entry(v).and_modify(|e| *e -= 1);
            }
        }
    }
    bull.to_string() + "A" + &cow.to_string() + "B"
}
