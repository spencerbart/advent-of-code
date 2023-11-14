use std::io::{stdin, Read};

pub fn p01() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut previous = input.lines().next().unwrap().parse().unwrap();
    let mut count = 0;
    for line in input.lines() {
        let num: i32 = line.parse().unwrap();
        if num > previous {
            count += 1;
        }
        previous = num;
    }
    println!("{}", count);
}

pub fn p02() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let nums = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut count = 0;

    for i in 0..nums.len()-3 {
        let previous = nums[i] + nums[i + 1] + nums[i + 2];
        let current = nums[i + 1] + nums[i + 2] + nums[i + 3];
        if current > previous {
            count += 1;
        }
    }

    println!("{}", count);
}
