use std::collections::VecDeque;
use std::fs;

fn read_file(filename: String) -> (Vec<VecDeque<char>>, Vec<Vec<i32>>) {
    let contents: String = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = contents.split('\n').collect();
    let cols: usize = (lines[0].len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); cols];
    lines.iter().take_while(|x| x.contains('[')).for_each(|x| {
        x.chars()
            .enumerate()
            .filter(|(_, c)| c.is_ascii_alphabetic())
            .for_each(|(i, c)| {
                stacks[i / 4].push_front(c);
            });
    });
    let mut ops: Vec<Vec<i32>> = vec![];
    lines
        .iter()
        .skip_while(|x| !x.contains("move"))
        .take_while(|x| x.contains("move"))
        .for_each(|x| {
            let mut temp = vec![];
            x.split(' ')
                .enumerate()
                .filter(|(i, _)| i % 2 == 1)
                .for_each(|(_, s)| {
                    temp.push(s.parse::<i32>().unwrap());
                });
            ops.push(temp);
        });
    (stacks, ops)
}

fn move_stack(stacks: &mut Vec<VecDeque<char>>, op: &Vec<i32>, reverse: bool) {
    let (from, to) = (op[1] as usize - 1, op[2] as usize - 1);
    let quantity = op[0] as usize;
    let len = stacks[from].len();
    if reverse {
        let mut temp = stacks[from].drain(len - quantity..).rev().collect();
        stacks[to].append(&mut temp);
    }
    else {
        let mut temp = stacks[from].drain(len - quantity..).collect();
        stacks[to].append(&mut temp);
    }
}


fn main() {
    let (mut stacks, ops) = read_file("input.txt".to_string());
    let mut stacks2 = stacks.clone();
    ops.iter().for_each(|x| move_stack(&mut stacks, x, true));
    stacks.iter().for_each(|x| {
        print!("{}", x[x.len() - 1]);
    });
    print!("\n");
    ops.iter().for_each(|x| move_stack(&mut stacks2, x, false));
    stacks2.iter().for_each(|x| {
        print!("{}", x[x.len() - 1]);
    });
    print!("\n");
}
