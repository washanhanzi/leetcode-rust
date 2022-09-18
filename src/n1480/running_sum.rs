//https://leetcode.com/problems/running-sum-of-1d-array/
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![0; nums.len()];
    for (k, &v) in nums.iter().enumerate() {
        if k == 0 {
            res[k] = v;
            continue;
        }
        res[k] = res[k - 1] + v;
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(running_sum(nums), vec![1, 3, 6, 10]);
    }
}
