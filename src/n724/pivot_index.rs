//https://leetcode.com/problems/find-pivot-index/
fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut right_sum = nums.iter().sum::<i32>();
    let mut left_sum = 0;
    for (k, &v) in nums.iter().enumerate() {
        right_sum -= v;
        if right_sum == left_sum {
            return k as i32;
        }
        left_sum += v;
    }
    -1
}

#[cfg(test)]
mod test {
    use crate::n724::pivot_index::pivot_index;

    #[test]
    fn case1() {
        let input = vec![1, 7, 3, 6, 5, 6];
        assert_eq!(pivot_index(input), 3)
    }

    #[test]
    fn case2() {
        let input = vec![1, 2, 3];
        assert_eq!(pivot_index(input), -1)
    }

    #[test]
    fn case3() {
        let input = vec![2, 1, -1];
        assert_eq!(pivot_index(input), 0)
    }
}
