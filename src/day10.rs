use std::io;

struct State{
  x: i32,
}

impl State{
  pub fn new() -> Self {
    return Self{
      x: 1
    };
  }
}

struct Computer{
 state : State,
 program: Vec<String>,
 index: usize,
 cycle: u32
}

impl Computer {
  pub fn new() -> Self {
    return Computer {
      state: State::new(),
      program: Vec::new(),
      index: 0,
      cycle: 1
    };
  }
  
  pub fn new_with_program(program: Vec<String>) -> Self {
    return Self {
      state: State::new(),
      program,
      index: 0,
      cycle: 1
    };
  }

  pub fn step(&mut self, steps: usize) {
    for _ in 0..steps{
      if self.index >= self.program.len() {
        panic!("Index is outside of program")
      }
      let splited_command = self.program[self.index].split(" ").collect::<Vec<&str>>();
      match splited_command[0]{
        "addx" => {self.state.x += splited_command[1].parse::<i32>().unwrap(); self.cycle += 2;},
        "noop" => self.cycle += 1,
        _ => panic!("Unknown command {}", self.program[self.index])
      }
      self.index += 1;
    }
  }
}

fn part1(v: &Vec<String>) -> Option<i32> {
  let mut computer = Computer::new_with_program(v.clone());
  let targets = vec![20, 60, 100, 140, 180, 220];

  let mut answer = 0;

  let mut last_x = computer.state.x;
  for target in targets{
    while computer.cycle <= target {
      last_x = computer.state.x;
      computer.step(1);
    }
    answer += (target as i32) * last_x;
  }

  return Some(answer);
}

fn part2(v: &Vec<String>) -> Option<Vec<String>> {
  let mut computer = Computer::new_with_program(v.clone());

  let mut last_x = computer.state.x;
  let mut answer: Vec<String>  = vec![String::from("");6];
  for target in 1..=240{
    while computer.cycle <= target {
      last_x = computer.state.x;
      computer.step(1);
    }
    let row = (target - 1) / 40;
    let col = (target - 1) % 40;
    answer[row as usize].push(if last_x - 1 <= col as i32 && col as i32 <= last_x + 1 {
      '#'
    } else {
      '.'
    });
  }

  return Some(answer);
}

pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string())
                       .collect::<Vec<String>>();
  println!("Day 10 Part 1: {}",part1(&v).unwrap());
  println!("Day 10 Part 2: \n{}",part2(&v).unwrap().join("\n"));
}
