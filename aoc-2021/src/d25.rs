use std::io::{stdin, Read};

pub fn p01() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut map: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        let mut row: Vec<char> = vec![];
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }

    println!("{:?}", map);

    // let new_map: Vec<Vec<char>> = vec![];
    // let mut step = 0;
    // loop {

    // }
}

pub fn p02() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
}
