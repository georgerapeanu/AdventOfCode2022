use std::io;
use std::cmp::max;
use std::collections::HashSet;
fn part1(v: &Vec<Vec<(u32, u32)>>) -> Option<u32> {
  let maximum_row = v.iter().flatten().fold(0, |acc, x| max(acc,x.1));
  let mut occupied_set: HashSet<(u32, u32) > =   HashSet::new();

  for path in v{
    if path.is_empty() {
      continue;
    }
    let mut last:(u32, u32) = *path.first().unwrap();
    occupied_set.insert(last);
    for elem in path {
      let current = *elem;
      while last.0 > current.0 {
        last.0 -= 1;
        occupied_set.insert(last);
      }
      while last.0 < current.0 {
        last.0 += 1;
        occupied_set.insert(last);
      }
      while last.1 > current.1 {
        last.1 -= 1;
        occupied_set.insert(last);
      }
      while last.1 < current.1 {
        last.1 += 1;
        occupied_set.insert(last);
      }
    }
  }
  let sand_start = (500, 0);
  let propagate_directions: Vec<(i32, i32)> = vec![(0, 1), (-1, 1), (1, 1)];
  let mut answer = 0;
  loop{
    let mut start = sand_start;
    if occupied_set.contains(&start){
      break;
    }
    let mut succesful = false;
    loop {
      if start.1 > maximum_row{
        break;
      }
      let mut next_start = start;
      for dir in propagate_directions.iter(){
        let next_i32 = (start.0 as i32 + dir.0, start.1 as i32 + dir.1);
        if next_i32.0 < 0 || next_i32.1 < 0 {
          continue;
        }
        let next = (next_i32.0 as u32, next_i32.1 as u32);
        if occupied_set.contains(&next) {
          continue;
        }
        next_start = next;
        break;
      }
      if next_start == start {
        answer += 1;
        occupied_set.insert(start);
        succesful = true;
        break;
      }
      start = next_start;
    }
    if !succesful{
      break;
    }
  }
  return Some(answer);
}

fn part2(v: &Vec<Vec<(u32, u32)>>) -> Option<u32> {
  let maximum_row = v.iter().flatten().fold(0, |acc, x| max(acc,x.1));
  let mut occupied_set: HashSet<(u32, u32) > =   HashSet::new();

  for path in v{
    if path.is_empty() {
      continue;
    }
    let mut last:(u32, u32) = *path.first().unwrap();
    occupied_set.insert(last);
    for elem in path {
      let current = *elem;
      while last.0 > current.0 {
        last.0 -= 1;
        occupied_set.insert(last);
      }
      while last.0 < current.0 {
        last.0 += 1;
        occupied_set.insert(last);
      }
      while last.1 > current.1 {
        last.1 -= 1;
        occupied_set.insert(last);
      }
      while last.1 < current.1 {
        last.1 += 1;
        occupied_set.insert(last);
      }
    }
  }
  let sand_start = (500, 0);
  let mut queue = vec![sand_start];

  let mut i = 0;
  let propagate_directions: Vec<(i32, i32)> = vec![(0, 1), (-1, 1), (1, 1)];
  while i < queue.len(){
    let current = queue[i];
    for dir in propagate_directions.iter(){
      let next_i32 = (current.0 as i32 + dir.0, current.1 as i32 + dir.1);
      if next_i32.0 < 0 || next_i32.1 < 0 {
        continue;
      }
      let next = (next_i32.0 as u32, next_i32.1 as u32);
      if occupied_set.contains(&next) || next.1 > maximum_row + 1{
        continue;
      }
      queue.push(next);
      occupied_set.insert(next);
    }
    i += 1;
  }
  return Some(queue.len() as u32);
}
  
pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string())
                       .map(|x| x.split("->").into_iter()
                                 .map(|x| x.trim()
                                           .split(",").into_iter()
                                                      .map(|x| x.parse::<u32>().unwrap())
                                                      .collect::<Vec<u32>>()
                                           .chunks(2)
                                           .map(|x| (x[0], x[1]))
                                           .collect::<Vec<(u32, u32)> >()
                                 )
                                 .flatten()
                                 .collect::<Vec<(u32, u32)>>()
                        )
                        .collect::<Vec<Vec<(u32, u32)>>>();

              
  println!("Day 14 Part 1: {}",part1(&v).unwrap());
  println!("Day 14 Part 2: {}",part2(&v).unwrap());
}
