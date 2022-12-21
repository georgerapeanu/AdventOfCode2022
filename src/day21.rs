use std::io;
use std::collections::HashMap;

trait Monkey{
  fn get_name(&self) -> String;
  fn compute(&self, values: &mut HashMap<String, i64>, monkeys: &HashMap<String, & dyn Monkey>) -> i64;
  fn compute_part2(&self, values: &mut HashMap<String, (f64, f64)>, monkeys: &HashMap<String, & dyn Monkey>) -> (f64, f64);
  fn get_dependencies(&self) -> Option<Vec<&String>>;
}

struct ConstantMonkey {
  name: String,
  value: i64
}

struct ExpressionMonkey {
  name: String,
  first_monkey_name: String,
  second_monkey_name: String,
  operation: char
}

impl Monkey for ConstantMonkey { 
  fn get_name(&self) -> String {
    return self.name.clone(); 
  }
  fn compute(&self, values: &mut HashMap<String, i64>, _monkeys: &HashMap<String, & dyn Monkey>) -> i64{
    if !values.contains_key(&self.name) {
      values.insert(self.name.clone(), self.value);
    }
    return *values.get(&self.name).unwrap();
  }
  fn compute_part2(&self, values: &mut HashMap<String, (f64, f64)>, monkeys: &HashMap<String, & dyn Monkey>) -> (f64, f64){
    if !values.contains_key(&self.name) {
      if self.name == "humn" {
        values.insert(self.name.clone(), (1 as f64, 0 as f64));
      } else {
        values.insert(self.name.clone(), (0 as f64, self.value as f64));
      }
    }
    return *values.get(&self.name).unwrap();
  }
  fn get_dependencies(&self) -> Option<Vec<&String>>{
    None
  }
}

impl Monkey for ExpressionMonkey {
  fn get_name(&self) -> String {
    return self.name.clone(); 
  }
  fn compute(&self, values: &mut HashMap<String, i64>, monkeys: &HashMap<String, & dyn Monkey>) -> i64{
    if !values.contains_key(&self.name) {
      let first_value = monkeys.get(&self.first_monkey_name).unwrap().compute(values, monkeys);
      let second_value = monkeys.get(&self.second_monkey_name).unwrap().compute(values, monkeys);
      let value = match self.operation{
        '+' => first_value + second_value,
        '-' => first_value - second_value,
        '*' => first_value * second_value,
        '/' => first_value / second_value,
        _ => panic!("Unsupported operation {}", self.operation)
      };
      values.insert(self.name.clone(), value);
    }
    return *values.get(&self.name).unwrap();
  }
  fn compute_part2(&self, values: &mut HashMap<String, (f64, f64)>, monkeys: &HashMap<String, & dyn Monkey>) -> (f64, f64){
    if !values.contains_key(&self.name) {
      if self.name == "humn"{
        values.insert(self.name.clone(), (1 as f64, 0 as f64));
      } else {
        let first_value = monkeys.get(&self.first_monkey_name).unwrap().compute_part2(values, monkeys);
        let second_value = monkeys.get(&self.second_monkey_name).unwrap().compute_part2(values, monkeys);
        let value = match self.operation{
          '+' => (first_value.0 + second_value.0, first_value.1 + second_value.1),
          '-' => (first_value.0 - second_value.0, first_value.1 - second_value.1),
          '*' => {
            if first_value.0 != 0 as f64 && second_value.0 != 0 as f64 {
              panic!("Nonlinear funtion possible through multiplication");
            }
            (first_value.0 * second_value.1 + first_value.1 * second_value.0, first_value.1 * second_value.1)
          }
          '/' => {
            if second_value.0 != 0 as f64 {
              panic!("Nonlinear function possible through division");
            }
            (first_value.0 / second_value.1, first_value.1 / second_value.1)
          }
          _ => panic!("Unsupported operation {}", self.operation)
        };
      values.insert(self.name.clone(), value);
      }
    }
    return *values.get(&self.name).unwrap();
  }
  fn get_dependencies(&self) -> Option<Vec<&String>>{
    Some(vec![&self.first_monkey_name, &self.second_monkey_name])
  }
}

fn get_monkey_from_string(s: String) -> Box<dyn Monkey> {
  let tokens = s.split(":").collect::<Vec<&str>>();
  let name = tokens[0].trim().to_string();
  match tokens[1].trim().parse::<i64>() {
    Err(_) => {
      let expression_tokens = tokens[1].trim().split(" ").filter(|x| x.len() > 0).map(|x| x.trim().to_string()).collect::<Vec<String>>();
      Box::new(ExpressionMonkey{
        name,
        first_monkey_name: expression_tokens[0].clone(),
        second_monkey_name: expression_tokens[2].clone(),
        operation: expression_tokens[1].chars().nth(0).unwrap()
      })
    },
    Ok(x) => {
      Box::new(ConstantMonkey{
        name,
        value: x
      })
    }
  }
}

fn part1(v: &Vec<Box<dyn Monkey>>) -> Option<i64> {
  let mut monkeys: HashMap<String, &dyn Monkey> = HashMap::new(); 
  let mut values: HashMap<String, i64> = HashMap::new(); 

  for i in 0..v.len(){
    monkeys.insert(v[i].as_ref().get_name(), v[i].as_ref());
  }
  let root_monkey = monkeys.get(&String::from("root")).unwrap();
  return Some(root_monkey.compute(&mut values, &monkeys));
}

fn part2(v: &Vec<Box<dyn Monkey>>) -> Option<i64> {
  let mut monkeys: HashMap<String, &dyn Monkey> = HashMap::new(); 
  let mut values: HashMap<String, (f64, f64)> = HashMap::new(); 

  for i in 0..v.len(){
    monkeys.insert(v[i].as_ref().get_name(), v[i].as_ref());
  }
  let root_monkey = monkeys.get(&String::from("root")).unwrap();
  let dependendecies = root_monkey.get_dependencies().unwrap();
  let first_eq = monkeys.get(dependendecies[0]).unwrap().compute_part2(&mut values, &monkeys);
  let second_eq = monkeys.get(dependendecies[1]).unwrap().compute_part2(&mut values, &monkeys);
  let final_eq = (first_eq.0 - second_eq.0, first_eq.1 - second_eq.1);
  return Some((-final_eq.1 / final_eq.0).round() as i64);
}
  
pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string())
                       .filter(|x| x.len() > 0)
                       .map(|x| get_monkey_from_string(x))
                       .collect::<Vec<Box<dyn Monkey>>>();
  println!("Day 21 Part 1: {}",part1(&v).unwrap());
  println!("Day 21 Part 2: {}",part2(&v).unwrap());
}
