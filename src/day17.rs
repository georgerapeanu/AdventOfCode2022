use std::io;
use std::collections::HashMap;

const SHAPES :[&[(isize, isize)];5]= [
  &[(0, 0), (0, 1), (0, 2), (0, 3)],
  &[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
  &[(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
  &[(0, 0), (1, 0), (2, 0), (3, 0)],
  &[(0, 0), (0, 1), (1, 0), (1, 1)],
];

fn part1(movement: &String) -> Option<usize> {
  let mut table: Vec<Vec<bool>> = Vec::new();
  let cnt_shapes = 2022;
  let cnt_cols = 7;

  let movement_chars = movement.chars().collect::<Vec<char>>();

  let mut cnt_move = 0;
  for i in 0..cnt_shapes{
    let shape = SHAPES[i % SHAPES.len()];
    let mut x = (table.len() + 3) as isize;
    let mut y = (2) as isize;

    let check_if_collision_exists = |x: isize, y:isize| -> bool {
      for i in 0..shape.len(){
        let xx = x + shape[i].0;
        let yy = y + shape[i].1;
        if xx < 0 {
          return true;
        }
        if yy < 0 || yy >= cnt_cols{
          return true;
        }
        if xx as usize >= table.len() {
          continue;
        }
        if table[xx as usize][yy as usize] {
          return true;
        }
      }
      return false;
    };

    loop{
      y += if movement_chars[cnt_move] == '<' {-1} else {1};
      if check_if_collision_exists(x, y) {
        y -= if movement_chars[cnt_move] == '<' {-1} else {1};
      }
      cnt_move += 1;
      cnt_move %= movement.len();
      x -= 1;
      if check_if_collision_exists(x, y) {
        x += 1;
        break;
      }
    }
    for i in 0..shape.len() {
      let xx = x + shape[i].0;
      let yy = y + shape[i].1;

      while table.len() <= xx as usize {
        table.push(vec![false;cnt_cols as usize]);
      }
      table[xx as usize][yy as usize] = true;
    }
  }

  /*
  for i in 0..table.len(){
    for j in 0..table[i].len(){
      print!("{}", if table[i][j] {'#'} else {'.'});
    }
    println!("");
  }
  */
  return Some(table.len());
}

fn part2(movement: &String) -> Option<u64> {
  let mut table: Vec<Vec<bool>> = Vec::new();
  let cnt_cols = 7;

  let movement_chars = movement.chars().collect::<Vec<char>>();

  let mut cnt_move = 0;
  let mut i = 0;
  let mut state_map: HashMap<(Vec<usize>, usize), usize> = HashMap::new();
  state_map.insert((vec![0;cnt_cols as usize], 0), 0);
  let loop_start_index;
  let mut height_after_shape: Vec<usize> = Vec::new();
  height_after_shape.push(0);
  let mut cnt_shapes:u64 = 1000000000000;
  loop{
    let shape = SHAPES[i % SHAPES.len()];
    let mut x = (table.len() + 3) as isize;
    let mut y = (2) as isize;

    let check_if_collision_exists = |x: isize, y:isize| -> bool {
      for i in 0..shape.len(){
        let xx = x + shape[i].0;
        let yy = y + shape[i].1;
        if xx < 0 {
          return true;
        }
        if yy < 0 || yy >= cnt_cols{
          return true;
        }
        if xx as usize >= table.len() {
          continue;
        }
        if table[xx as usize][yy as usize] {
          return true;
        }
      }
      return false;
    };

    loop{
      y += if movement_chars[cnt_move] == '<' {-1} else {1};
      if check_if_collision_exists(x, y) {
        y -= if movement_chars[cnt_move] == '<' {-1} else {1};
      }
      cnt_move += 1;
      cnt_move %= movement.len();
      x -= 1;
      if check_if_collision_exists(x, y) {
        x += 1;
        break;
      }
    }
    for i in 0..shape.len() {
      let xx = x + shape[i].0;
      let yy = y + shape[i].1;

      while table.len() <= xx as usize {
        table.push(vec![false;cnt_cols as usize]);
      }
      table[xx as usize][yy as usize] = true;
    }
    i += 1;
    let mut state: (Vec<usize>, usize) = (Vec::new(), cnt_move);

    for i in 0..cnt_cols{
      let mut space = 0;
      while table.len() - 1 >= space && table[table.len() - 1 - space][i as usize] == false{
        space += 1;
      }
      state.0.push(space);
    }
    height_after_shape.push(table.len());
    if state_map.contains_key(&state){
      loop_start_index = *state_map.get(&state).unwrap();
      break;
    }
    state_map.insert(state, i);
  }
  
  let mut answer: u64;

  if cnt_shapes < height_after_shape.len() as u64 {
    answer = height_after_shape[cnt_shapes as usize] as u64;
    return Some(answer);
  }

  cnt_shapes -= (height_after_shape.len() - 1) as u64;
  answer = *height_after_shape.last().unwrap() as u64;
  answer += ((height_after_shape.last().unwrap() - height_after_shape[loop_start_index]) as u64) * ((cnt_shapes / ((height_after_shape.len() - loop_start_index - 1) as u64)));
  cnt_shapes %= ((height_after_shape.len() - loop_start_index - 1)) as u64; 
  answer += (height_after_shape[loop_start_index + (cnt_shapes as usize)] - height_after_shape[loop_start_index]) as u64;
  /*for i in 0..table.len(){
    for j in 0..table[i].len(){
      print!("{}", if table[i][j] {'#'} else {'.'});
    }
    println!("");
  }*/
  
  return Some(answer);

}
  
pub fn run() {
  let stdin = io::stdin();
  let movement = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string()).fold(None, |answer, x| if answer.is_none(){Some(x)} else {answer}).unwrap();
  println!("Day 17 Part 1: {}",part1(&movement).unwrap());
  println!("Day 17 Part 2: {}",part2(&movement).unwrap());
}
