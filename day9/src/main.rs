use std::fs;
use std::collections::HashSet;

fn read_file(filename: String) -> Vec<Vec<usize>> {
    let contents: String = fs::read_to_string(filename).unwrap();
    contents.strip_suffix('\n').unwrap().split('\n')
    .map(
        |x: &str| x.split(' ').map(
            |y| {
                if let Ok(z) = y.parse::<usize>() {
                    z
                }
                else {
                    match y {
                        "L" => 1,
                        "R" => 2,
                        "U" => 3,
                        "D" => 4,
                        _ => 0,
                    }
                }
            }
        ).collect::<Vec<usize>>()
    ).collect::<Vec<_>>()
}

fn process_commands(commands: &Vec<Vec<usize>>, num_knots: usize) -> HashSet<(i32, i32)> {
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); num_knots];
    let mut results = HashSet::new();
    results.insert(knots[num_knots - 1].clone());
    for i in commands.iter() {
        for _ in 0..i[1] {
            if i[0] == 1 {
                knots[0].0 -= 1;
            }
            else if i[0] == 2 {
                knots[0].0 += 1;
            }
            else if i[0] == 3 {
                knots[0].1 -= 1;
            }
            else if i[0] == 4 {
                knots[0].1 += 1;
            }
            for j in 1..num_knots {
                let distance = (knots[j - 1].0 - knots[j].0, knots[j - 1].1 - knots[j].1);
                if distance.0 == 0 && (distance.1).abs() == 2 {
                    knots[j].1 += (distance.1).signum();
                }
                else if distance.1 == 0 && distance.0.abs() == 2 {
                    knots[j].0 += (distance.0).signum();
                }
                else if distance.0.abs() > 1 || distance.1.abs() > 1 {
                    knots[j].0 += (distance.0).signum();
                    knots[j].1 += (distance.1).signum();
                }
            }
            results.insert(knots[num_knots - 1].clone());
        }
    }
    results
}

fn main() {
    let commands = read_file("input.txt".to_string());
    let tail_pos = process_commands(&commands, 2);
    println!("{:?}", tail_pos.len());
    let tail_pos = process_commands(&commands, 10);
    println!("{:?}", tail_pos.len());
}
