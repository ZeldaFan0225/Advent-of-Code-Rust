use std::str::from_utf8;
use ahash::{HashMap, HashMapExt};

pub fn part_1(input: &[u8]) -> i64 {
    let mut stones = HashMap::new();

    for num in from_utf8(input).unwrap().split(" ") {
        let num = num.parse::<u64>().unwrap();
        stones.insert(num, stones.get(&num).ok_or(0).unwrap_or(&0) + 1);
    }

    for _ in 0..25 {
        stones = blink_once(&stones);
    }

    count_stones(stones) as i64
}

pub fn part_2(input: &[u8]) -> i64 {
    let mut stones = HashMap::new();

    for num in from_utf8(input).unwrap().split(" ") {
        let num = num.parse::<u64>().unwrap();
        stones.insert(num, stones.get(&num).ok_or(0).unwrap_or(&0) + 1);
    }

    for _ in 0..75 {
        stones = blink_once(&stones);
    }

    count_stones(stones) as i64
}

fn blink_once(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::new();
    for (stone, count) in stones.iter() {
        if *stone == 0 {
            new_stones.insert(1, new_stones.get(&1).ok_or(0).unwrap_or(&0) + count);
            continue;
        }
        let digits = stone.ilog10() + 1;
        match digits {
            _ if digits % 2 == 0 && digits > 1 => {
                let half = digits / 2;
                let left = stone / 10u64.pow(half);
                let right = stone % 10u64.pow(half);
                new_stones.insert(left, new_stones.get(&left).ok_or(0).unwrap_or(&0) + count);
                new_stones.insert(right, new_stones.get(&right).ok_or(0).unwrap_or(&0) + count);
            }
            _ => {
                let new_stone = stone * 2024;
                new_stones.insert(new_stone, new_stones.get(&new_stone).ok_or(0).unwrap_or(&0) + count);
            }
        }
    }

    new_stones
}

fn count_stones(stones: HashMap<u64, u64>) -> u64 {
    stones.iter().map(|(_, count)| count).sum()
}