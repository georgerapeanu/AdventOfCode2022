use std::io;
use std::cmp::max;
use num::Integer;
use std::rc::Rc;
use std::collections::{HashMap, HashSet};

fn get_first_cell_in_direction(matrix: &Vec<String>, mut x: usize, mut y: usize, dx: isize, dy: isize) -> (char, usize, usize) {
  loop{
    let mut xx = (x as isize) + dx;
    let mut yy = (y as isize) + dy;
    if xx < 0 {
      xx = matrix.len() as isize - 1;
    }
    if xx >= matrix.len() as isize{
      xx = 0;
    }

    if yy < 0 {
      yy = matrix[0].len() as isize - 1;
    }
    if yy >= matrix[0].len() as isize {
      yy = 0;
    }

    x = xx as usize;
    y = yy as usize;

    let current = matrix[x].chars().nth(y).unwrap();
    if current == ' ' {
      continue;
    }
    return (current, x, y);
  }
}

fn part1(v: &Vec<String>, instructions: &Vec<(usize, char)>) -> Option<usize> {
  let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
  let mut dir = 1;
  let mut x = 0;
  let mut y = get_first_cell_in_direction(v, x, 0, dirs[dir].0, dirs[dir].1).2;

  instructions.iter().for_each(|elem| {
    match elem.1 {
      'L' => {
        dir = (((dir as isize - elem.0 as isize) % (dirs.len() as isize) + (dirs.len() as isize)) % (dirs.len() as isize)) as usize;
      },
      'R' => {
        dir = (dir + elem.0) % dirs.len();
      },
      'F' => {
        for _ in 0..elem.0{
          let tmp = get_first_cell_in_direction(v, x, y, dirs[dir].0, dirs[dir].1);
          if tmp.0 == '#'{
            break;
          }
          x = tmp.1;
          y = tmp.2;
        }
      },
      _ => panic!("Illegal operation")
    }
  });

  return Some((x + 1) * 1000 + (y + 1) * 4 + ((dir + dirs.len() - 1) % dirs.len()))
}

fn gcd<T: Integer> (a: T, b: T) -> T {
  if b.is_zero() {
    return a;
  }
  let c = a.mod_floor(&b);
  return gcd(b, c);
}


trait TransformerFunction {
  fn apply(&self, cell: (usize, usize)) -> (usize, usize); 
  fn invert(&self, cell: (usize, usize)) -> (usize, usize); 
  fn get_inverse(&self) -> Rc<dyn TransformerFunction>; 
}

#[derive(Clone)]
struct TransformerFunctionLeaf{
  direct_function: Rc<dyn Fn((usize, usize)) -> (usize, usize)>, 
  inverse_function: Rc<dyn Fn((usize, usize)) -> (usize, usize)>
}

impl TransformerFunction for TransformerFunctionLeaf {
  fn apply(&self, cell: (usize, usize)) -> (usize, usize) {
    return self.direct_function.as_ref()(cell);
  }

  fn invert(&self, cell: (usize, usize)) -> (usize, usize) {
    return self.inverse_function.as_ref()(cell);
  }
  
  fn get_inverse(&self) -> Rc<dyn TransformerFunction> {
    return Rc::new(Self{
      direct_function: self.inverse_function.clone(),
      inverse_function: self.direct_function.clone()
    });
  }
}

struct TransformerFunctionComposite {
  first_function: Rc<dyn TransformerFunction>,
  second_function: Rc<dyn TransformerFunction>,
}

impl TransformerFunction for TransformerFunctionComposite {
  fn apply(&self, cell: (usize, usize)) -> (usize, usize) {
    return self.first_function.as_ref().apply(self.second_function.as_ref().apply(cell));
  }

  fn invert(&self, cell: (usize, usize)) -> (usize, usize) {
    return self.second_function.as_ref().invert(self.first_function.as_ref().invert(cell));
  }
  
  fn get_inverse(&self) -> Rc<dyn TransformerFunction> {
    return Rc::new(Self{
      first_function: self.second_function.get_inverse().clone(),
      second_function: self.first_function.get_inverse().clone()
    });
  }
}

fn build_transformer_function_composite_from_vector(functions: Vec<Rc<dyn TransformerFunction>>) -> Rc<dyn TransformerFunction> {
  let mut answer = functions[0].clone();
  for i in 1..functions.len() {
    answer = Rc::new(
      TransformerFunctionComposite{
        first_function: answer,
        second_function: functions[i].clone()
      }
    );
  }
  return answer;
}

