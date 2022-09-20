//https://leetcode.com/problems/find-numbers-with-even-number-of-digits/
fn find_numbers(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for v in nums {
        //let digits = v.to_string().len(); will get 0ms runtime
        let digits = (v.abs() as f64 + 0.1).log10().ceil() as u32;
        if digits % 2 == 0 {
            res = res + 1
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let nums = vec![12, 345, 2, 6, 7896];
        assert_eq!(find_numbers(nums), 2)
    }

    #[test]
    fn case2() {
        let nums = vec![555, 901, 482, 1771];
        assert_eq!(find_numbers(nums), 1)
    }

    #[test]
    fn case3() {
        let nums = vec![100000];
        assert_eq!(find_numbers(nums), 1)
    }

    #[test]
    fn max_i32() {
        let nums = vec![2147483647];
        assert_eq!(find_numbers(nums), 1)
    }

    #[test]
    fn negative_number() {
        let nums = vec![-1000];
        assert_eq!(find_numbers(nums), 1)
    }
}
