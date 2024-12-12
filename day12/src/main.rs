use std::{collections::HashSet, fs};

static DIRS_2: &[(i64, i64, char); 4] = &[(-1, 0, 'U'), (0, 1, 'R'), (1, 0, 'D'), (0, -1, 'L')];
static DIRS: &[[i64; 2]; 4] = &[[-1, 0], [0, 1], [1, 0], [0, -1]];

fn find_area(
    garden: &Vec<Vec<char>>,
    plant: char,
    point: [usize; 2],
    visited: &mut Vec<[usize; 2]>,
) -> Vec<[i64; 2]> {
    if visited.contains(&point) {
        return vec![];
    }
    visited.push(point);

    let mut perimeter = vec![];
    for dir in DIRS {
        let next = [point[0] as i64 + dir[0], point[1] as i64 + dir[1]];
        if next[0] < 0
            || next[0] >= garden.len() as i64
            || next[1] < 0
            || next[1] >= garden[0].len() as i64
            || garden[next[0] as usize][next[1] as usize] != plant
        {
            perimeter.push(next);
        } else {
            perimeter = [
                perimeter,
                find_area(garden, plant, [next[0] as usize, next[1] as usize], visited),
            ]
            .concat();
        }
    }

    perimeter
}

fn dfs(
    i: i64,
    j: i64,
    plant: char,
    garden: &Vec<Vec<char>>,
    edges: &mut HashSet<(i64, i64, char)>,
    dir: Option<char>,
    visited: &mut HashSet<[usize; 2]>,
) -> [usize; 2] {
    if i < 0
        || j < 0
        || i >= garden.len() as i64
        || j >= garden[0].len() as i64
        || garden[i as usize][j as usize] != plant
    {
        edges.insert((i, j, dir.unwrap()));
        return [0, 1];
    }

    if visited.contains(&[i as usize, j as usize]) {
        return [0, 0];
    }

    visited.insert([i as usize, j as usize]);

    let mut res = [1, 0];
    for (x, y, d) in DIRS_2 {
        let [a, b] = dfs(i + x, j + y, plant, garden, edges, Some(*d), visited);
        res[0] += a;
        res[1] += b;
    }
    res
}

fn reduce_edges(edges: &mut HashSet<(i64, i64, char)>) -> HashSet<(i64, i64, char)> {
    let mut result: HashSet<(i64, i64, char)> = HashSet::new();
    while !edges.is_empty() {
        if let Some((x, y, dir)) = edges.clone().iter().next() {
            if *dir == 'L' || *dir == 'R' {
                let mut dx = 0;
                while edges.contains(&(x - dx, *y, *dir)) {
                    edges.remove(&(x - dx, *y, *dir));
                    dx += 1;
                }
                dx = 1;
                while edges.contains(&(x + dx, *y, *dir)) {
                    edges.remove(&(x + dx, *y, *dir));
                    dx += 1;
                }
            } else {
                let mut dy = 0;
                while edges.contains(&(*x, y - dy, *dir)) {
                    edges.remove(&(*x, y - dy, *dir));
                    dy += 1;
                }
                dy = 1;
                while edges.contains(&(*x, y + dy, *dir)) {
                    edges.remove(&(*x, y + dy, *dir));
                    dy += 1;
                }
            }
            result.insert((*x, *y, *dir));
            edges.remove(&(*x, *y, *dir));
        }
    }

    result
}

fn part2(garden: &Vec<Vec<char>>) {
    let mut visited: HashSet<[usize; 2]> = HashSet::new();
    let mut result = 0;

    for (i, line) in garden.iter().enumerate() {
        for (j, plant) in line.iter().enumerate() {
            let mut edges: HashSet<(i64, i64, char)> = HashSet::new();
            let [a, _] = dfs(
                i as i64,
                j as i64,
                *plant,
                garden,
                &mut edges,
                None,
                &mut visited,
            );
            result += a * reduce_edges(&mut edges).len();
        }
    }

    println!("Task2: {}", result)
}

fn main() {
    let garden = read_file("input");

    let mut result = 0;
    let mut all_points = vec![];
    for (i, line) in garden.iter().enumerate() {
        for (j, plant) in line.iter().enumerate() {
            if all_points.contains(&[i, j]) {
                continue;
            }
            let mut visited = vec![];
            let perimeter = find_area(&garden, *plant, [i, j], &mut visited);
            result += visited.len() * perimeter.len();
            all_points = [all_points, visited.clone()].concat();
        }
    }

    println!("Task1: {}", result);
    part2(&garden);
}

fn read_file(path: &str) -> Vec<Vec<char>> {
    let content = fs::read_to_string(path).expect("Failed to read file");
    content
        .trim()
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned().chars().collect::<Vec<char>>())
        .collect()
}
