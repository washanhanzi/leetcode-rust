use std::collections::HashMap;

fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

fn fib_with_hash_map(n: i32) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(0, 0);
    map.insert(1, 1);
    match map.get(&n) {
        Some(n) => {
            return *n;
        }
        None => {
            map.insert(n, fib_with_hash_map(n - 1) + fib_with_hash_map(n - 2));
        }
    }
    return *map.get(&n).unwrap();
}

fn fib_with_vec(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut res = Vec::with_capacity(n as usize);
    res.push(0);
    res.push(1);
    for i in 2..n as usize + 1 {
        res.push(res[i - 1] + res[i - 2]);
    }
    *res.last().unwrap()
}
