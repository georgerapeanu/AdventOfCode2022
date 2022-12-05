use std::io;


fn part1(stacks: &Vec<Vec<char>>, moves: &Vec<(u32, u32, u32)>) -> String {
  let mut local_stacks = stacks.clone();

  for __move in moves{
    for _ in 0..__move.0{
      let moved_item = local_stacks[__move.1 as usize].pop().unwrap();
      local_stacks[__move.2 as usize].push(moved_item);
    }
  }

  let mut answer = String::from("");
 
  for i in 1..local_stacks.len(){
    answer.push(*local_stacks[i].last().unwrap());
  }


  return answer;
}

fn part2(stacks: &Vec<Vec<char>>, moves: &Vec<(u32, u32, u32)>) -> String {
  let mut local_stacks = stacks.clone();

  for __move in moves{
    let mut tmp_stack:Vec<char> = Vec::new();
    for _ in 0..__move.0{
      let moved_item = local_stacks[__move.1 as usize].pop().unwrap();
      tmp_stack.push(moved_item);
    }
    for _ in 0..__move.0{
      let moved_item = tmp_stack.pop().unwrap();
      local_stacks[__move.2 as usize].push(moved_item);
    }
  }

  let mut answer = String::from("");
 
  for i in 1..local_stacks.len(){
    answer.push(*local_stacks[i].last().unwrap());
  }


  return answer;
}

pub fn run() {
  let stdin = io::stdin();
  let mut v = stdin.lines().map(|x| x.unwrap().to_string())
                        .collect::<Vec<String>>();
  let mut initial_conf:Vec<String> = Vec::new();
  while *v.first().unwrap() != String::from("") {
    initial_conf.push(v.first().unwrap().clone());
    v.remove(0);
  }
  v.remove(0);
  let cnt_stacks:u32 = initial_conf.last().unwrap().split(" ").into_iter().filter(|x| x.trim().len() > 0).map(|x| x.to_string().parse::<u32>().unwrap()).max().unwrap();
  let mut stacks:Vec<Vec<char> > = vec![vec![]; (cnt_stacks + 1) as usize];
  initial_conf.pop();
  initial_conf.reverse();
  for line in &initial_conf{
    let mut current_stack = 1;
    for i in (1..line.len()).step_by(4){
      let current_elem = *line.as_bytes().get(i).unwrap() as char;
      if current_elem != ' '{
        stacks[current_stack].push(current_elem);
      }
      current_stack += 1;
    }
  }

  let moves = v.into_iter()
               .map(|x| x.split(" ").into_iter()
                         .map(|x| x.to_string())
                         .collect::<Vec<String>>())
               .map(|x| (x[1].parse::<u32>().unwrap(), x[3].parse::<u32>().unwrap(), x[5].parse::<u32>().unwrap()))
               .collect::<Vec<(u32, u32, u32)>>();
  println!("Day 5 Part 1: {}",part1(&stacks, &moves));
  println!("Day 5 Part 2: {}",part2(&stacks, &moves));
}
