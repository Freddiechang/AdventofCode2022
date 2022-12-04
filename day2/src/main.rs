use std::fs;

fn read_file(filename: String) -> Vec<(i32, i32)> {
    let contents: String = fs::read_to_string(filename).unwrap();
    let mut results: Vec<(i32, i32)> = vec![];
    let f = |x: &str| {
        let hands = x.split(' ').collect::<Vec<_>>();
        if hands.len() == 2 {
            if let Some(c1) = hands[0].chars().next() {
                if let Some(c2) = hands[1].chars().next() {
                    results.push( ( ((c1 as u8) - ('A' as u8)) as i32, ((c2 as u8) - ('X' as u8)) as i32 ) )
                }
            }
        }
        ()
    };
    contents.split('\n').map(f).collect::<Vec<_>>();
    results
}

fn process_hands1(hands: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let f = |x: &(i32, i32)| {
        let my = x.1 + 1;
        let outcome = match x.1 - x.0 {
            0 => 3,
            1 => 6,
            2 => 0,
            -1 => 0,
            -2 => 6,
            _ => 0,
        };
        (my, outcome)
    };
    return hands.iter().map(f).collect();
}

fn process_hands2(hands: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let f = |x: &(i32, i32)| {
        let outcome = x.1 * 3;
        let my = match x.1 {
            0 => (x.0 + 2) % 3 + 1,
            1 => x.0 + 1,
            2 => (x.0 + 1) % 3 + 1,
            _ => 0
        };
        (my, outcome)
    };
    return hands.iter().map(f).collect();
}

fn main() {
    let hands = read_file("input.txt".to_string());
    let scores = process_hands1(&hands);
    println!("{:?}", scores.iter().map(|x| x.0 + x.1).sum::<i32>());
    let scores = process_hands2(&hands);
    println!("{:?}", scores.iter().map(|x| x.0 + x.1).sum::<i32>());
}
