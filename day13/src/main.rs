use core::f64;
use std::fs;

use regex::Regex;

fn part1(input: &str) {
    let mut games: Vec<Vec<[u64; 2]>> = vec![];
    let mut game: Vec<[u64; 2]> = vec![];

    let re = Regex::new(r"(\+|=)(?<val>(\d+))").unwrap();
    for (i, line) in input
        .split("\n")
        .filter(|x| !x.trim().is_empty())
        .enumerate()
    {
        let mut point: [u64; 2] = [0, 0];
        for (i, caps) in re.captures_iter(line).enumerate() {
            point[i] = caps.name("val").unwrap().as_str().parse::<u64>().unwrap();
        }
        game.push(point);
        if i % 3 == 2 {
            games.push(game);
            game = vec![];
        }
    }

    let mut result = 0;
    for game in games {
        let x_b = game[2][0] / game[1][0];
        let y_b = game[2][1] / game[1][1];

        let mut count;
        if x_b < y_b {
            count = x_b;
        } else {
            count = y_b;
        }
        let mut start = 0;
        while count > 0 {
            start = 0;
            while game[0][0] * start + game[1][0] * count < game[2][0]
                && game[0][1] * start + game[1][1] * count < game[2][1]
            {
                start += 1;
            }
            if game[0][0] * start + game[1][0] * count == game[2][0]
                && game[0][1] * start + game[1][1] * count == game[2][1]
            {
                break;
            }

            count -= 1;
        }
        if count > 0 {
            result += start * 3 + count;
        }
    }

    println!("Task1: {}", result);
}

fn part2(input: &str) {
    let mut games: Vec<Vec<[i64; 2]>> = vec![];
    let mut game: Vec<[i64; 2]> = vec![];

    let re = Regex::new(r"(\+|=)(?<val>(\d+))").unwrap();
    for (i, line) in input
        .split("\n")
        .filter(|x| !x.trim().is_empty())
        .enumerate()
    {
        let mut point: [i64; 2] = [0, 0];
        for (i, caps) in re.captures_iter(line).enumerate() {
            point[i] = caps.name("val").unwrap().as_str().parse::<i64>().unwrap();
        }
        game.push(point);
        if i % 3 == 2 {
            let extra = 10000000000000;
            game[2][0] += extra;
            game[2][1] += extra;
            games.push(game);
            game = vec![];
        }
    }

    let mut result = 0.0;
    for game in games {
        let [a1, a2] = game[0];
        let [b1, b2] = game[1];
        let [p1, p2] = game[2];

        let x: f64 = ((p1 * b2 - p2 * b1) as f64) / ((a1 * b2 - a2 * b1) as f64);
        let y: f64 = ((a1 * p2 - a2 * p1) as f64) / ((a1 * b2 - a2 * b1) as f64);

        if x % 1.0 == 0.0 && y % 1.0 == 0.0 {
            result += x * 3.0 + y;
        }
    }

    println!("Task2: {}", result);
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read file");
    part1(&input);
    part2(&input);
}
