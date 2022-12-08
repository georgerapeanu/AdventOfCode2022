use std::io;
use std::cmp::max;

fn part1(v: &Vec<String>) -> Option<u32> {
  let d: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
  let mut answer = 0;
  for i in 0..v.len(){
    for j in 0..v[i].len(){
     let mut found = false;
     for k in 0..d.len(){
      let mut __max = -1;
      let mut x = i as i32;
      let mut y = j as i32;
      loop{
        x += d[k].0;
        y += d[k].1;
        if x < 0 || x >= v.len() as i32 || y < 0 || y >= v[x as usize].len() as i32 {
          break;
        }
         __max = max(__max, v[x as usize].as_bytes()[y as usize] as i32);
      }
      if __max < v[i as usize].as_bytes()[j as usize] as i32 {
        found = true;
        break;
      }
     }
     answer += if found {1} else {0};
    }
  }
  return Some(answer); 
}

fn part2(v: &Vec<String>) -> Option<u32> {
  let d: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
  let mut answer = 0;
  for i in 0..v.len(){
    for j in 0..v[i].len(){
     let mut cell_answer = 1;
     for k in 0..d.len(){
      let mut x = i as i32;
      let mut y = j as i32;
      let mut current_answer = 0;
      loop{
        x += d[k].0;
        y += d[k].1;
        if x < 0 || x >= v.len() as i32 || y < 0 || y >= v[x as usize].len() as i32 {
          break;
        }
        current_answer += 1;
        if v[x as usize].as_bytes()[y as usize] >= v[i as usize].as_bytes()[j as usize]{
          break;
        }
      }
      cell_answer *= current_answer;
     }
     answer = max(answer, cell_answer);
    }
  }
  return Some(answer); 
}

pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string())
                        .collect::<Vec<String>>();
  println!("Day 8 Part 1: {}",part1(&v).unwrap());
  println!("Day 8 Part 2: {}",part2(&v).unwrap());
}
