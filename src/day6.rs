use std::io;


fn part1(v: &String) -> Option<usize> {
  let v_bytes = v.as_bytes();
  for i in 0..v_bytes.len() - 3 {
    let mut found = false;
    for j in 0..4{
      for k in j + 1..4{
        if v_bytes[i + j] == v_bytes[i + k] {
          found = true;
        }
      }
    }
    if !found {
      return Some(i + 4);
    }
  }
  return None
}

fn part2(v: &String) -> Option<usize> {
  let v_bytes = v.as_bytes();
  for i in 0..v_bytes.len() - 3 {
    let mut found = false;
    for j in 0..14{
      for k in j + 1..14{
        if v_bytes[i + j] == v_bytes[i + k] {
          found = true;
        }
      }
    }
    if !found {
      return Some(i + 14);
    }
  }
  return None
}

pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string())
                        .collect::<Vec<String>>().remove(0);
  println!("Day 6 Part 1: {}",part1(&v).unwrap());
  println!("Day 6 Part 2: {}",part2(&v).unwrap());
}
