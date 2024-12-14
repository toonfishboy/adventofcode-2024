use std::{
    fs::{self, File},
    io::BufWriter,
    path::Path,
};

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: [i64; 2],
    vel: [i64; 2],
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read file");
    let robots = input
        .trim()
        .split("\n")
        .map(|line| {
            let mut robot = Robot {
                pos: [0, 0],
                vel: [0, 0],
            };
            for (i, value) in line.trim().split(" ").enumerate() {
                let (_, value) = value.split_at(2);
                let coords = value
                    .split(",")
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                match i {
                    0 => robot.pos = [coords[0], coords[1]],
                    1 => robot.vel = [coords[0], coords[1]],
                    _ => panic!("Value not read correctly"),
                }
            }
            robot
        })
        .collect::<Vec<Robot>>();
    let width = 101;
    let heigth = 103;

    let mut qudrants: [usize; 4] = [0, 0, 0, 0];
    for robot in &robots {
        let mut pos_x = (robot.pos[0] + (robot.vel[0] * 100)) % width;
        let mut pos_y = (robot.pos[1] + (robot.vel[1] * 100)) % heigth;
        if pos_x < 0 {
            pos_x += width;
        }
        if pos_y < 0 {
            pos_y += heigth;
        }
        if pos_x < width / 2 && pos_y < heigth / 2 {
            qudrants[0] += 1;
        } else if pos_x > width / 2 && pos_y < heigth / 2 {
            qudrants[1] += 1;
        } else if pos_x < width / 2 && pos_y > heigth / 2 {
            qudrants[2] += 1;
        } else if pos_x > width / 2 && pos_y > heigth / 2 {
            qudrants[3] += 1;
        }
    }

    let mut result = 1;
    for count in qudrants {
        result *= count;
    }
    println!("Task1: {}", result);

    match fs::exists("easteregg") {
        Ok(_) => {}
        Err(_) => {
            fs::create_dir("easteregg").expect("Failed to create dir easteregg");
        }
    }

    let mut robots_2 = robots.clone();
    for seconds in 0..10000 {
        let file_name = format!("easteregg/{}.png", seconds + 1);
        let path = Path::new(&file_name);
        let file = File::create(path).unwrap();
        let w = &mut BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, 101, 103);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        for robot in &mut robots_2 {
            let mut pos_x = (robot.pos[0] + robot.vel[0]) % width;
            let mut pos_y = (robot.pos[1] + robot.vel[1]) % heigth;
            if pos_x < 0 {
                pos_x += width;
            }
            if pos_y < 0 {
                pos_y += heigth;
            }
            robot.pos = [pos_x, pos_y];
        }
        let mut data = vec![];
        for i in 0..heigth {
            for j in 0..width {
                let robot = &robots_2.iter().find(|x| x.pos == [j, i]);
                data.push(0);
                if robot.is_some() {
                    data.push(255);
                } else {
                    data.push(0);
                };
                data.push(0);
                data.push(255);
            }
        }
        writer.write_image_data(&data).unwrap();
    }
}
