use std::io;
use std::collections::HashMap;
use std::cmp::max;

fn part1(v: &Vec<String>) -> Option<u32> {
  let mut current_path = String::from("/");
  let mut paths: HashMap<String, u32> = HashMap::new();
  paths.insert(current_path.clone(), 0);
  for command in v {
    if command.starts_with("$ cd") {
      let path = command[5..command.len()].to_string();
      if path.starts_with("/") {
        current_path = path;
      } else if path.starts_with(".."){
        if current_path == "/"{
          continue;
        }
        current_path.pop();
        while *current_path.as_bytes().last().unwrap() != '/' as u8 {
          current_path.pop();
        }
      } else {
        current_path.push_str(&path);
        current_path.push_str(&String::from("/"));
      }
      paths.insert(current_path.clone(), 0);
    } else if command.starts_with("dir"){
      let stuff = command.split_once(" ").unwrap();
      let path = current_path.clone() + stuff.1 + "/";
      paths.insert(path, 0);
    }else if !command.starts_with("$ ls"){
      let stuff = command.split_once(" ").unwrap();
      let size = stuff.0.parse::<u32>().unwrap();
      let path = current_path.clone() + stuff.1;
      paths.insert(path, size);
    }
  }
  let mut answer = 0;
  for first_path in paths.keys(){
    if *paths.get(first_path).unwrap() > 0{
      continue ;
    }
    
    let mut size:u32 = 0;

    for second_path in paths.keys() {
      if second_path.starts_with(first_path){
        size += paths.get(second_path).unwrap();
      }
    }
    if size <= 100000 {
      answer += size;
    }
  }

  return Some(answer);
}

fn part2(v: &Vec<String>) -> Option<u32> {
  let mut current_path = String::from("/");
  let mut paths: HashMap<String, u32> = HashMap::new();
  paths.insert(current_path.clone(), 0);
  for command in v {
    if command.starts_with("$ cd") {
      let path = command[5..command.len()].to_string();
      if path.starts_with("/") {
        current_path = path;
      } else if path.starts_with(".."){
        if current_path == "/"{
          continue;
        }
        current_path.pop();
        while *current_path.as_bytes().last().unwrap() != '/' as u8 {
          current_path.pop();
        }
      } else {
        current_path.push_str(&path);
        current_path.push_str(&String::from("/"));
      }
      paths.insert(current_path.clone(), 0);
    } else if command.starts_with("dir"){
      let stuff = command.split_once(" ").unwrap();
      let path = current_path.clone() + stuff.1 + "/";
      paths.insert(path, 0);
    }else if !command.starts_with("$ ls"){
      let stuff = command.split_once(" ").unwrap();
      let size = stuff.0.parse::<u32>().unwrap();
      let path = current_path.clone() + stuff.1;
      paths.insert(path, size);
    }
  }
  let mut occupied_size = 0;
  for second_path in paths.keys() {
    if second_path.starts_with("/"){
      occupied_size += paths.get(second_path).unwrap();
    }
  }
  let min_size = max(0, 30000000 - (70000000 - occupied_size));
  let mut best_size = occupied_size;
  for first_path in paths.keys(){
    if *paths.get(first_path).unwrap() > 0{
      continue ;
    }
    
    let mut size:u32 = 0;

    for second_path in paths.keys() {
      if second_path.starts_with(first_path){
        size += paths.get(second_path).unwrap();
      }
    }
    if size > min_size && size < best_size{
      best_size = size;
    }
  }

  return Some(best_size);
}

pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string())
                        .collect::<Vec<String>>();
  println!("Day 7 Part 1: {}",part1(&v).unwrap());
  println!("Day 7 Part 2: {}",part2(&v).unwrap());
}
