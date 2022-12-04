use std::io;


fn part1(v: &Vec<(u32, u32, u32, u32)>) -> u32 {
  return v.into_iter().filter(|x| (x.0 <= x.2 && x.3 <= x.1) || (x.2 <= x.0 && x.1 <= x.3)).collect::<Vec<&(u32, u32, u32, u32)>>().len() as u32;
}

fn part2(v: &Vec<(u32, u32, u32, u32)>) -> u32 {
  return v.into_iter().filter(|x| !(x.1 < x.2 || x.0 > x.3)).collect::<Vec<&(u32, u32, u32, u32)>>().len() as u32;
}

pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().trim().to_string())
                        .map(|x| x.split(",").map(|x| x.to_string()).collect::<Vec<String>>()).map(|x| (x[0].clone(), x[1].clone()))
                        .map(|x| (x.0.split("-").map(|x| x.to_string()).collect::<Vec<String>>(), x.1.split("-").map(|x| x.to_string()).collect::<Vec<String>>()))
                        .map(|x| (x.0[0].clone(), x.0[1].clone(), x.1[0].clone(), x.1[1].clone()))
                        .map(|x| (x.0.parse::<u32>().unwrap(), x.1.parse::<u32>().unwrap(), x.2.parse::<u32>().unwrap(), x.3.parse::<u32>().unwrap()))
                        .collect::<Vec<(u32, u32, u32, u32)>>();
  println!("Day 4 Part 1: {}",part1(&v));
  println!("Day 4 Part 2: {}",part2(&v));
}
