use std::collections::{HashSet, VecDeque};

pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let mut res = image;
    let target_color = res[sr as usize][sc as usize];
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((sr as usize, sc as usize));
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        res[x][y] = color;
        if y < res[0].len() - 1 && !visited.contains(&(x, y + 1)) && res[x][y + 1] == target_color {
            queue.push_back((x, y + 1));
            visited.insert((x, y + 1));
        }
        if y > 0 && !visited.contains(&(x, y - 1)) && res[x][y - 1] == target_color {
            queue.push_back((x, y - 1));
            visited.insert((x, y - 1));
        }
        if x > 0 && !visited.contains(&(x - 1, y)) && res[x - 1][y] == target_color {
            queue.push_back((x - 1, y));
            visited.insert((x - 1, y));
        }
        if x < res.len() - 1 && !visited.contains(&(x + 1, y)) && res[x + 1][y] == target_color {
            queue.push_back((x + 1, y));
            visited.insert((x + 1, y));
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(
            flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );
    }
}
