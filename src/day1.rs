use std::io;

fn part1(v: &Vec<String>) -> u32 {
    let values:Vec<u32> = v.split(|x| x.trim() == "").into_iter().map(|x| x.into_iter().map(|y| y.parse::<u32>().unwrap())).map(|x| x.sum()).collect::<Vec<u32>>();
    return *values.iter().max().unwrap();
}

fn part2(v: &Vec<String>) -> u32 {
    let mut values:Vec<u32> = v.split(|x| x.trim() == "").into_iter().map(|x| x.into_iter().map(|y| y.parse::<u32>().unwrap())).map(|x| x.sum()).collect::<Vec<u32>>();
    values.sort();
    values.reverse();
    return values[0] + values[1] + values[2];
}

pub fn run() {
    let stdin = io::stdin();
    let v = stdin.lines().map(|x| x.unwrap().trim().to_string()).collect::<Vec<String>>();
    println!("Day 1 Part 1: {}",part1(&v));
    println!("Day 1 Part 2: {}",part2(&v));
}
