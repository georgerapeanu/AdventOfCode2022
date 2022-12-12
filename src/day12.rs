use std::io;

fn part1(v: &Vec<String>) -> Option<i32> {
  let dirs: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

  let mut start_x = 0;
  let mut start_y = 0;
  let mut end_x = 0;
  let mut end_y = 0;
  for i in 0..v.len() {
    for j in 0..v[i].as_bytes().len() {
      if v[i].as_bytes()[j] == 'S' as u8{
        start_x = i;
        start_y = j;
      }
      if v[i].as_bytes()[j] == 'E' as u8{
        end_x = i;
        end_y = j;
      }
    }
  }

  let mut dist = vec![vec![-1; v[0].len()];v.len()];

  let mut q: Vec<(usize, usize)> = vec![];

  dist[start_x][start_y] = 0;
  q.push((start_x, start_y));

  let mut i = 0;
  while i < q.len(){
    for k in 0..dirs.len(){
      let x = q[i].0;
      let y = q[i].1;
      let xx = (x as i32) + dirs[k].0;
      let yy = (y as i32) + dirs[k].1;
      if xx < 0 || xx as usize >= v.len() || yy < 0 || yy as usize >= v[x].len() || dist[xx as usize][yy as usize] != -1{
        continue;
      }
      let current_elevation = match v[x].as_bytes()[y] as char{
        'S' => 0,
        'E' => 'z' as u8 - 'a' as u8,
        _ => v[x].as_bytes()[y] as u8 - 'a' as u8
      };
      let next_elevation = match v[xx as usize].as_bytes()[yy as usize] as char{
        'S' => 0,
        'E' => 'z' as u8 - 'a' as u8,
        _ => v[xx as usize].as_bytes()[yy as usize] as u8 - 'a' as u8
      };

      if (current_elevation as i32) - (next_elevation as i32) < -1 {
        continue;
      }
      dist[xx as usize][yy as usize] = 1 + dist[x][y];
      q.push((xx as usize, yy as usize));
    }
    i += 1;
  }

  return Some(dist[end_x][end_y]);
}

fn part2(v: &Vec<String>) -> Option<i32> {
  let dirs: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

  let mut end_x = 0;
  let mut end_y = 0;
  for i in 0..v.len() {
    for j in 0..v[i].as_bytes().len() {
      if v[i].as_bytes()[j] == 'E' as u8{
        end_x = i;
        end_y = j;
      }
    }
  }

  let mut dist = vec![vec![-1; v[0].len()];v.len()];

  let mut q: Vec<(usize, usize)> = vec![];

  dist[end_x][end_y] = 0;
  q.push((end_x, end_y));

  let mut i = 0;
  while i < q.len(){
    for k in 0..dirs.len(){
      let x = q[i].0;
      let y = q[i].1;
      let xx = (x as i32) + dirs[k].0;
      let yy = (y as i32) + dirs[k].1;
      if xx < 0 || xx as usize >= v.len() || yy < 0 || yy as usize >= v[x].len() || dist[xx as usize][yy as usize] != -1{
        continue;
      }
      let current_elevation = match v[x].as_bytes()[y] as char{
        'S' => 0,
        'E' => 'z' as u8 - 'a' as u8,
        _ => v[x].as_bytes()[y] as u8 - 'a' as u8
      };
      let next_elevation = match v[xx as usize].as_bytes()[yy as usize] as char{
        'S' => 0,
        'E' => 'z' as u8 - 'a' as u8,
        _ => v[xx as usize].as_bytes()[yy as usize] as u8 - 'a' as u8
      };

      if (next_elevation as i32) - (current_elevation as i32) < -1 {
        continue;
      }
      dist[xx as usize][yy as usize] = 1 + dist[x][y];
      q.push((xx as usize, yy as usize));
    }
    i += 1;
  }

  let mut answer = -1;

  for i in 0..v.len(){
    for j in 0..v[i].len() {
      if dist[i][j] == -1 {
        continue;
      }
      if v[i].as_bytes()[j] as char == 'a' || v[i].as_bytes()[j] as char == 'S' {
        if answer == -1 || dist[i][j] < answer {
          answer = dist[i][j];
        }
      }
    }
  }

  return Some(answer);
}
  
pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string())
                       .collect::<Vec<String>>();
  println!("Day 12 Part 1: {}",part1(&v).unwrap());
  println!("Day 12 Part 2: {}",part2(&v).unwrap());
}
