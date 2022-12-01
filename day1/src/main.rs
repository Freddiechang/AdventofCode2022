use std::fs;

fn read_file(filename: String) -> Vec<Vec<i32>> {
    let contents: String = fs::read_to_string(filename).unwrap();
    let mut results: Vec<Vec<i32>> = vec![];
    let mut temp: Vec<i32> = vec![];
    let f = |x: &str| {
        if let Ok(value) = x.parse::<i32>() {
            temp.push(value);
        }
        else {
            results.push(temp.clone());
            temp.clear();
        }
        ()
    };
    contents.split('\n').map(f).collect::<Vec<_>>();
    results
}

fn main() {
    println!("Hello, world!");
    let items = read_file(String::from("input.txt"));
    let mut sums: Vec<i32> = items.iter().map(|x| x.iter().sum()).collect();
    sums.sort();
    println!("{}", sums[sums.len() - 1]);
    println!("{}", sums.iter().skip(sums.len() - 3).sum::<i32>());
}
