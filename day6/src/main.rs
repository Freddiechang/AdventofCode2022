use std::fs;
use std::collections::HashMap;

fn read_file(filename: String) -> Vec<char> {
    let contents: String = fs::read_to_string(filename).unwrap();
    return contents.strip_suffix('\n').unwrap().chars().collect();
}

fn find_marker(line: &Vec<char>, num_distinct: usize) -> usize {
    assert!(line.len() >= num_distinct);
    let mut window = HashMap::new();
    for i in 0..num_distinct {
        if let Some(x)  = window.get_mut(&line[i]) {
            *x += 1;
        }
        else {
            window.insert(line[i], 1);
        }
    }
    let mut index = num_distinct;
    while window.len() != num_distinct {
        let mut out = window.get_mut(&line[index - num_distinct]).unwrap();
        if *out == 1 {
            window.remove(&line[index - num_distinct]);
        }
        else {
            *out -= 1;
        }
        if let Some(x)  = window.get_mut(&line[index]) {
            *x += 1;
        }
        else {
            window.insert(line[index], 1);
        }
        index += 1;
    }
    return index;
}

fn main() {
    let buffer = read_file("input.txt".to_string());
    let r = find_marker(&buffer, 4);
    println!("{}", r);
    let r = find_marker(&buffer, 14);
    println!("{}", r);
}
