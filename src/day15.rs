use std::io;
use std::cmp::{min, max};
fn part1(v: &Vec<((i32, i32), (i32, i32))>) -> Option<u32> {
  let target_y = 2000000;
  let mut segments: Vec<(i32, i32)> = Vec::new();
  for row in v.iter() {
    let sensor_location = row.0;
    let beacon_location = row.1;
    let distance = (sensor_location.0 - beacon_location.0).abs() + (sensor_location.1 - beacon_location.1).abs();
    if (target_y - sensor_location.1).abs() <= distance {
     let dy = target_y - sensor_location.1;
     let dx = (distance - dy.abs()).abs();
     segments.push((sensor_location.0 - dx, -1));
     segments.push((sensor_location.0 + dx + 1, 1));
    }
  }
  let mut beacon_locations = v.into_iter().filter(|x| x.1.1 == target_y).map(|x| x.1).collect::<Vec<(i32, i32)>>(); 
  beacon_locations.dedup();
  let beacon_count = beacon_locations.len() as i32;
  segments.sort(); 

  let mut last_x = 0;
  let mut active = 0;
  let mut answer = -beacon_count;

  for segment in segments{
    if segment.1 < 0 {
      if active > 0 {
        answer += segment.0 - last_x; 
      }
      active += 1;
    } else {
      if active > 0 {
        answer += segment.0 - last_x; 
      }
      active -= 1;
    }
    last_x = segment.0;
  }
  return Some(answer as u32);
}

fn part2(v: &Vec<((i32, i32), (i32, i32))>) -> Option<u64> {
//  let coord_max:i32 = 20;
  let coord_max:i32 = 4000000;
  let mut segments: Vec<Vec<(i32, i32)>> = vec![Vec::new();(coord_max + 1) as usize];
  for row in v.iter() {
    let sensor_location = row.0;
    let beacon_location = row.1;
    let distance = (sensor_location.0 - beacon_location.0).abs() + (sensor_location.1 - beacon_location.1).abs();
    
    // x + dx >= 0 -> dx >= -x
    // x + dx <= lim -> dx <= lim - x
    for dx in max(-distance, -sensor_location.0)..=min(distance, coord_max -sensor_location.0){
      let dy = (distance - dx.abs()).abs();
      segments[(sensor_location.0 + dx) as usize].push((sensor_location.1 - dy, -1));
      segments[(sensor_location.0 + dx) as usize].push((sensor_location.1 + dy + 1, 1));
  //    println!("debuginit {} {} {} {} {} {}", sensor_location.0, sensor_location.1, dx, dy, sensor_location.1 - dy, sensor_location.1 + dy + 1);
    }
  }
  for x in 0..segments.len(){
    segments[x].sort();
    let mut last_y = -1;
    let mut active = 0;
    for segment in segments[x].iter(){
//      println!("debug {} {} {} {} {}", x, last_y, active, segment.0, segment.1);
      if active == 0 && last_y >= 0 && last_y < segment.0{
        return Some(((4000000 as u64) * (x as u64) + (last_y as u64)) as u64); 
      }
      active -= segment.1;
      last_y = max(last_y, segment.0);
    }
    if last_y <= coord_max {
        return Some(((4000000 as u64) * (x as u64) + (last_y as u64)) as u64); 
    }
  }


  return None;
}
  
pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string())
                       .map(|x| x.split(":")
                                 .map(|x| {
                                    if x.starts_with("Sensor") {
                                      x.strip_prefix("Sensor at ").unwrap()
                                    } else {
                                      x.strip_prefix(" closest beacon is at ").unwrap()
                                    }
                                 }).collect::<Vec<&str>>()
                                    .into_iter()
                                    .map(|x| x.split(",").map(|x| {
                                        if x.trim().starts_with("x=") {
                                          x.trim().strip_prefix("x=").unwrap()
                                        } else {
                                          x.trim().strip_prefix("y=").unwrap()
                                        }
                                      })
                                      .map(|x| x.parse::<i32>().unwrap())
                                      .collect::<Vec<i32>>()
                                     )
                                      .map(|x| (x[0], x[1]))
                                   .collect::<Vec<(i32, i32)>>())
                                .map(|x| (x[0], x[1]))
                        .collect::<Vec<((i32, i32), (i32, i32))>>();

              
  println!("Day 15 Part 1: {}",part1(&v).unwrap());
  println!("Day 15 Part 2: {}",part2(&v).unwrap());
}
