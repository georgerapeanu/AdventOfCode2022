use std::io;
use std::collections::HashSet;
use std::cmp::{min, max};

fn part1(v: &Vec<Vec<i32>>) -> Option<usize> {
  let mut cube_set: HashSet<Vec<i32>> = HashSet::new();
  for i in 0.. v.len(){
    cube_set.insert(v[i].clone());
  }
  let mut answer = 0;
  for i in 0..v.len() {
    for j in 0..v[i].len(){
      for k in (-1..=1).step_by(2){
        let mut neighbor = v[i].clone();
        neighbor[j] += k;
        if !cube_set.contains(&neighbor) {
          answer += 1;
        }
      }
    }
  }
  return Some(answer);
}

fn part2(v: &Vec<Vec<i32>>) -> Option<usize> {
  let mut cube_set: HashSet<Vec<i32>> = HashSet::new();
  let mut visited_set: HashSet<Vec<i32>> = HashSet::new();
  let mut bounding_box = vec![vec![std::i32::MAX, std::i32::MAX, std::i32::MAX], vec![std::i32::MIN, std::i32::MIN, std::i32::MIN]];
  for i in 0.. v.len(){
    cube_set.insert(v[i].clone());
    for j in 0..v[i].len(){
      bounding_box[0][j] = min(bounding_box[0][j], v[i][j] - 1);
      bounding_box[1][j] = max(bounding_box[1][j], v[i][j] + 1);
    }
  }
  let mut q = vec![bounding_box[0].clone()];
  visited_set.insert(bounding_box[0].clone());
  let mut i = 0;
  let mut answer = 0;
  while i < q.len() {
    let current = q[i].clone();
    for j in 0..current.len(){
      for k in (-1..=1).step_by(2){
        let mut neighbor = current.clone();
        neighbor[j] += k;
        if neighbor[j] < bounding_box[0][j] || neighbor[j] > bounding_box[1][j]{
          continue;
        }
        if cube_set.contains(&neighbor) {
          answer += 1;
        } else if !visited_set.contains(&neighbor) {
          visited_set.insert(neighbor.clone());
          q.push(neighbor);
        }
      }
    } 
    i += 1;
  }

  return Some(answer);
}
  
pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string())
                       .map(|x| x.split(",").into_iter().map(|x| x.trim().to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>())
                       .collect::<Vec<Vec<i32>>>();
  println!("Day 18 Part 1: {}",part1(&v).unwrap());
  println!("Day 18 Part 2: {}",part2(&v).unwrap());
}
