use core::panic;
use std::fs;

fn find_robot(grid: &[Vec<char>]) -> [usize; 2] {
    let mut robot: [usize; 2] = [0, 0];
    for (i, line) in grid.iter().enumerate() {
        for (j, symbol) in line.iter().enumerate() {
            if *symbol == '@' {
                robot = [j, i];
            }
        }
    }
    robot
}

fn get_dir(operator: &char) -> [i32; 2] {
    match operator {
        '^' => [0, -1],
        'v' => [0, 1],
        '<' => [-1, 0],
        '>' => [1, 0],
        _ => panic!("unkown operator"),
    }
}

fn main() {
    let content = fs::read_to_string("example").expect("Failed to read file");
    let mut grid: Vec<Vec<char>> = vec![];
    let mut input: Vec<char> = vec![];

    for line in content.split("\n") {
        let symbols = line.chars().collect::<Vec<char>>();
        if line.starts_with("#") {
            grid.push(symbols);
        } else if !line.is_empty() {
            input = [input, symbols].concat();
        }
    }

    let mut grid_2 = vec![];
    //scale grid
    for line in grid.iter() {
        let mut new_line = vec![];
        for symbol in line {
            let symbols = match symbol {
                'O' => ['[', ']'],
                '#' => ['#', '#'],
                '@' => ['@', '.'],
                '.' => ['.', '.'],
                _ => panic!("Unkown symbol to scale"),
            };
            new_line = [new_line, symbols.to_vec()].concat();
        }
        grid_2.push(new_line);
    }

    let mut robot = find_robot(&grid);
    for operator in &input {
        let dir = get_dir(operator);
        let next_point = [robot[0] as i32 + dir[0], robot[1] as i32 + dir[1]];
        let next = grid[next_point[1] as usize][next_point[0] as usize];
        if next == '.' {
            grid[next_point[1] as usize][next_point[0] as usize] = '@';
            grid[robot[1]][robot[0]] = '.';
            robot = [next_point[0] as usize, next_point[1] as usize];
        } else if next == 'O' {
            let mut pos = 1;
            let mut value = next;
            while value != '#' {
                value = grid[(next_point[1] + dir[1] * pos) as usize]
                    [(next_point[0] + dir[0] * pos) as usize];
                if value == '.' {
                    break;
                }
                pos += 1;
            }

            if value != '#' {
                grid[robot[1]][robot[0]] = '.';
                grid[next_point[1] as usize][next_point[0] as usize] = '@';
                for i in 0..pos {
                    grid[(next_point[1] + dir[1] * (i + 1)) as usize]
                        [(next_point[0] + dir[0] * (i + 1)) as usize] = 'O';
                }
                robot = [next_point[0] as usize, next_point[1] as usize];
            }
        }
    }

    let mut result = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, symbol) in line.iter().enumerate() {
            if *symbol == 'O' {
                result += 100 * i + j;
            }
        }
    }
    println!("Task1: {}", result);

    let mut robot = find_robot(&grid_2);
    for operator in &input {
        let dir = get_dir(operator);
        let next_point = [robot[0] as i32 + dir[0], robot[1] as i32 + dir[1]];
        let next = grid[next_point[1] as usize][next_point[0] as usize];
        if next == '.' {
            grid[next_point[1] as usize][next_point[0] as usize] = '@';
            grid[robot[1]][robot[0]] = '.';
            robot = [next_point[0] as usize, next_point[1] as usize];
        } else if next == '[' || next == ']' {
            let mut pos = 1;
            let mut value = next;
            while value != '#' {
                value = grid[(next_point[1] + dir[1] * pos) as usize]
                    [(next_point[0] + dir[0] * pos) as usize];
                if value == '.' {
                    break;
                }
                pos += 1;
            }

            if value != '#' {
                grid[robot[1]][robot[0]] = '.';
                grid[next_point[1] as usize][next_point[0] as usize] = '@';
                for i in 0..pos {
                    match operator {
                        '^' => {
                            grid[(next_point[1] + dir[1] * (i + 1)) as usize]
                                [(next_point[0] + dir[0] * (i + 1)) as usize] = 'O';
                        }
                        'v' => [0, 1],
                        '<' => [-1, 0],
                        '>' => [1, 0],
                        _ => panic!("unkown operator"),
                    }
                }
                robot = [next_point[0] as usize, next_point[1] as usize];
            }
        }
    }

    for line in grid_2.iter() {
        for symbol in line.iter() {
            print!("{}", symbol);
        }
        println!();
    }
}
