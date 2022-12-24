use std::io;
use std::cmp::{min, max};
use std::collections::HashMap;

fn part1(v: &Vec<String>) -> Option<u64> {
  let dirs: Vec<Vec<(isize, isize)>> = vec![vec![(-1, -1), (-1, 0), (-1, 1)], vec![(1, -1), (1, 0), (1, 1)], vec![(-1, -1), (0, -1), (1, -1)], vec![(-1, 1), (0, 1), (1, 1)]];
  let round_count = 10;
  let mut elves: HashMap<(isize, isize), usize>  = HashMap::new();

  for i in 0..v.len(){
    for j in 0..v[i].len(){
      if v[i].as_bytes()[j] == '#' as u8{
        elves.insert((i as isize, j as isize), 1);
      }
    }
  }

  for i in 0..round_count{
    let mut movement: Vec<((isize, isize), (isize, isize))> = Vec::new();
    for elf in elves.keys() {
      movement.push((*elf, *elf));
      let mut has_neighbor = false;
      for dx in -1..=1 {
        for dy in -1..=1 {
          if dx == 0 && dy == 0 {
            continue;
          }
          if elves.contains_key(&(elf.0 + dx, elf.1 + dy)) {
            has_neighbor = true;
          }
        }
      }
      if !has_neighbor {
        continue;
      }
      for j in 0..dirs.len() {
        let mut ok = true;
        for dir in dirs[(i + j) % dirs.len()].iter() {
          if elves.contains_key(&(elf.0 + dir.0, elf.1 + dir.1)) {
            ok = false;
            break;
          }
        }
        if ok {
          movement.pop();
          movement.push(((elf.0 + dirs[(i + j) % dirs.len()][1].0, elf.1 + dirs[(i + j) % dirs.len()][1].1), *elf));
          break;
        }
      }
    }
    movement.sort();
    for i in 0..movement.len() {
      if i > 0 && movement[i].0 == movement[i - 1].0 {
        continue;
      }
      if i + 1 < movement.len() && movement[i].0 == movement[i + 1].0 {
        continue;
      }
      movement[i].1 = movement[i].0;
    }
    elves.clear();
    for elem in movement{
      elves.insert(elem.1, 1);
    }
  }
  
  let mut bounding_box = ((std::isize::MAX, std::isize::MAX), (std::isize::MIN, std::isize::MIN));
  
  for elf in elves.keys(){
    bounding_box.0.0 = min(bounding_box.0.0, elf.0);
    bounding_box.1.0 = max(bounding_box.1.0, elf.0);
    bounding_box.0.1 = min(bounding_box.0.1, elf.1);
    bounding_box.1.1 = max(bounding_box.1.1, elf.1);
  }

  return Some(((bounding_box.1.0 - bounding_box.0.0 + 1) as u64 * (bounding_box.1.1 - bounding_box.0.1 + 1) as u64) - elves.len() as u64);
}

fn part2(v: &Vec<String>) -> Option<usize> {
  let dirs: Vec<Vec<(isize, isize)>> = vec![vec![(-1, -1), (-1, 0), (-1, 1)], vec![(1, -1), (1, 0), (1, 1)], vec![(-1, -1), (0, -1), (1, -1)], vec![(-1, 1), (0, 1), (1, 1)]];
  let mut elves: HashMap<(isize, isize), usize>  = HashMap::new();

  for i in 0..v.len(){
    for j in 0..v[i].len(){
      if v[i].as_bytes()[j] == '#' as u8{
        elves.insert((i as isize, j as isize), 1);
      }
    }
  }

  let mut i = 0;
  loop{
    let mut movement: Vec<((isize, isize), (isize, isize))> = Vec::new();
    for elf in elves.keys() {
      movement.push((*elf, *elf));
      let mut has_neighbor = false;
      for dx in -1..=1 {
        for dy in -1..=1 {
          if dx == 0 && dy == 0 {
            continue;
          }
          if elves.contains_key(&(elf.0 + dx, elf.1 + dy)) {
            has_neighbor = true;
          }
        }
      }
      if !has_neighbor {
        continue;
      }
      for j in 0..dirs.len() {
        let mut ok = true;
        for dir in dirs[(i + j) % dirs.len()].iter() {
          if elves.contains_key(&(elf.0 + dir.0, elf.1 + dir.1)) {
            ok = false;
            break;
          }
        }
        if ok {
          movement.pop();
          movement.push(((elf.0 + dirs[(i + j) % dirs.len()][1].0, elf.1 + dirs[(i + j) % dirs.len()][1].1), *elf));
          break;
        }
      }
    }
    movement.sort();
    let mut has_moved = false;
    for i in 0..movement.len() {
      if i > 0 && movement[i].0 == movement[i - 1].0 {
        continue;
      }
      if i + 1 < movement.len() && movement[i].0 == movement[i + 1].0 {
        continue;
      }
      if movement[i].0 != movement[i].1 {
        has_moved = true;
      }
      movement[i].1 = movement[i].0;
    }
    elves.clear();
    for elem in movement{
      elves.insert(elem.1, 1);
    }
    i += 1;
    if !has_moved {
      break;
    }
  }
 
  return Some(i);
}
  
pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string())
                       .filter(|x| x.len() > 0)
                       .collect::<Vec<String>>();
  println!("Day 23 Part 1: {}",part1(&v).unwrap());
  println!("Day 23 Part 2: {}",part2(&v).unwrap());
}
