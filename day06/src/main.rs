use core::panic;
use std::{collections::HashSet, fs};

fn find_start(lines: &[Vec<char>]) -> [i64; 2] {
    let mut pos: [i64; 2] = [-1, -1];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '^' {
                pos = [i as i64, j as i64];
                break;
            }
        }
        if pos[0] > -1 {
            break;
        }
    }
    pos
}

fn rotate(dir: &[i64; 2]) -> [i64; 2] {
    match dir {
        [-1, 0] => [0, 1],
        [0, 1] => [1, 0],
        [1, 0] => [0, -1],
        [0, -1] => [-1, 0],
        _ => panic!("This dir should nerver occure"),
    }
}

fn main() {
    let lines = read_file("input");
    let mut dir: [i64; 2] = [-1, 0];
    let start_pos = find_start(&lines);
    let mut pos = start_pos;

    let mut positions: HashSet<[i64; 2]> = HashSet::new();
    loop {
        positions.insert(pos);
        let next_pos = [pos[0] + dir[0], pos[1] + dir[1]];
        if next_pos[0] < 0
            || next_pos[0] >= lines.len() as i64
            || next_pos[1] < 0
            || next_pos[1] >= lines[0].len() as i64
        {
            break;
        }

        let next_char = lines[next_pos[0] as usize][next_pos[1] as usize];
        if next_char == '#' {
            dir = rotate(&dir);
        } else {
            pos = [pos[0] + dir[0], pos[1] + dir[1]];
        }
    }

    println!("Task1: {}", positions.len());
    let mut result = 0;
    for (i, line) in lines.iter().enumerate() {
        for j in 0..line.len() {
            let map_pos = [i as i64, j as i64];
            if positions.contains(&map_pos)
                && map_loops(lines.clone(), start_pos, map_pos)
                && map_pos != start_pos
            {
                result += 1;
            }
        }
    }

    println!("Task2: {}", result);
}

fn map_loops(lines: Vec<Vec<char>>, start_pos: [i64; 2], block_pos: [i64; 2]) -> bool {
    let mut lines = lines.clone();
    lines[block_pos[0] as usize][block_pos[1] as usize] = '#';

    let mut dir: [i64; 2] = [-1, 0];
    let mut pos = start_pos;
    let mut visited: HashSet<[i64; 4]> = HashSet::new();

    loop {
        let state = [pos[0], pos[1], dir[0], dir[1]];
        if visited.contains(&state) {
            return true;
        }
        visited.insert(state);
        let next_pos = [pos[0] + dir[0], pos[1] + dir[1]];
        if next_pos[0] < 0
            || next_pos[0] >= lines.len() as i64
            || next_pos[1] < 0
            || next_pos[1] >= lines[0].len() as i64
        {
            return false;
        }

        let next_char = lines[next_pos[0] as usize][next_pos[1] as usize];
        if next_char == '#' {
            dir = rotate(&dir);
        } else {
            pos = [pos[0] + dir[0], pos[1] + dir[1]];
        }
    }
}

fn read_file(path: &str) -> Vec<Vec<char>> {
    let content = fs::read_to_string(path).expect("Failed to read file");
    content
        .split("\n")
        .map(|x| x.to_owned().chars().collect())
        .filter(|x: &Vec<char>| !x.is_empty())
        .collect()
}