/*
 * let a cube be unfolded like this
 *        0
 *       123
 *        4
 *        5
 * The goal is to store each such face in memory in the orientation given by the above unfolding.
 * let define the orientation as a 2d matrix, which maps from input relative coordinate to face coordinates. Then we can find adjacency matrixes for each face.
 * This "normal" orientation is (1,1).
 */
fn part2(v: &Vec<String>, instructions: &Vec<(usize, char)>) -> Option<usize> {
  let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
  let mut dirs_to_index: HashMap<(isize, isize), usize> = HashMap::new();

  for i in 0..dirs.len() {
    dirs_to_index.insert(dirs[i], i);
  }

  let side_length = gcd(v[0].len(), v.len());
  let mut faces = vec![vec![vec![' ';side_length];side_length];6];

  let mut rotations: Vec<Rc<dyn TransformerFunction>> = Vec::new();
  rotations.push(Rc::new(
    TransformerFunctionLeaf{
      direct_function: Rc::new(move |cell: (usize, usize)| cell),
      inverse_function: Rc::new(move |cell: (usize, usize)| cell)
    }
  ));
  
  rotations.push(Rc::new(
    TransformerFunctionLeaf{
      direct_function: Rc::new(move |cell: (usize, usize)| (side_length - 1 - cell.1, cell.0)),
      inverse_function: Rc::new(move |cell: (usize, usize)| (cell.1, side_length - 1 - cell.0))
    }
  ));

  rotations.push(Rc::new(
    TransformerFunctionLeaf{
      direct_function: Rc::new(move |cell: (usize, usize)| (side_length - 1 - cell.0, side_length - 1 - cell.1)),
      inverse_function: Rc::new(move |cell: (usize, usize)| (side_length - 1 - cell.0, side_length - 1 - cell.1)),
    }
  ));
  
  rotations.push(rotations[1].get_inverse());

  let adjacency = vec![
    vec![(5, rotations[0].clone()), (3, rotations[3].clone()), (2, rotations[0].clone()), (1, rotations[1].clone())],
    vec![(0, rotations[3].clone()), (2, rotations[0].clone()), (4, rotations[1].clone()), (5, rotations[2].clone())], 
    vec![(0, rotations[0].clone()), (3, rotations[0].clone()), (4, rotations[0].clone()), (1, rotations[0].clone())],
    vec![(0, rotations[1].clone()), (5, rotations[2].clone()), (4, rotations[3].clone()), (2, rotations[0].clone())], 
    vec![(2, rotations[0].clone()), (3, rotations[1].clone()), (5, rotations[0].clone()), (1, rotations[3].clone())],
    vec![(4, rotations[0].clone()), (3, rotations[2].clone()), (0, rotations[0].clone()), (1, rotations[2].clone())],
  ];

  let mut q: Vec<(usize ,usize, usize, Rc<dyn TransformerFunction>)> = Vec::new();
  let mut visited: HashSet<(usize, usize)> = HashSet::new();
  let mut face_to_offset: HashMap<usize, (usize, usize)> = HashMap::new();
  for start_x in (0..v.len()).step_by(side_length){
    for start_y in (0..v[start_x].len()).step_by(side_length) {
      if v[start_x].as_bytes()[start_y] == ' ' as u8{
        continue;
      }
      if q.is_empty() {
        q.push((start_x / side_length, start_y / side_length, 0, rotations[0].clone()));
      }
    }
  }
  visited.insert((q[0].0, q[0].1));

  let mut i = 0;

  while i < q.len() {
    let start_x = q[i].0 * side_length;
    let start_y = q[i].1 * side_length;
    let face = q[i].2;
    let function = q[i].3.clone();
    face_to_offset.insert(face, (start_x, start_y));

    for x in 0..side_length {
      for y in 0..side_length{
        let cell = function.apply((x, y)); 
        faces[face][cell.0][cell.1] = v[start_x + x].as_bytes()[start_y + y] as char;
      }
    }
    
    for x in  start_x..start_x + side_length {
      for y in start_y..start_y + side_length {
        for k in 0..dirs.len() {
          let xx = x as isize + dirs[k].0;
          let yy = y as isize + dirs[k].1;
          if xx < 0 || yy < 0 || xx >= v.len() as isize || yy >= v[xx as usize].len() as isize {
            continue;
          }
          if v[xx as usize].as_bytes()[yy as usize] == ' ' as u8{
            continue;
          }
          let direction_vector_helper_point_0 = (1, 1);
          let direction_vector_helper_point_1 = ((1 + dirs[k].0) as usize, (1 + dirs[k].1) as usize);
          let result_vector_helper_0 = function.apply(direction_vector_helper_point_0);
          let result_vector_helper_1 = function.apply(direction_vector_helper_point_1);
          let localized_vector = (result_vector_helper_1.0 as isize - result_vector_helper_0.0 as isize, result_vector_helper_1.1 as isize - result_vector_helper_0.1 as isize);
          let localized_direction = *dirs_to_index.get(&localized_vector).unwrap();

          let neighbor = (xx as usize / side_length, yy as usize / side_length, adjacency[face][localized_direction].0, build_transformer_function_composite_from_vector(vec![function.clone(), adjacency[face][localized_direction].1.clone()]));
          if visited.contains(&(neighbor.0, neighbor.1)) {
            continue;
          }
          visited.insert((neighbor.0, neighbor.1));
          q.push(neighbor);
        }
      }
    }
    i += 1;
  }

  let mut current = (0, 0, 0, 1);  
 
  instructions.iter().for_each(|elem| {
    match elem.1 {
      'L' => {
        current.3 = if current.3 == 0 {dirs.len() - 1} else {current.3 - 1};
      },
      'R' => {
        current.3 = if current.3 == dirs.len() - 1 {0} else {current.3 + 1};
      },
      'F' => {
        for _ in 0..elem.0{
          let next_cell_isize = (current.1 as isize + dirs[current.3].0, current.2 as isize + dirs[current.3].1);
          let next: (usize, usize, usize, usize);
          next = if next_cell_isize.0 < 0 || next_cell_isize.0 >= side_length as isize || next_cell_isize.1 < 0 || next_cell_isize.1 >= side_length as isize {
            let next_face = adjacency[current.0][current.3].0;
            let transform_function = adjacency[current.0][current.3].1.clone();
            let direction_helper_cell_0 = (1, 1);
            let direction_helper_cell_1 = ((1 + dirs[current.3].0) as usize, (1 + dirs[current.3].1) as usize);
            let result_direction_helper_cell_0 = transform_function.as_ref().apply(direction_helper_cell_0);
            let result_direction_helper_cell_1 = transform_function.as_ref().apply(direction_helper_cell_1);
            let result_direction = (result_direction_helper_cell_1.0 as isize - result_direction_helper_cell_0.0 as isize,
                                    result_direction_helper_cell_1.1 as isize - result_direction_helper_cell_0.1 as isize);
            let result_direction_index = *dirs_to_index.get(&result_direction).unwrap();

            let mut next_cell = (0, 0);

            next_cell.0 = (next_cell_isize.0 + if next_cell_isize.0 < 0 {side_length as isize} else if next_cell_isize.0 >= side_length as isize {-(side_length as isize)} else {0}) as usize;
            next_cell.1 = (next_cell_isize.1 + if next_cell_isize.1 < 0 {side_length as isize} else if next_cell_isize.1 >= side_length as isize {-(side_length as isize)} else {0}) as usize;
            next_cell = transform_function.as_ref().apply(next_cell);

            (next_face, next_cell.0, next_cell.1, result_direction_index)
          } else {
            (current.0, next_cell_isize.0 as usize, next_cell_isize.1 as usize, current.3)
          };
          
          if faces[next.0][next.1][next.2] == '#'{
            break;
          }
          current = next;
        }
      },
      _ => panic!("Illegal operation")
    }
  });

  let offset = *face_to_offset.get(&current.0).unwrap();
  let answer = (offset.0 + current.1, offset.1 + current.2, current.3);
  return Some((answer.0 + 1) * 1000 + (answer.1 + 1) * 4 + ((answer.2 + dirs.len() - 1) % dirs.len()));
}
  
pub fn run() {
  let stdin = io::stdin();
  let mut v = stdin.lines().map(|x| x.unwrap().to_string().replace("\n", ""))
                       .filter(|x| x.len() > 0)
                       .collect::<Vec<String>>();
  let instructions_raw = v.pop().unwrap();
  let mut current_count = 0;
  let mut instructions: Vec<(usize, char)> = Vec::new();
  instructions_raw.chars().for_each(|x| {
    match x{
      'L' | 'R' => {
        instructions.push((current_count, 'F'));
        instructions.push((1, x));
        current_count = 0;
      }
      '0'..='9' => {
        current_count = current_count * 10 + (x as usize - '0' as usize);
      }
      _ => {
        panic!("Illegal character");
      }
    }
  });

  let max_len = v.iter().fold(0, |result, x| max(result, x.len()));
  for i in 0..v.len(){
    while v[i].len() < max_len {
      v[i].push(' ');
    }
  }
  instructions.push((current_count, 'F'));
  println!("Day 22 Part 1: {}",part1(&v, &instructions).unwrap());
  println!("Day 22 Part 2: {}",part2(&v, &instructions).unwrap());
}
