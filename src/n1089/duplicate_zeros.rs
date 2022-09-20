fn duplicate_zeros(arr: &mut Vec<i32>) {
    let mut count = arr.iter().filter(|&&d| d == 0).count();
    if count > 0 {
        let length = arr.len();
        for i in (0..length).rev() {
            //if we got 0, we move an additional 0 to i+count, and decrease the count
            if arr[i] == 0 {
                if i + count < length {
                    arr[i + count] = arr[i];
                }
                count -= 1;
            }
            if i + count < length {
                arr[i + count] = arr[i]
            }
        }
    }
}

//time complexity: O(N), space complexity: O(N)
fn duplicate_zeros_naive(arr: &mut Vec<i32>) {
    //use a vec to store the values waiting to write into arr
    let mut waiting_seq: Vec<i32> = vec![];
    for v in arr.iter_mut() {
        if waiting_seq.len() != 0 {
            //if the value is 0, we need an additional 0 to write
            if *v == 0 {
                waiting_seq.insert(0, 0);
            }
            //pop the last value from the waiting_seq, write it to arr, and move the iteration value to waiting_seq
            let insert = waiting_seq.pop().unwrap();
            waiting_seq.insert(0, *v);
            *v = insert;
            continue;
        }
        //when the iteration first got an 0, it means we need to use the waiting_seq
        if *v == 0 {
            waiting_seq.insert(0, 0);
            continue;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestData {
        got: Vec<i32>,
        want: Vec<i32>,
    }

    #[test]
    fn test() {
        run_test(duplicate_zeros_naive);
        run_test(duplicate_zeros);
    }

    fn run_test(f: fn(&mut Vec<i32>)) {
        let mut data: Vec<TestData> = vec![
            TestData {
                got: vec![1, 0, 2, 3, 0, 4, 5, 0],
                want: vec![1, 0, 0, 2, 3, 0, 0, 4],
            },
            TestData {
                got: vec![1, 2, 3],
                want: vec![1, 2, 3],
            },
            TestData {
                got: vec![0, 4, 1, 0, 0, 8, 0, 0, 3],
                want: vec![0, 0, 4, 1, 0, 0, 0, 0, 8],
            },
        ];
        for d in data.iter_mut() {
            f(&mut d.got);
            assert_eq!(d.got, d.want);
        }
    }
}
