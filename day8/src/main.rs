use std::fs;
use std::cmp;

fn read_file(filename: String) -> Vec<Vec<i32>> {
    let contents: String = fs::read_to_string(filename).unwrap();
    contents.strip_suffix('\n').unwrap().split('\n')
    .map(
        |x: &str| x.chars().map(
            |y| y.to_digit(10).unwrap() as i32
        ).collect::<Vec<i32>>()
    ).collect::<Vec<Vec<i32>>>()
}

fn build_visibility(trees: &Vec<Vec<i32>>) -> Vec<Vec<i8>> {
    let rows = trees.len();
    let cols = trees[0].len();
    let mut result = vec![vec![0; cols]; rows];
    // left, right, up, down
    // 1,    2,     4,  8
    // horizontal
    for i in 0..rows {
        let mut max = -1;
        for j in 0..cols {
            if trees[i][j] > max {
                result[i][j] += 1;
                max = trees[i][j];
            }
        }
        max = -1;
        for j in 0..cols {
            if trees[i][cols - 1 - j] > max {
                result[i][cols - 1 - j] += 2;
                max = trees[i][cols - 1 - j];
            }
        }
    }
    //vertical 
    for i in 0..cols {
        let mut max = -1;
        for j in 0..rows {
            if trees[j][i] > max {
                result[j][i] += 4;
                max = trees[j][i];
            }
        }
        max = -1;
        for j in 0..rows {
            if trees[rows - 1 - j][i] > max {
                result[rows - 1 - j][i] += 8;
                max = trees[rows - 1 - j][i];
            }
        }
    }
    return result;
}

fn scenic_score(trees: &Vec<Vec<i32>>, vis: &Vec<Vec<i8>>) -> Vec<Vec<usize>> {
    let rows = trees.len();
    let cols = trees[0].len();
    let mut result = vec![vec![1; cols]; rows];
    // left, right, up, down
    // 1,    2,     4,  8
    for i in 0..rows {
        for j in 0..cols {
            if vis[i][j] & 0x1 > 0 {
                result[i][j] *= j;
            }
            else {
                let mut k = j - 1;
                while trees[i][k] < trees[i][j] {
                    k -= 1;
                }
                result[i][j] *= j - k;
            }
            if vis[i][j] & 0x2 > 0 {
                result[i][j] *= cols - j - 1;
            }
            else {
                let mut k = j + 1;
                while trees[i][k] < trees[i][j] {
                    k += 1;
                }
                result[i][j] *= k - j;
            }

            if vis[i][j] & 0x4 > 0 {
                result[i][j] *= i;
            }
            else {
                let mut k = i - 1;
                while trees[k][j] < trees[i][j] {
                    k -= 1;
                }
                result[i][j] *= i - k;
            }
            if vis[i][j] & 0x8 > 0 {
                result[i][j] *= rows - i - 1;
            }
            else {
                let mut k = i + 1;
                while trees[k][j] < trees[i][j] {
                    k += 1;
                }
                result[i][j] *= k - i;
            }
        }
    }
    result
}


fn main() {
    let trees = read_file("input.txt".to_string());
    let visibility = build_visibility(&trees);
    let score = scenic_score(&trees, &visibility);
    println!("{:?}", visibility.iter().flatten().fold(0, |acc, x| acc + (*cmp::min(x, &1) as i32) ));
    println!("{:?}", score.iter().flatten().max().unwrap());
}
