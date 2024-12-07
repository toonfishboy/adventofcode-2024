use std::fs;

fn main() {
    let lines = read_file("input");
    let mut result = 0;
    let mut result_2 = 0;
    for line in lines {
        let temp: Vec<&str> = line.split(':').collect();
        let res = temp[0].trim().parse::<i64>().unwrap();
        let numbers = temp[1]
            .trim()
            .split(' ')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut values: Vec<i64> = vec![numbers[0]];
        let mut values_2: Vec<i64> = vec![numbers[0]];

        for number in &numbers[1..] {
            let mut new_values: Vec<i64> = vec![];
            for v in values {
                let res1 = v * number;
                if res1 <= res {
                    new_values.push(res1);
                }
                let res2 = v + number;
                if res2 <= res {
                    new_values.push(res2);
                }
            }
            values = new_values;
            let mut new_values_2: Vec<i64> = vec![];
            for v in values_2 {
                let res1 = v * number;
                if res1 <= res {
                    new_values_2.push(res1);
                }
                let res2 = v + number;
                if res2 <= res {
                    new_values_2.push(res2);
                }
                let res3 = format!("{}{}", v, number).parse::<i64>().unwrap();
                if res3 <= res {
                    new_values_2.push(res3);
                }
            }
            values_2 = new_values_2;
        }

        let has_res = values.iter().any(|x| *x == res);
        if has_res {
            result += res;
        }

        let has_res_2 = values_2.iter().any(|x| *x == res);
        if has_res_2 {
            result_2 += res;
        }
    }
    println!("Task1: {}", result);
    println!("Task2: {}", result_2);
}

fn read_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Failed to read file");
    content
        .split("\n")
        .map(|x| x.to_owned())
        .filter(|x| !x.is_empty())
        .collect()
}
