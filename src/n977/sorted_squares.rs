//https://leetcode.com/problems/squares-of-a-sorted-array/
//use the two pointer solution
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; nums.len()];
    let len = nums.len();
    let mut l = 0;
    let mut r = len - 1;
    for i in (0..len).rev() {
        if i == 0 {
            res[0] = nums[l].pow(2);
            break;
        }
        if nums[l].abs() < nums[r].abs() {
            res[i] = nums[r].pow(2);
            r = r - 1;
        } else {
            res[i] = nums[l].pow(2);
            l = l + 1;
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let nums = vec![-4, -1, 0, 3, 10];
        assert_eq!(sorted_squares(nums), vec![0, 1, 9, 16, 100])
    }
    #[test]
    fn case2() {
        let nums = vec![-7, -3, 2, 3, 11];
        assert_eq!(sorted_squares(nums), vec![4, 9, 9, 49, 121])
    }
}
