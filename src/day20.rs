use std::io;

fn part1(v: &Vec<i32>) -> Option<i32> {
  let mut order: Vec<usize> = Vec::new();

  for i in 0..v.len() {
    order.push(i);
  }

  for i in 0..v.len() {
    let mut position = order.iter().position(|x| *x == i).unwrap();
    let mut value = v[i];
    while value > 0 {
      value -= 1;
      let next_position = (position + 1) % order.len();
      order.swap(position, next_position);
      position = next_position;
    }

    while value < 0 {
      value += 1;
      let next_position = (position + (order.len() - 1)) % order.len();
      order.swap(position, next_position);
      position = next_position;
    }
  }

  let zero_original_index = v.iter().position(|x| *x == 0).unwrap();
  let zero_index = order.iter().position(|x| *x==zero_original_index).unwrap();
  let mut answer = 0;

  for i in 1..=3{
    answer += v[order[(zero_index + i * 1000) % order.len()]];
  }

  return Some(answer);
}

fn part2(v: &Vec<i32>) -> Option<i64> {
  let encryption_key = 811589153;
  let mut order: Vec<usize> = Vec::new();

  for i in 0..v.len() {
    order.push(i);
  }

  for _ in 0..10{
    for i in 0..v.len() {
      let mut position = order.iter().position(|x| *x == i).unwrap();
      let mut value = ((v[i] as i64) * encryption_key) % (v.len() as i64 - 1);
      while value > 0 {
        value -= 1;
        let next_position = (position + 1) % order.len();
        order.swap(position, next_position);
        position = next_position;
      }

      while value < 0 {
        value += 1;
        let next_position = (position + (order.len() - 1)) % order.len();
        order.swap(position, next_position);
        position = next_position;
      }
    }
  }
  let zero_original_index = v.iter().position(|x| *x == 0).unwrap();
  let zero_index = order.iter().position(|x| *x==zero_original_index).unwrap();
  let mut answer = 0;

  for i in 1..=3{
    answer += (v[order[(zero_index + i * 1000) % order.len()]] as i64) * encryption_key;
  }

  return Some(answer);
}
  
pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string())
                       .filter(|x| x.len() > 0)
                       .map(|x| x.parse::<i32>().unwrap())
                       .collect::<Vec<i32>>();
  println!("Day 20 Part 1: {}",part1(&v).unwrap());
  println!("Day 20 Part 2: {}",part2(&v).unwrap());
}
