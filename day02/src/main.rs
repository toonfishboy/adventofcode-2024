use std::fs;

#[derive(PartialEq)]
enum Level {
    Increase,
    Decrease,
}

fn check_line(line: &[i64]) -> Vec<usize> {
    let mut level = Level::Increase;
    let mut errors: Vec<usize> = vec![];
    if line[0] - line[1] < 0 {
        level = Level::Decrease;
    }
    for i in 0..line.len() - 1 {
        let diff = line[i] - line[i + 1];
        let mut new_level = Level::Increase;
        if diff < 0 {
            new_level = Level::Decrease;
        }
        let is_safe = (1..=3).contains(&diff.abs()) && level == new_level;
        level = new_level;
        if !is_safe {
            errors.push(i);
        }
    }
    errors
}

fn main() {
    let lines = read_file("input");
    let mut result = 0;

    for line in lines.clone() {
        let line_errors = check_line(&line);
        if line_errors.is_empty() {
            result += 1;
        }
    }

    println!("Task1: {}", result);

    let mut result = 0;

    for line in lines {
        let line_errors = check_line(&line);
        if line_errors.is_empty() {
            result += 1;
            continue;
        }

        for error in &line_errors {
            let mut mod_line = line.clone();
            mod_line.remove(*error + 1);
            let mod_errors = check_line(&mod_line);
            if mod_errors.is_empty() {
                result += 1;
                break;
            }

            mod_line = line.clone();
            mod_line.remove(*error);
            let mod_errors = check_line(&mod_line);
            if mod_errors.is_empty() {
                result += 1;
                break;
            }

            if *error > 0 {
                mod_line = line.clone();
                mod_line.remove(*error - 1);
                let mod_errors = check_line(&mod_line);
                if mod_errors.is_empty() {
                    result += 1;
                    break;
                }
            }
        }
    }

    println!("Task2: {}", result);
}

fn read_file(path: &str) -> Vec<Vec<i64>> {
    let content = fs::read_to_string(path).expect("Failed to read file");
    let lines: Vec<String> = content.split("\n").map(|x| x.to_owned()).collect();
    let mut result: Vec<Vec<i64>> = vec![];

    for line in lines {
        let numbers = line
            .split(" ")
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        if !numbers.is_empty() {
            result.push(numbers);
        }
    }
    result
}
