pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let mut next = 0;
    let mut write_next = false;
    let mut write_zero: usize = usize::MIN;
    for (k, v) in arr.iter_mut().enumerate() {
        if write_next {
            if k == write_zero {
                *v = 0;
                write_zero = usize::MIN;
                continue;
            }
            if *v == 0 {
                write_zero = k + 1;
            }
            let temp = *v;
            *v = next;
            next = temp;
            continue;
        }
        if *v == 0 {
            write_next = true;
            continue;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let mut input = vec![1, 0, 2, 3, 0, 4, 5, 0];
        duplicate_zeros(&mut input);
        assert_eq!(input, vec![1, 0, 0, 2, 3, 0, 0, 4])
    }

    #[test]
    fn case2() {
        let mut input = vec![1, 2, 3];
        duplicate_zeros(&mut input);
        assert_eq!(input, vec![1, 2, 3])
    }

    #[test]
    fn case3() {
        let mut input = vec![0, 4, 1, 0, 0, 8, 0, 0, 3];
        duplicate_zeros(&mut input);
        assert_eq!(input, vec![0, 0, 4, 1, 0, 0, 0, 0, 8])
    }
}
