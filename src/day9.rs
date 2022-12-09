use std::io;
use std::collections::{HashMap, HashSet};
use map_macro::map; 

fn part1(v: &Vec<(String, u32)>) -> Option<u32> {
  let directions:HashMap<String, (i32, i32)> = map!{
    String::from("U") => (-1, 0),
    String::from("R") => (0, 1),
    String::from("D") => (1, 0),
    String::from("L") => (0, -1),
  };
  let mut head_x = 0;
  let mut head_y = 0; 
  let mut tail_x = 0;
  let mut tail_y = 0;

  let mut visited: HashSet<(i32, i32)> = HashSet::new();
  visited.insert((tail_x, tail_y)); 
  for elem in v{
    for _ in 0..elem.1{
      head_x += directions.get(&elem.0).unwrap().0;
      head_y += directions.get(&elem.0).unwrap().1;
      if (head_x - tail_x).abs() > 1 || (head_y - tail_y).abs() > 1 {
        if head_x != tail_x || head_y != tail_y {
          tail_x += if head_x - tail_x > 0 {1} else if head_x - tail_x < 0 {-1} else {0};
          tail_y += if head_y - tail_y > 0 {1} else if head_y - tail_y < 0 {-1} else {0};
        }
      }
      visited.insert((tail_x, tail_y)); 
    }
  }

  return Some(visited.len() as u32);
}

fn part2(v: &Vec<(String, u32)>) -> Option<u32> {
  let directions:HashMap<String, (i32, i32)> = map!{
    String::from("U") => (-1, 0),
    String::from("R") => (0, 1),
    String::from("D") => (1, 0),
    String::from("L") => (0, -1),
  };
  let mut cells = vec![(0, 0);10];

  let mut visited: HashSet<(i32, i32)> = HashSet::new();
  visited.insert(*cells.last().unwrap()); 
  for elem in v{
    for _ in 0..elem.1{
      cells[0].0 += directions.get(&elem.0).unwrap().0;
      cells[0].1 += directions.get(&elem.0).unwrap().1;
      for i in 0..cells.len() - 1{
        if (cells[i].0 - cells[i + 1].0).abs() > 1 || (cells[i].1 - cells[i + 1].1).abs() > 1 {
          if cells[i].0 != cells[i + 1].0 || cells[i].1 != cells[i + 1].1 {
            cells[i + 1].0 += if cells[i].0 - cells[i + 1].0 > 0 {1} else if cells[i].0 - cells[i + 1].0 < 0 {-1} else {0};
            cells[i + 1].1 += if cells[i].1 - cells[i + 1].1 > 0 {1} else if cells[i].1 - cells[i + 1].1 < 0 {-1} else {0};
          }
        }
      }
      visited.insert(*cells.last().unwrap()); 
    }
  }

  return Some(visited.len() as u32);
}

pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string())
                       .map(|x| x.split(" ").map(|y| y.trim().to_string())
                       .collect::<Vec<String>>())
                       .map(|x| (x[0].clone(), x[1].parse::<u32>().unwrap()))
                       .collect::<Vec<(String, u32)>>();
  println!("Day 9 Part 1: {}",part1(&v).unwrap());
  println!("Day 9 Part 2: {}",part2(&v).unwrap());
}
