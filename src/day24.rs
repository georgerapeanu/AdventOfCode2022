use std::io;
use num::Integer;

fn part1(v: &Vec<String>) -> Option<usize> {
  let dirs: Vec<(isize, isize)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1), (0, 0)];
  let inner_rows = v.len() - 2;
  let inner_cols = v[0].len() - 2;
  let mut states = vec![vec![vec![0;v[0].len()];v.len()];(inner_rows).lcm(&inner_cols)];
  let mut dist = vec![vec![vec![0;v[0].len()];v.len()];(inner_rows).lcm(&inner_cols)];
  for i in 0..inner_rows {
    for j in 0..inner_cols {
      for k in 0..states.len() {
        let x:usize;
        let y:usize;
        match v[i + 1].as_bytes()[j + 1] as char{
          '^' => {x = (i + k * (inner_rows - 1)) % inner_rows;y = j},
          'v' => {x = (i + k) % inner_rows;y = j},
          '>' => {x = i;y = (j + k) % inner_cols;},
          '<' => {x = i;y = (j + k * (inner_cols - 1)) % inner_cols;},
          _ => continue
        }
        states[k][x + 1][y + 1] = 1;
      }
    }
  }
  
  for i in 0..v.len() {
    for j in 0..v[i].len() {
      if i == 0 || i == v.len() - 1 || j == 0 || j == v[i].len() - 1 {
        if (i != 0 || j != 1) && (i != v.len() - 1 || j != v[i].len() - 2)  {
          for k in 0..states.len() {
            states[k][i][j] = 1;
          }
        }
      }
    }
  }

  let mut q: Vec<(usize, usize, usize)> = Vec::new();
  let mut i = 0;
  q.push((0, 0, 1));
  dist[0][0][1] = 1;

  while i < q.len() {
    let k = q[i].0;
    let x = q[i].1;
    let y = q[i].2;

    for dir in dirs.iter() {
      let kk = (k + 1) % states.len();
      let xx = x as isize + dir.0; 
      let yy = y as isize + dir.1;
      if xx < 0 || xx >= v.len() as isize || yy < 0 || yy >= v[x].len() as isize {
        continue;
      }
      
      if states[kk][xx as usize][yy as usize] == 1 || dist[kk][xx as usize][yy as usize] != 0{
        continue;
      }

      dist[kk][xx as usize][yy as usize] = 1 + dist[k][x][y];
      q.push((kk, xx as usize, yy as usize));
    }

    i += 1;
  }

  return dist.iter().map(|x| x[v.len() - 1][v[0].len() - 2]).filter(|x| *x > 0).map(|x| x - 1).min();
}

fn part2(v: &Vec<String>) -> Option<usize> {
  let dirs: Vec<(isize, isize)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1), (0, 0)];
  let inner_rows = v.len() - 2;
  let inner_cols = v[0].len() - 2;
  let mut states = vec![vec![vec![0;v[0].len()];v.len()];(inner_rows).lcm(&inner_cols)];
  let mut dist = vec![vec![vec![vec![0;v[0].len()];v.len()];(inner_rows).lcm(&inner_cols)];4];
  for i in 0..inner_rows {
    for j in 0..inner_cols {
      for k in 0..states.len() {
        let x:usize;
        let y:usize;
        match v[i + 1].as_bytes()[j + 1] as char{
          '^' => {x = (i + k * (inner_rows - 1)) % inner_rows;y = j},
          'v' => {x = (i + k) % inner_rows;y = j},
          '>' => {x = i;y = (j + k) % inner_cols;},
          '<' => {x = i;y = (j + k * (inner_cols - 1)) % inner_cols;},
          _ => continue
        }
        states[k][x + 1][y + 1] = 1;
      }
    }
  }
  
  for i in 0..v.len() {
    for j in 0..v[i].len() {
      if i == 0 || i == v.len() - 1 || j == 0 || j == v[i].len() - 1 {
        if (i != 0 || j != 1) && (i != v.len() - 1 || j != v[i].len() - 2)  {
          for k in 0..states.len() {
            states[k][i][j] = 1;
          }
        }
      }
    }
  }

  let mut q: Vec<(usize, usize, usize, usize)> = Vec::new();
  let mut i = 0;
  q.push((0, 0, 0, 1));
  dist[0][0][0][1] = 1;

  while i < q.len() {
    let state = q[i].0;
    let k = q[i].1;
    let x = q[i].2;
    let y = q[i].3;

    for dir in dirs.iter() {
      let kk = (k + 1) % states.len();
      let xx = x as isize + dir.0; 
      let yy = y as isize + dir.1;
      if xx < 0 || xx >= v.len() as isize || yy < 0 || yy >= v[x].len() as isize {
        continue;
      }
      
      let mut ss = state;
      if state % 2 == 0 {
        if xx == v.len() as isize - 1 && yy == v[0].len() as isize - 2 {
          ss += 1;
        }
      } else {
        if xx == 0 && yy == 1 {
          ss += 1;
        }
      }

      if ss >= dist.len() {
        continue;
      }
      
      if states[kk][xx as usize][yy as usize] == 1 || dist[ss][kk][xx as usize][yy as usize] != 0{
        continue;
      }

      dist[ss][kk][xx as usize][yy as usize] = 1 + dist[state][k][x][y];
      q.push((ss, kk, xx as usize, yy as usize));
    }

    i += 1;
  }

  return dist[3].iter().map(|x| x[v.len() - 1][v[0].len() - 2]).filter(|x| *x > 0).map(|x| x - 1).min();
}
  
pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string())
                       .filter(|x| x.len() > 0)
                       .collect::<Vec<String>>();
  println!("Day 24 Part 1: {}",part1(&v).unwrap());
  println!("Day 24 Part 2: {}",part2(&v).unwrap());
}
