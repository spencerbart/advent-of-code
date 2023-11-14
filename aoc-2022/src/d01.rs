use std::{
    io::{stdin, Read},
    vec,
};

pub fn p01() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut max = 0;
    let mut count = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            if count > max {
                max = count;
            }
            count = 0;
        } else {
            let num = line.trim().parse::<u32>().unwrap();
            count += num;
        }
    }
    if count > max {
        max = count;
    }
    println!("{}", max);
}

pub fn p02() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut sums = vec![];
    let mut count = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            sums.push(count);
            count = 0;
        } else {
            let num = line.trim().parse::<u32>().unwrap();
            count += num;
        }
    }
    sums.push(count);
    sums.sort();
    // sum the top 3
    if sums.len() < 3 {
        println!("{}", sums.iter().sum::<u32>());
        return;
    }
    println!(
        "{}",
        sums[sums.len() - 1] + sums[sums.len() - 2] + sums[sums.len() - 3]
    );
}
