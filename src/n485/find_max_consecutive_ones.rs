//https://leetcode.com/problems/max-consecutive-ones/
fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let (mut cur, mut max) = (0, 0);
    for v in nums {
        if v == 1 {
            cur = cur + 1;
            if cur > max {
                max = cur
            }
        } else {
            cur = 0;
        }
    }
    max
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let nums = vec![1, 1, 0, 1, 1, 1];
        assert_eq!(find_max_consecutive_ones(nums), 3);
    }

    #[test]
    fn case2() {
        let nums = vec![1, 0, 1, 1, 0, 1];
        assert_eq!(find_max_consecutive_ones(nums), 2);
    }
}
