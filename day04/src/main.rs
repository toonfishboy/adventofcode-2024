use std::{char, fs};

fn main() {
    let content = read_file("input");
    let mut rows: Vec<String> = vec![];
    for i in 0..content.len() {
        for j in 0..content[i].len() {
            if content[i][j] == 'X' {
                if i >= 3 {
                    let result = format!(
                        "{}{}{}{}",
                        content[i][j],
                        content[i - 1][j],
                        content[i - 2][j],
                        content[i - 3][j]
                    );
                    rows.push(result);

                    if j >= 3 {
                        let result = format!(
                            "{}{}{}{}",
                            content[i][j],
                            content[i - 1][j - 1],
                            content[i - 2][j - 2],
                            content[i - 3][j - 3]
                        );
                        rows.push(result);
                    }
                    if j < content[i].len() - 3 {
                        let result = format!(
                            "{}{}{}{}",
                            content[i][j],
                            content[i - 1][j + 1],
                            content[i - 2][j + 2],
                            content[i - 3][j + 3]
                        );
                        rows.push(result);
                    }
                }
                if i < content.len() - 3 {
                    let result = format!(
                        "{}{}{}{}",
                        content[i][j],
                        content[i + 1][j],
                        content[i + 2][j],
                        content[i + 3][j]
                    );
                    rows.push(result);
                    if j >= 3 {
                        let result = format!(
                            "{}{}{}{}",
                            content[i][j],
                            content[i + 1][j - 1],
                            content[i + 2][j - 2],
                            content[i + 3][j - 3]
                        );
                        rows.push(result);
                    }
                    if j < content[i].len() - 3 {
                        let result = format!(
                            "{}{}{}{}",
                            content[i][j],
                            content[i + 1][j + 1],
                            content[i + 2][j + 2],
                            content[i + 3][j + 3]
                        );
                        rows.push(result);
                    }
                }
                if j >= 3 {
                    let result = format!(
                        "{}{}{}{}",
                        content[i][j],
                        content[i][j - 1],
                        content[i][j - 2],
                        content[i][j - 3]
                    );
                    rows.push(result);
                }
                if j < content[i].len() - 3 {
                    let result = format!(
                        "{}{}{}{}",
                        content[i][j],
                        content[i][j + 1],
                        content[i][j + 2],
                        content[i][j + 3]
                    );
                    rows.push(result);
                }
            }
        }
    }

    println!(
        "Task1: {}",
        rows.into_iter().filter(|x| x == "XMAS").count()
    );

    let mut result = 0;
    for i in 0..content.len() {
        for j in 0..content[i].len() {
            if content[i][j] == 'A'
                && i > 0
                && i < content.len() - 1
                && j > 0
                && j < content[i].len() - 1
            {
                let mut dia_1: Vec<char> = [content[i - 1][j - 1], content[i + 1][j + 1]].to_vec();
                let mut dia_2: Vec<char> = [content[i - 1][j + 1], content[i + 1][j - 1]].to_vec();
                dia_1.sort();
                dia_2.sort();
                if dia_1[0] == 'M' && dia_1[1] == 'S' && dia_2[0] == 'M' && dia_2[1] == 'S' {
                    result += 1;
                }
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
