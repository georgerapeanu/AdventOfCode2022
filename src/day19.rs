use std::io;
use regex::Regex;
use std::cmp::{min, max};
use std::collections::{HashSet, HashMap};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Blueprint{
  id: u32,
  costs: Vec<Vec<u32>>
}

impl Blueprint{
  
  pub fn get_index_from_type(string: &str) -> usize{
    match string {
      "ore" => 0,
      "clay" => 1,
      "obsidian" => 2,
      "geode" => 3,
      _ => panic!("Unexpected robot type")
    }
  } 

  pub fn from_string(s: String) -> Self{
    let mut answer = Blueprint{
      id:0,
      costs: vec![vec![0;4];4]
    };
    let sentences = s.split(|x| x == '.').filter(|x| x.trim().len() > 0).collect::<Vec<&str>>();
    answer.id = sentences[0].strip_prefix("Blueprint ").unwrap().parse::<u32>().unwrap();
    let re = Regex::new(r"Each (.+) robot costs (.*)$").unwrap();
    for i in 1..sentences.len(){
      let captures = re.captures(sentences[i]).unwrap();
      let type_id = Blueprint::get_index_from_type(captures[1].to_string().trim());
      captures[2].to_string().split("and").for_each(|x| {
        let tmp = x.trim().split(" ").map(|x| x.trim()).collect::<Vec<&str>>();
        answer.costs[type_id][Blueprint::get_index_from_type(tmp[1])] += tmp[0].parse::<u32>().unwrap();
      });
      
    }
    return answer;
  }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State<'a>{
  blueprint: &'a Blueprint,
  robot_count: Vec<u32>,
  resource_count: Vec<u32>
}

impl<'a> State<'a>{
  pub fn get_quality_level(&self) -> u64{
    return (self.blueprint.id as u64) * (self.resource_count[3] as u64);
  }
  pub fn get_state_building(&self, build_id: usize) -> Result<Self, String>{
    if build_id >= self.blueprint.costs.len(){
      return Err(format!("Robot id {} does not exist", build_id).to_string());
    }
    for i in 0..self.blueprint.costs[build_id].len(){
      if self.blueprint.costs[build_id][i] > self.resource_count[i] {
        return Err(format!("Not enough resources with type id {} to build robot with id {}", i, build_id).to_string());
      }
    }
    let mut answer = self.clone();
    answer.advance(1);
    for i in 0..answer.blueprint.costs[build_id].len(){
      answer.resource_count[i] -= answer.blueprint.costs[build_id][i];
    }
    answer.robot_count[build_id] += 1;
    return Ok(answer);
  }

  pub fn advance(&mut self, steps: u32){
    for i in 0..self.resource_count.len(){
      self.resource_count[i] += self.robot_count[i] * steps;
    }
  }

  pub fn new(blueprint: &'a Blueprint) -> Self{
    return Self{
      blueprint,
      robot_count: vec![1,0,0,0],
      resource_count: vec![0,0,0,0]
    }
  }
}

fn run_blueprint_bfs(blueprint: &Blueprint, time_limit: u32) -> u64{
  let mut current_level = vec!(State::new(blueprint));
  let mut answer = 0;

  let mut visited: HashSet<State> = HashSet::new();
  visited.insert(current_level[0].clone());

  let mut maximum_cost = vec![0;4];
  
  for i in 0..blueprint.costs.len(){
    for j in 0..blueprint.costs[i].len(){
      maximum_cost[j] = max(maximum_cost[j], blueprint.costs[i][j]);
    }
  }

  for i in 0..=time_limit{
    println!("Processing states with distance {} with {} states", i, current_level.len());
    let mut next_level = vec![];
    for mut current in current_level{
      answer = max(answer, current.get_quality_level());
      if i == time_limit{
        continue;
      }
      for i in 0..current.robot_count.len() {
        let tmp = current.get_state_building(i);
        match tmp{
          Ok(new_state) => {
            next_level.push(new_state);
          }
          Err(_) => {}
        }
      }
      current.advance(1);
      next_level.push(current);
    }
    next_level.sort();
    next_level.dedup();
    next_level = next_level.into_iter()
                           .filter(|x| {
                             for i in 0..x.robot_count.len() {
                              if i != 3 && x.robot_count[i] > maximum_cost[i] {
                                return false;
                              }
                             }
                             return true;
                           })
                           .filter(|x| !visited.contains(x))
                           .collect();
    for i in 0..next_level.len(){
      visited.insert(next_level[i].clone());
    }
    current_level = next_level;
  }
  return answer;
}



