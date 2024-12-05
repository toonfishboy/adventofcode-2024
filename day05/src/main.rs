use std::{collections::HashMap, fs};

fn main() {
    let lines = read_file("input");
    let mut numbers: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut is_sequence = false;
    let mut failed_sequences = vec![];
    let mut result = 0;

    for line in lines {
        if line.is_empty() {
            is_sequence = true;
            continue;
        }
        if !is_sequence {
            let pages = line
                .split("|")
                .map(|x| x.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            if let std::collections::hash_map::Entry::Vacant(e) = numbers.entry(pages[1]) {
                e.insert(vec![pages[0]]);
            } else {
                let v = numbers.get_mut(&pages[1]).unwrap();
                v.push(pages[0]);
            }
        } else {
            let sequence = line
                .split(",")
                .map(|x| x.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let mut is_seq = true;
            for (i, page) in sequence.iter().enumerate() {
                if let Some(deps) = numbers.get(page) {
                    let in_seq_deps = deps.iter().filter(|d| sequence.iter().any(|s| &s == d));
                    for dep in in_seq_deps {
                        let has_dep = sequence.iter().take(i).any(|s| s == dep);
                        if !has_dep {
                            is_seq = false;
                            break;
                        }
                    }
                }

                if !is_seq {
                    failed_sequences.push(sequence.clone());
                    break;
                }
            }
            if is_seq {
                result += sequence[sequence.len() / 2];
            }
        }
    }

    println!("Task1: {}", result);

    let mut result = 0;
    for sequence in failed_sequences {
        let mut updated: Vec<u64> = vec![];
        while updated.len() < sequence.len() {
            for page in &sequence {
                if updated.iter().any(|u| *u == *page) {
                    continue;
                }
                if let Some(deps) = numbers.get(page) {
                    let in_seq_deps = deps.iter().filter(|d| sequence.iter().any(|s| &s == d));
                    if in_seq_deps.clone().count() == 0 {
                        updated.push(*page);
                        break;
                    }
                    let mut has_dep = true;
                    for dep in in_seq_deps {
                        if !updated.iter().any(|s| s == dep) {
                            has_dep = false;
                            break;
                        }
                    }
                    if has_dep {
                        updated.push(*page);
                    }
                } else {
                    updated.push(*page);
                    break;
                }
            }
        }
        result += updated[updated.len() / 2];
    }
    println!("Task2: {}", result);
}

fn read_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Failed to read file");
    content.split("\n").map(|x| x.to_owned()).collect()
}
