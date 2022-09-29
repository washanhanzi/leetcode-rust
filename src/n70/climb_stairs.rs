pub fn climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    let mut dp = Vec::with_capacity(n as usize + 1);
    dp.push(0);
    dp.push(1);
    dp.push(2);
    for i in 3..n + 1 {
        dp.push(dp[i as usize - 1] + dp[i as usize - 2]);
    }
    *dp.last().unwrap()
}
