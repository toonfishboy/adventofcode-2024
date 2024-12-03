use regex::Regex;
use std::fs;

fn main() {
    let re = Regex::new(r"mul\((\d+,\d+)\)").unwrap();
    let content = read_file("input");

    let mut result = 0;
    for (_, [mul]) in re.captures_iter(&content).map(|c| c.extract()) {
        let number: Vec<i64> = mul
            .split(",")
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();
        if number.len() >= 2 {
            result += number[0] * number[1];
        }
    }
    println!("Task1: {}", result);

    let mut result = 0;
    let re = Regex::new(r"(?<op>mul|don't|do)\((?<val>()|(\d+,\d+))\)").unwrap();
    let mut enabled = true;

    for caps in re.captures_iter(&content) {
        if let Some(res) = caps.name("op") {
            let op = res.as_str();

            match op {
                "do" => enabled = true,
                "don't" => enabled = false,
                val if val == "mul" && enabled => {
                    let mul = caps.name("val").unwrap().as_str();
                    let number: Vec<i64> = mul
                        .split(",")
                        .map(|x| x.trim().parse::<i64>().unwrap())
                        .collect();
                    if number.len() >= 2 {
                        result += number[0] * number[1];
                    }
                }
                _ => {}
            }
        }
    }

    println!("Task2: {}", result);
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read file")
}
