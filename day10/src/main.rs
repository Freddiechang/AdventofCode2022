use std::fs;

fn read_file(filename: String) -> Vec<i32> {
    let contents: String = fs::read_to_string(filename).unwrap();
    let mut r = 1;
    let mut result = vec![];
    contents
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .for_each(|x| match x {
            "noop" => {
                result.push(r);
            }
            y => {
                result.push(r);
                result.push(r);
                r += y.split(' ').skip(1).next().unwrap().parse::<i32>().unwrap();
            }
        });
    result
}

fn crt(pos: &Vec<i32>) -> Vec<String> {
    let mut temp = String::new();
    let mut result = vec![];
    for (idx, i) in pos.iter().enumerate() {
        if idx != 0 && idx % 40 == 0 {
            result.push(temp.clone());
            temp.clear();
        }
        if (idx % 40) as i32 >= i - 1 && (idx % 40) as i32 <= i + 1 {
            temp.push('#');
        }
        else {
            temp.push('.');
        }
    }
    result.push(temp.clone());
    result
}

fn main() {
    let pos = read_file("input.txt".to_string());
    let sum = pos
        .iter()
        .enumerate()
        .skip(19)
        .filter(|(idx, _)| (idx - 19) % 40 == 0)
        .take(6)
        .fold(0, |acc, (idx, x)| acc + (idx as i32 + 1) * x);
    println!("{}", sum);
    crt(&pos).iter().for_each(|x| println!("{}", x));
}
