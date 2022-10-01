use std::collections::HashMap;

//https://leetcode.com/problems/min-cost-climbing-stairs/
pub fn min_cost_climbing_stairs_bottom_up(cost: Vec<i32>) -> i32 {
    //the problem to find min cost to climb the starts is to find the the min cost of n-1 and n-2
    //cuz we can climb from the n-1 or n-2 to top
    let n = cost.len();
    //we need a new vec to store the min cost to climb to the ith stair
    let mut res: Vec<i32> = Vec::with_capacity(n + 1);
    //the min cost to climb 0th stair is 0
    res.push(0);
    //the min cost to climb 1st stair is 0 since we can climb from 0th or 1st stair
    res.push(0);
    //then we can calculate the min cost to climb the 2ed stair all the way to n+1 stair
    for i in 2..n + 1 {
        if res[i - 1] + cost[i - 1] < res[i - 2] + cost[i - 2] {
            res.push(res[i - 1] + cost[i - 1]);
        } else {
            res.push(res[i - 2] + cost[i - 2]);
        }
    }
    *res.last().unwrap()
}

pub fn min_cost_climbing_stairs_top_down(cost: Vec<i32>) -> i32 {
    //we can also solve the problem from top with recursion, but we need to store the intermediate results to avoid repetitive calculation
    //rust default hashmap use a cryptography hash function
    let mut memo: HashMap<usize, i32> = HashMap::new();
    memo.insert(0, 0);
    memo.insert(1, 0);
    return recursive(cost.len(), &cost, &mut memo);
}

fn recursive(n: usize, cost: &Vec<i32>, memo: &mut HashMap<usize, i32>) -> i32 {
    if memo.contains_key(&n) {
        return *memo.get(&n).unwrap();
    }
    let one_stair_down = cost[n - 1] + recursive(n - 1, cost, memo);
    let two_stair_down = cost[n - 2] + recursive(n - 2, cost, memo);
    if one_stair_down < two_stair_down {
        memo.insert(n, one_stair_down);
    } else {
        memo.insert(n, two_stair_down);
    }
    return *memo.get(&n).unwrap();
}