fn get_theoretical_maximum(state: &State, time: u32, time_limit: u32) -> u32 {
  return state.resource_count[Blueprint::get_index_from_type("geode")] + (state.robot_count[Blueprint::get_index_from_type("geode")] * 2 + time_limit - time - 1) * (time_limit - time) / 2;
}

fn dfs<'a, 'b>(state: &'a mut State<'b>, time: u32, best_time: &'a mut HashMap<State<'b>, u32>, time_limit:u32, most_geodes: &'a mut u32, maximum_cost: &'a Vec<u32>) {
  if time >= time_limit{
    *most_geodes = max(*most_geodes, state.resource_count[Blueprint::get_index_from_type("geode")]);
    return ;
  }
  if best_time.contains_key(&state) && *best_time.get(&state).unwrap() <= time {
    return;
  }
  best_time.insert(state.clone(), time);
  if get_theoretical_maximum(&state, time, time_limit) <= *most_geodes {
    return ;
  }
  for i in (0..state.blueprint.costs.len()).rev(){
    match state.get_state_building(i){
      Ok(mut next_state) => {
        let mut to_visit = true;
        for i in 0..maximum_cost.len() {
          if i != Blueprint::get_index_from_type("geode") && next_state.robot_count[i] > maximum_cost[i] {
            to_visit = false;
            break; 
          }
        }
        if !to_visit{
          continue;
        }
        dfs(&mut next_state, time + 1, best_time, time_limit, most_geodes, maximum_cost);
      },
      Err(_) => ()
    }
  }
  state.advance(1);
  dfs(state, time + 1, best_time, time_limit, most_geodes, maximum_cost);
}
fn run_blueprint_dfs(blueprint: &Blueprint, time_limit: u32) -> u64{
  let mut answer = 0;
  let mut best_time: HashMap<State, u32> = HashMap::new();
  
  let mut maximum_cost = vec![0;4];
  
  for i in 0..blueprint.costs.len(){
    for j in 0..blueprint.costs[i].len(){
      maximum_cost[j] = max(maximum_cost[j], blueprint.costs[i][j]);
    }
  }
  dfs(&mut State::new(blueprint), 0, &mut best_time, time_limit, &mut answer, &maximum_cost);
  return (answer as u64) * (blueprint.id as u64);
}

fn part1(v: &Vec<Blueprint>) -> Option<u64> {
  Some(v.iter().fold(0, |acc, x| acc + run_blueprint_dfs(x, 24)))
}

fn part2(v: &Vec<Blueprint>) -> Option<u64> {
  let mut answer = 1;
  for i in 0..min(3, v.len()){
    answer *= run_blueprint_dfs(&v[i], 32) / (v[i].id as u64);
  }
  return Some(answer);
}
  
pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string())
                       .filter(|x| x.len() > 0)
                       .collect::<Vec<String>>();
  let blueprints = v.into_iter().fold("".to_string(), |acc, x| acc + x.trim())
                    .split(|x| x == '.' || x == ':')
                    .filter(|x| x.trim().len() > 0)
                    .collect::<Vec<&str>>()
                    .chunks(5).map(|x| x.iter().fold(String::from(""), |acc, x| acc + x.trim() + "."))
                    .map(|x| Blueprint::from_string(x))
                    .collect::<Vec<Blueprint>>();
  println!("Day 19 Part 1: {}",part1(&blueprints).unwrap());
  println!("Day 19 Part 2: {}",part2(&blueprints).unwrap());
}
