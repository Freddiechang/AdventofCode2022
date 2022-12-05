use std::fs;

fn read_file(filename: String) -> Vec<Vec<i32>> {
    let contents: String = fs::read_to_string(filename).unwrap();
    let mut result = vec![];
    for i in contents.split('\n') {
        let mut temp = vec![];
        for j in i.split(',') {
            for k in j.split('-') {
                if let Ok(x) = k.parse::<i32>() {
                    temp.push(x);
                }
            }
        }
        if temp.len() != 0 {
            result.push(temp);
        }
    }
    result
}

fn find_fully_contained1(assignments: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    let mut f = |x: &Vec<i32>| {
        if (x[0] >= x[2] && x[1] <= x[3]) || (x[0] <= x[2] && x[1] >= x[3]) {
            result += 1;
        }
    };
    assignments
    .iter()
    .for_each(|y| f(y));
    result
}

fn find_fully_contained2(assignments: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    let mut f = |x: &Vec<i32>| {
        if (x[0] >= x[2] && x[0] <= x[3]) || 
        (x[1] >= x[2] && x[1] <= x[3]) || 
        (x[0] >= x[2] && x[1] <= x[3]) || 
        (x[0] <= x[2] && x[1] >= x[3]) {
            result += 1;
        }
    };
    assignments
    .iter()
    .for_each(|y| f(y));
    result
}

fn main() {
    let assignments = read_file("input.txt".to_string());
    println!("{:?}", find_fully_contained1(&assignments));
    println!("{:?}", find_fully_contained2(&assignments));
}
