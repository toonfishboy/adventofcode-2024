use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

static DIRS: &[[i32; 2]; 4] = &[[-1, 0], [0, 1], [1, 0], [0, -1]];

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct State {
    point: [usize; 2],
    direction: [i32; 2],
    turns: usize,
    steps: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .turns
            .cmp(&self.turns)
            .then_with(|| other.steps.cmp(&self.steps))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = fs::read_to_string("example").expect("Failed to read file.");
    let maze = input
        .trim()
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    //find start
    let mut start: [usize; 2] = [0, 0];
    let mut end: [usize; 2] = [0, 0];
    for (i, line) in maze.iter().enumerate() {
        for (j, symbol) in line.iter().enumerate() {
            if *symbol == 'S' {
                start = [i, j];
            }
            if *symbol == 'E' {
                end = [i, j];
            }
        }
    }

    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut came_from = HashMap::new();

    heap.push(State {
        point: start,
        direction: [0, 1],
        turns: 0,
        steps: 0,
    });
    visited.insert((start, [0, 1]));

    let mut result = (vec![], 0);

    while let Some(current) = heap.pop() {
        if current.point == end {
            println!("Visited: {:?}", visited);
            let mut path = Vec::new();
            let mut step = current.point;
            while step != start {
                path.push(step);
                step = came_from[&step];
            }
            path.push(start);
            path.reverse();
            result = (path, current.turns);
            break;
        }

        for dir in DIRS {
            let neighbor = [
                (current.point[0] as i32 + dir[0]) as usize,
                (current.point[1] as i32 + dir[1]) as usize,
            ];

            let new_turn = if current.direction == *dir {
                current.turns
            } else {
                current.turns + 1
            };

            if maze[neighbor[0]][neighbor[1]] != '#' && !visited.contains(&(neighbor, *dir)) {
                heap.push(State {
                    point: neighbor,
                    direction: *dir,
                    turns: new_turn,
                    steps: current.steps + 1,
                });
                visited.insert((neighbor, *dir));
                came_from.insert(neighbor, current.point);
            }
        }
    }

    println!("Test");
    for (i, line) in maze.iter().enumerate() {
        for (j, symbol) in line.iter().enumerate() {
            if result.0.iter().any(|x| *x == [i, j]) {
                print!("O");
            } else {
                print!("{}", symbol);
            }
        }
        println!()
    }

    println!("Task1: {:?} {:?}", result.0.len(), result.1);
}
