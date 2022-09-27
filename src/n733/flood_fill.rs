use std::collections::{HashSet, VecDeque};

pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    //target color
    let mut res = image;
    let target_color = res[sr as usize][sc as usize];
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((sr as usize, sc as usize));
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        res[x][y] = color;
        let mut row = res[x].clone();
        left_check_row(
            &mut res,
            &mut visited,
            &mut queue,
            x,
            y,
            target_color,
            color,
        );
        right_check_row(
            &mut res,
            &mut visited,
            &mut queue,
            x,
            y,
            target_color,
            color,
        );
    }

    res
}

fn left_check_row(
    image: &mut Vec<Vec<i32>>,
    visited: &mut HashSet<(usize, usize)>,
    queue: &mut VecDeque<(usize, usize)>,
    x: usize,
    y: usize,
    target_color: i32,
    color: i32,
) {
    let mut recursive = false;
    if y > 0 && !visited.contains(&(x, y - 1)) && image[x][y - 1] == target_color {
        image[x][y - 1] = color;
        visited.insert((x, y - 1));
        recursive = true;
    }
    if x > 0 && !visited.contains(&(x - 1, y)) && image[x - 1][y] == target_color {
        queue.push_back((x - 1, y));
        visited.insert((x - 1, y));
    }
    if x < image.len() - 1 && !visited.contains(&(x + 1, y)) && image[x + 1][y] == target_color {
        queue.push_back((x + 1, y));
        visited.insert((x + 1, y));
    }
    if recursive {
        left_check_row(image, visited, queue, x, y - 1, target_color, color);
    }
}

fn right_check_row(
    image: &mut Vec<Vec<i32>>,
    visited: &mut HashSet<(usize, usize)>,
    queue: &mut VecDeque<(usize, usize)>,
    x: usize,
    y: usize,
    target_color: i32,
    color: i32,
) {
    let mut recursive = false;
    if y < image[0].len() - 1 && !visited.contains(&(x, y + 1)) && image[x][y + 1] == target_color {
        image[x][y + 1] = color;
        visited.insert((x, y + 1));
        recursive = true;
    }
    if x > 0 && !visited.contains(&(x - 1, y)) && image[x - 1][y] == target_color {
        queue.push_back((x - 1, y));
        visited.insert((x - 1, y));
    }
    if x < image.len() - 1 && !visited.contains(&(x + 1, y)) && image[x + 1][y] == target_color {
        queue.push_back((x + 1, y));
        visited.insert((x + 1, y));
    }
    if recursive {
        right_check_row(image, visited, queue, x, y + 1, target_color, color);
    }
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
