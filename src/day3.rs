use std::io;
use std::collections::HashMap;


fn part1(v: &Vec<String>) -> u32 {
  let mut char_to_priority: HashMap<char, u32> = HashMap::new();

  for i in 'a'..='z' {
    char_to_priority.insert(i, i as u32 - 'a' as u32 + 1);
  }
  
  for i in 'A'..='Z' {
    char_to_priority.insert(i, i as u32 - 'A' as u32 + 27);
  }

  let mut answer:u32 = 0;
  for current in v{
    let current_bytes = current.as_bytes();
    let last_first = current.len() / 2;
    let mut appears:Vec<i32> = vec![0; 53];
    for i in 0..current_bytes.len(){
      if i < last_first{
        appears[*char_to_priority.get(&(current_bytes[i] as char)).unwrap() as usize] |= 1;
      } else {
        appears[*char_to_priority.get(&(current_bytes[i] as char)).unwrap() as usize] |= 2;
      }
    }
    for i in 0..appears.len(){
      if appears[i] == 3 {
        answer += i as u32;
      }
    }
  }
  return answer;
}

fn part2(v: &Vec<String>) -> u32 {
  let mut char_to_priority: HashMap<char, u32> = HashMap::new();

  for i in 'a'..='z' {
    char_to_priority.insert(i, i as u32 - 'a' as u32 + 1);
  }
  
  for i in 'A'..='Z' {
    char_to_priority.insert(i, i as u32 - 'A' as u32 + 27);
  }

  let mut answer:u32 = 0;
  for current_index in (0..v.len()).step_by(3){
    let mut appears:Vec<i32> = vec![0; 53];
    for group_index in 0..3 {
      let current_bytes = v[current_index + group_index].as_bytes();
      for i in 0..current_bytes.len(){
        appears[*char_to_priority.get(&(current_bytes[i] as char)).unwrap() as usize] |= 1 << group_index;
      }
    }
    for i in 0..appears.len(){
      if appears[i] == 7 {
        answer += i as u32;
      }
    }
  }
  return answer;
}

pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().trim().to_string()).collect::<Vec<String>>();
  println!("Day 3 Part 1: {}",part1(&v));
  println!("Day 3 Part 2: {}",part2(&v));
}
