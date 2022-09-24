//https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
//For every element, we are calculating the difference between that element and the minimum of all the values before that element
//and we are updating the maximum profit if the difference thus found is greater than the current maximum profit.
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut lowest = i32::MAX;
    for v in prices {
        if v - lowest > max_profit {
            max_profit = v - lowest;
        }
        if v < lowest {
            lowest = v;
        }
    }
    max_profit
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let input = vec![7, 1, 5, 3, 6, 4];
        let output = max_profit(input);
        assert_eq!(output, 5);
    }
    #[test]
    fn case2() {
        let input = vec![7, 6, 4, 3, 1];
        let output = max_profit(input);
        assert_eq!(output, 0);
    }
}
