use std::{collections::HashMap, fs};

fn blink(stone: u64, level: u64, cache: &mut HashMap<[u64; 2], u64>) -> u64 {
    if let Some(res) = cache.get(&[stone, level]) {
        return *res;
    }
    if level == 0 {
        return 1;
    }

    let left;
    let mut right = None;

    let stone_string = stone.to_string();
    if stone == 0 {
        left = 1;
    } else if stone_string.len() % 2 == 0 {
        let (l, r) = stone_string.split_at(stone_string.len() / 2);
        left = l.parse::<u64>().unwrap();
        right = Some(r.parse::<u64>().unwrap());
    } else {
        left = stone * 2024;
    }

    let value = match right {
        Some(r) => blink(left, level - 1, cache) + blink(r, level - 1, cache),
        None => blink(left, level - 1, cache),
    };

    cache.insert([stone, level], value);
    *cache.get(&[stone, level]).unwrap()
}

fn main() {
    let stones = read_file("input");
    let mut cache: HashMap<[u64; 2], u64> = HashMap::new();
    let mut count = 0;

    for stone in stones {
        count += blink(stone, 75, &mut cache)
    }

    println!("Task1 {:?}", count);
}

fn read_file(path: &str) -> Vec<u64> {
    let content = fs::read_to_string(path).expect("Failed to read file");
    content
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty() && *x != "\n")
        .map(|x| x.to_owned().parse::<u64>().unwrap())
        .collect()
}
