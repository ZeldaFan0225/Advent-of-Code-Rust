#![allow(clippy::identity_op)]

use std::collections::{HashSet, VecDeque};

pub fn part_1(input: &[u8]) -> i64 {
    let mut width = 0;
    let mut extra_width: usize = 1;
    for i in 0..input.len() {
        if input[i] == b'\n' {
            if input[i - 1] == b'\r' {
                extra_width += 1;
            }
            width = i - 1;
            break;
        }
    }

    let mut result = 0;
    for i in 0..input.len() {
        if input[i] == b'0' {
            let r = get_trailhead_score(input, i, width, extra_width, true);
            result += r;
        }
    }

    result as i64
}

pub fn part_2(input: &[u8]) -> i64 {
    let mut width = 0;
    let mut extra_width: usize = 1;
    for i in 0..input.len() {
        if input[i] == b'\n' {
            if input[i - 1] == b'\r' {
                extra_width += 1;
            }
            width = i - 1;
            break;
        }
    }

    let mut result = 0;
    for i in 0..input.len() {
        if input[i] == b'0' {
            let r = get_trailhead_score(input, i, width, extra_width, false);
            result += r;
        }
    }

    result as i64
}

fn get_trailhead_score(grid: &[u8], start_index: usize, width: usize, extra_width: usize, unique_trails: bool) -> u32 {
    let mut trail_ends = HashSet::<usize>::new();
    let mut explore_next= VecDeque::new();
    explore_next.push_back(start_index);
    let mut result = 0;

    while let Some(index) = explore_next.pop_front() {
        let value = grid[index];
        match value {
            b'\r' | b'\n' => {
                continue;
            }
            b'9' => {
                if unique_trails {
                    if trail_ends.contains(&index) {
                        continue;
                    }
                    trail_ends.insert(index);
                }
                result += 1;
                continue;
            }
            _ => { }
        }

        if index >= width + extra_width {
            let top_index = index - width - extra_width;
            if grid[top_index] == value + 1 {
                explore_next.push_back(top_index);
            }
        }

        let right_index = index + 1;
        let bottom_index = index + width + extra_width;
        let left_index = index - 1;

        if right_index < grid.len() && (right_index % (width + extra_width)) < width && grid[right_index] == value + 1 {
            explore_next.push_back(right_index);
        }
        if bottom_index < grid.len() && grid[bottom_index] == value + 1 {
            explore_next.push_back(bottom_index);
        }
        if index % (width + extra_width) != 0 && grid[left_index] == value + 1 {
            explore_next.push_back(left_index);
        }
    }

    result
}