use std::fs;

fn main() {
    let lines = read_file("input");
    let mut list_a = vec![];
    let mut list_b = vec![];

    for line in lines {
        let numbers = line
            .split(" ")
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        if numbers.len() == 2 {
            list_a.push(numbers[0]);
            list_b.push(numbers[1]);
        }
    }

    list_a.sort();
    list_b.sort();

    //task 1
    let mut result = 0;

    for i in 0..list_a.len() {
        result += (list_a[i] - list_b[i]).abs();
    }
    println!("Task1: {}", result);

    //task2
    let mut result = 0;

    for value in list_a {
        result += value
            * list_b
                .clone()
                .into_iter()
                .filter(|x| *x == value)
                .collect::<Vec<i64>>()
                .len() as i64;
    }

    println!("Task2: {}", result);
}

fn read_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Failed to read file");
    content.trim().split("\n").map(|x| x.to_owned()).collect()
}
