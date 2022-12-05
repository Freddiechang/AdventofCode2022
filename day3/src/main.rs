use std::fs;

fn read_file(filename: String) -> Vec<Vec<i32>> {
    let contents: String = fs::read_to_string(filename).unwrap();
    let f = |x: char| {
        if (x as u8) >= ('a' as u8) { (x as u8) - ('a' as u8) + 1 }
        else { (x as u8) - ('A' as u8) + 27 }
    };
    contents
    .split('\n')
    .map(|y| y.chars().map(|z| f(z) as i32).collect::<Vec<i32>>())
    .collect::<Vec<_>>()
}

fn find_common1(items: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![];
    for i in items.iter() {
        let mut count = vec![0; 52];
        for (idx, j) in i.iter().enumerate() {
            if idx < i.len() / 2 {
                count[(*j as usize) - 1] += 1;
            }
            else {
                if count[(*j as usize) - 1] != 0 {
                    result.push(*j);
                    break;
                }
            }
        }
    }
    result
}

fn find_common2(items: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![];
    let mut count = vec![vec![0;52], vec![0;52]];
    for (idx, i) in items.iter().enumerate() {
        if idx % 3 == 0 {
            count.iter_mut().for_each(|x| x.iter_mut().for_each(|y| *y = 0));
        }
        if idx % 3 != 2 {
            for j in i.iter() {
                count[(idx % 3) as usize][(*j as usize) - 1] += 1;
            }
        }
        else {
            for j in i.iter() {
                if count[0][(*j as usize) - 1] > 0 && count[1][(*j as usize) - 1] > 0 {
                    result.push(*j);
                    break;
                }
            }
        }
    }
    result
}

fn main() {
    let items = read_file("input.txt".to_string());
    let unique_items = find_common1(&items);
    println!("{:?}", unique_items.iter().sum::<i32>());
    let unique_items = find_common2(&items);
    println!("{:?}", unique_items.iter().sum::<i32>());
}
