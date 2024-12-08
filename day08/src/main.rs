use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let lines = read_file("input");
    let mut frequencies: HashMap<char, Vec<[i64; 2]>> = HashMap::new();
    let mut results: HashSet<[i64; 2]> = HashSet::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c != '.' {
                let p = [i as i64, j as i64];
                frequencies
                    .entry(*c)
                    .and_modify(|pos| pos.push(p))
                    .or_insert(vec![p]);
            }
        }
    }

    for positions in frequencies.values() {
        for (i, pos_1) in positions.iter().enumerate() {
            for pos_2 in positions.iter().take(i) {
                let diff = [pos_2[0] - pos_1[0], pos_2[1] - pos_1[1]];
                let res_1 = [pos_2[0] + diff[0], pos_2[1] + diff[1]];
                if res_1[0] >= 0
                    && res_1[0] < lines.len() as i64
                    && res_1[1] >= 0
                    && res_1[1] < lines[0].len() as i64
                {
                    results.insert(res_1);
                }
                let res_2 = [pos_1[0] - diff[0], pos_1[1] - diff[1]];
                if res_2[0] >= 0
                    && res_2[0] < lines.len() as i64
                    && res_2[1] >= 0
                    && res_2[1] < lines[0].len() as i64
                {
                    results.insert(res_2);
                }
            }
        }
    }
    println!("Task1: {}", results.len());

    let mut result_map = lines.clone();
    for positions in frequencies.values() {
        for (i, pos_1) in positions.iter().enumerate() {
            for pos_2 in positions.iter().take(i) {
                let diff = [pos_2[0] - pos_1[0], pos_2[1] - pos_1[1]];
                let mut res_1 = [pos_2[0] + diff[0], pos_2[1] + diff[1]];
                while res_1[0] >= 0
                    && res_1[0] < lines.len() as i64
                    && res_1[1] >= 0
                    && res_1[1] < lines[0].len() as i64
                {
                    result_map[res_1[0] as usize][res_1[1] as usize] = '#';
                    res_1 = [res_1[0] + diff[0], res_1[1] + diff[1]];
                }
                let mut res_2 = [pos_1[0] - diff[0], pos_1[1] - diff[1]];
                while res_2[0] >= 0
                    && res_2[0] < lines.len() as i64
                    && res_2[1] >= 0
                    && res_2[1] < lines[0].len() as i64
                {
                    result_map[res_2[0] as usize][res_2[1] as usize] = '#';
                    res_2 = [res_2[0] - diff[0], res_2[1] - diff[1]];
                }
            }
        }
    }
    let mut result = 0;
    for line in result_map {
        for c in line {
            if c != '.' {
                result += 1;
            }
        }
    }

    println!("Task2: {}", result);
}

fn read_file(path: &str) -> Vec<Vec<char>> {
    let content = fs::read_to_string(path).expect("Failed to read file");
    content
        .split("\n")
        .map(|x| x.to_owned().chars().collect())
        .filter(|x: &Vec<char>| !x.is_empty())
        .collect()
}
