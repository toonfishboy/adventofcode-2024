use std::{cmp::Ordering, fs};

fn main() {
    let numbers = read_file("input");
    let mut expanded: Vec<i64> = vec![];

    let mut ids = 0;
    for (i, number) in numbers.iter().enumerate() {
        if i % 2 == 0 {
            expanded.resize(expanded.len() + *number as usize, ids);
            ids += 1;
        } else {
            expanded.resize(expanded.len() + *number as usize, -1);
        }
    }

    let mut res_1 = expanded.clone();
    let mut pos = res_1.len() - 1;
    let mut result = 0;
    for (i, space) in expanded.iter().enumerate() {
        if i >= pos {
            break;
        }
        if *space == -1 {
            while res_1[pos] == -1 {
                pos -= 1;
            }
            res_1[i] = res_1[pos];
            res_1[pos] = -1;
            result += i as i64 * res_1[i];
        } else {
            result += i as i64 * space;
        }
    }
    println!("Task1 {}", result);

    let mut disk: Vec<[i64; 2]> = vec![];
    for (i, len) in numbers.iter().enumerate() {
        if *len == 0 {
            continue;
        }
        let id = match i % 2 {
            0 => i as i64 / 2,
            _ => -1,
        };
        disk.push([id, *len]);
    }
    let mut pos = disk.len() - 1;
    while pos > 0 {
        while disk[pos][0] == -1 {
            pos -= 1;
        }

        let mut free_pos = 0;
        while (disk[free_pos][0] >= 0 || disk[free_pos][1] < disk[pos][1]) && free_pos < pos {
            free_pos += 1;
        }

        if free_pos >= pos {
            pos -= 1;
            continue;
        }

        match disk[free_pos][1].cmp(&disk[pos][1]) {
            Ordering::Greater => {
                let diff = disk[free_pos][1] - disk[pos][1];
                disk.splice(free_pos..free_pos, vec![disk[pos]]);

                free_pos += 1;
                pos += 1;

                disk[pos][0] = -1;
                disk[free_pos][1] = diff;
            }
            Ordering::Equal => {
                disk[free_pos][0] = disk[pos][0];
                disk[pos][0] = -1;
            }
            _ => {}
        };

        if pos + 1 < disk.len() && disk[pos + 1][0] == -1 {
            disk[pos][1] += disk[pos + 1][1];
            disk.remove(pos + 1);
        }

        if disk[pos - 1][0] == -1 {
            disk[pos - 1][1] += disk[pos][1];
            disk.remove(pos);
            pos -= 1;
        }

        pos -= 1;
    }

    let mut result = 0;
    let mut block_pos = 0;
    for file in disk {
        for _i in 0..file[1] {
            if file[0] >= 0 {
                result += block_pos * file[0];
            }
            block_pos += 1;
        }
    }

    println!("Task2 {}", result);
}

fn read_file(path: &str) -> Vec<i64> {
    let content = fs::read_to_string(path).expect("Failed to read file");
    content
        .split("")
        .filter(|x| !x.is_empty() && *x != "\n")
        .map(|x| x.to_owned().parse::<i64>().unwrap())
        .collect()
}
