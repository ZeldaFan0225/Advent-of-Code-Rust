use std::collections::HashSet;
use std::str::from_utf8;

pub fn part_1(input: &[u8]) -> i64 {
    let raw = from_utf8(input).unwrap();
    let mut visited = HashSet::new();
    let mut total = 0;

    let lines: Vec<&str> = raw.split('\n').collect();
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if !visited.contains(&(y, x)) {
                total += calculate_fence_price(y, x, &lines, &mut visited, 1);
            }
        }
    }

    total as i64
}

pub fn part_2(input: &[u8]) -> i64 {
    let raw = from_utf8(input).unwrap();
    let mut visited = HashSet::new();
    let mut total = 0;

    let lines: Vec<&str> = raw.split('\n').collect();
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if !visited.contains(&(y, x)) {
                total += calculate_fence_price(y, x, &lines, &mut visited, 2);
            }
        }
    }

    total as i64
}

fn calculate_fence_price(y: usize, x: usize, grid: &[&str], visited: &mut HashSet<(usize, usize)>, part: u8) -> i64 {
    let letter = grid[y].as_bytes()[x];
    let mut queue = vec![(y, x)];
    let mut area = 0;
    let mut perimeter = 0;

    while let Some((y, x)) = queue.pop() {
        if visited.contains(&(y, x)) {
            continue;
        }
        visited.insert((y, x));
        area += 1;

        let nb_top = grid.get(y.wrapping_sub(1)).and_then(|row| row.as_bytes().get(x));
        let nb_bottom = grid.get(y + 1).and_then(|row| row.as_bytes().get(x));
        let nb_left = grid[y].as_bytes().get(x.wrapping_sub(1));
        let nb_right = grid[y].as_bytes().get(x + 1);

        if nb_top != Some(&letter) {
            if part == 1 || nb_right != Some(&letter) || (nb_right == Some(&letter) && grid.get(y.wrapping_sub(1)).and_then(|row| row.as_bytes().get(x + 1)) == Some(&letter)) {
                perimeter += 1;
            }
        } else {
            queue.push((y.wrapping_sub(1), x));
        }

        if nb_bottom != Some(&letter) {
            if part == 1 || nb_left != Some(&letter) || (nb_left == Some(&letter) && grid.get(y + 1).and_then(|row| row.as_bytes().get(x.wrapping_sub(1))) == Some(&letter)) {
                perimeter += 1;
            }
        } else {
            queue.push((y + 1, x));
        }

        if nb_left != Some(&letter) {
            if part == 1 || nb_top != Some(&letter) || (nb_top == Some(&letter) && grid.get(y.wrapping_sub(1)).and_then(|row| row.as_bytes().get(x.wrapping_sub(1))) == Some(&letter)) {
                perimeter += 1;
            }
        } else {
            queue.push((y, x.wrapping_sub(1)));
        }

        if nb_right != Some(&letter) {
            if part == 1 || nb_bottom != Some(&letter) || (nb_bottom == Some(&letter) && grid.get(y + 1).and_then(|row| row.as_bytes().get(x + 1)) == Some(&letter)) {
                perimeter += 1;
            }
        } else {
            queue.push((y, x + 1));
        }
    }

    (area * perimeter) as i64
}