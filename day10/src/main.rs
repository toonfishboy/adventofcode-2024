use std::{collections::HashSet, fs};

static DIRS: &[[i64; 2]; 4] = &[[-1, 0], [0, 1], [1, 0], [0, -1]];

fn check_path(grid: &[Vec<i64>], current_value: i64, point: [i64; 2]) -> HashSet<[i64; 2]> {
    if current_value == 9 {
        return HashSet::from([point]);
    }
    let mut result = HashSet::new();
    for dir in DIRS {
        let next_point = [point[0] + dir[0], point[1] + dir[1]];
        if next_point[0] >= 0
            && next_point[0] < grid.len() as i64
            && next_point[1] >= 0
            && next_point[1] < grid[0].len() as i64
            && grid[next_point[0] as usize][next_point[1] as usize] == current_value + 1
        {
            let checks = check_path(grid, current_value + 1, next_point);
            result.extend(checks);
        }
    }
    result
}

fn check_path_2(grid: &[Vec<i64>], current_value: i64, point: [i64; 2]) -> u64 {
    if current_value == 9 {
        return 1;
    }
    let mut result = 0;
    for dir in DIRS {
        let next_point = [point[0] + dir[0], point[1] + dir[1]];
        if next_point[0] >= 0
            && next_point[0] < grid.len() as i64
            && next_point[1] >= 0
            && next_point[1] < grid[0].len() as i64
            && grid[next_point[0] as usize][next_point[1] as usize] == current_value + 1
        {
            result += check_path_2(grid, current_value + 1, next_point);
        }
    }
    result
}

fn main() {
    let grid = read_file("input");

    let mut result = 0;
    let mut result_2 = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, point) in line.iter().enumerate() {
            if *point == 0 {
                let checks = check_path(&grid, 0, [i as i64, j as i64]);
                result += checks.len();
                result_2 += check_path_2(&grid, 0, [i as i64, j as i64]);
            }
        }
    }
    println!("Task1: {}", result);
    println!("Task2: {}", result_2);
}

fn read_file(path: &str) -> Vec<Vec<i64>> {
    let content = fs::read_to_string(path).expect("Failed to read file");
    content
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split("")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i64>().unwrap_or(-1))
                .collect::<Vec<i64>>()
        })
        .collect()
}
