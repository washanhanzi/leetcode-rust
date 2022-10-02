//https://leetcode.com/problems/last-stone-weight/description/
//TODO bucket sort?
use std::collections::BinaryHeap;

pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut stones = BinaryHeap::from(stones);
    while stones.len() >= 2 {
        let first_largest_stone = stones.pop().unwrap();
        let second_largest_stone = stones.pop().unwrap();
        let diff = first_largest_stone - second_largest_stone;
        if diff != 0 {
            stones.push(diff);
        }
    }
    stones.pop().unwrap_or(0)
}
