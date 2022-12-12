pub mod tree;
use std::fs;

fn read_file(filename: String) -> Vec<Vec<String>> {
    let contents: String = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = contents.strip_suffix('\n').unwrap().split('\n').collect();
    let results = lines
        .iter()
        .map(|x| x.split(' ').map(|x| x.to_string()).collect::<Vec<String>>())
        .collect();
    results
}

fn main() {
    let commands = read_file("input.txt".to_string());
    let mut tree = tree::Tree::create_tree(&commands);
    tree.update_size(0);
    println!("{}", tree.part1(0));
    println!("{}", tree.part2(0));
    //tree.pretty_print();
}
