use std::io;

struct Monkey{
  id: u32,
  items: Vec<u32>,
  transform_worry : Box<dyn Fn(u32) -> u64>,
  get_next_monkey : Box<dyn Fn(u32) -> u32>,
  cnt_inspected: u32
}

impl Monkey{
  pub fn new(monkey_description: &Vec<String>) -> Self{
    let remove_prefixes = vec!["Monkey ", "Starting items: ", "Operation: new = ", "Test: divisible by ", "If true: throw to monkey ", "If false: throw to monkey "];
    if monkey_description.len() != 6 {
      panic!("Invalid input for monkey");
    }

    let id = monkey_description[0].strip_prefix(remove_prefixes[0]).unwrap().strip_suffix(":").unwrap().parse::<u32>().unwrap();
    let items = monkey_description[1].strip_prefix(remove_prefixes[1]).unwrap().split(",").map(|x| x.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let expression = monkey_description[2].strip_prefix(remove_prefixes[2]).unwrap().trim().to_string();
    let transform_worry = Box::new(move |x| -> u64 {
      let operator;
      if expression.contains("+") {
        operator = '+';
      } else if expression.contains("*") {
        operator = '*';
      } else {
        panic!("Invalid operation for monkey");
      }
      let terms = expression.split(operator).map(|x| x.trim()).collect::<Vec<&str>>();
      if terms.len() != 2 {
        panic!("Invalid operation for monkey");
      }
      return if terms[0].trim() == "old" {
        if terms[1].trim() == "old" {
          if operator == '+' {
            (x as u64) + (x as u64)
          } else {
            (x as u64) * (x as u64)
          }
        } else {
          if operator == '+' {
            (x as u64) + terms[1].parse::<u64>().unwrap()
          } else {
            (x as u64) * terms[1].parse::<u64>().unwrap()
          }
        }
      } else {
        if terms[1].trim() == "old" {
          if operator == '+' {
            terms[0].parse::<u64>().unwrap() + (x as u64)
          } else {
            terms[0].parse::<u64>().unwrap() * (x as u64)
          }
        } else {
          if operator == '+' {
            terms[0].parse::<u64>().unwrap() + terms[1].parse::<u64>().unwrap()
          } else {
            terms[0].parse::<u64>().unwrap() * terms[1].parse::<u64>().unwrap()
          }
        }
      };
    });
    let test_number = monkey_description[3].strip_prefix(remove_prefixes[3]).unwrap().parse::<u32>().unwrap(); 
    let true_monkey = monkey_description[4].strip_prefix(remove_prefixes[4]).unwrap().parse::<u32>().unwrap();
    let false_monkey = monkey_description[5].strip_prefix(remove_prefixes[5]).unwrap().parse::<u32>().unwrap();
    let get_next_monkey =  Box::new(move |x| if x % test_number == 0 {true_monkey} else {false_monkey});

    return Monkey{
      id,
      items,
      transform_worry,
      get_next_monkey,
      cnt_inspected: 0
    };
  }

  pub fn do_round_part1(monkeys: &mut Vec<Monkey>){
    for i in 0..monkeys.len() {
      if i as u32 != monkeys[i].id {
        panic!("Monkeys not in correct positions");
      }
    }

    for i in 0..monkeys.len() {
      while !monkeys[i].items.is_empty() {
        monkeys[i].cnt_inspected += 1;
        let mut item = monkeys[i].items.remove(0);
        item = (monkeys[i].transform_worry.as_ref()(item) / 3) as u32;
        let next_monkey = monkeys[i].get_next_monkey.as_ref()(item);
        monkeys[next_monkey as usize].items.push(item);
      }
    }
  }
  
  pub fn do_round_part2(monkeys: &mut Vec<Monkey>){
    let lcm_of_tests = 3 * 13 * 2 * 11 * 19 * 17 * 5 * 7 * 23; //hardcoded i know, ugly
    for i in 0..monkeys.len() {
      if i as u32 != monkeys[i].id {
        panic!("Monkeys not in correct positions");
      }
    }

    for i in 0..monkeys.len() {
      while !monkeys[i].items.is_empty() {
        monkeys[i].cnt_inspected += 1;
        let mut item = monkeys[i].items.remove(0);
        item = (monkeys[i].transform_worry.as_ref()(item) % lcm_of_tests) as u32;
        let next_monkey = monkeys[i].get_next_monkey.as_ref()(item);
        monkeys[next_monkey as usize].items.push(item);
      }
    }
  }
}

fn part1(v: &Vec<Vec<String>>) -> Option<u32> {
  let mut monkeys = v.into_iter().map(|x| Monkey::new(x)).collect::<Vec<Monkey>>();

  for _ in 0..20{
    Monkey::do_round_part1(&mut monkeys);
  }
  
  let mut monkey_scores = monkeys.into_iter()
                      .map(|x| x.cnt_inspected)
                      .collect::<Vec<u32>>();
  monkey_scores.sort();
  monkey_scores.reverse();
  return Some(monkey_scores[0] * monkey_scores[1]);
}

fn part2(v: &Vec<Vec<String>>) -> Option<u64> {
  let mut monkeys = v.into_iter().map(|x| Monkey::new(x)).collect::<Vec<Monkey>>();

  for _ in 0..10000{
    Monkey::do_round_part2(&mut monkeys);
  }
  
  let mut monkey_scores = monkeys.into_iter()
                      .map(|x| x.cnt_inspected)
                      .collect::<Vec<u32>>();
  monkey_scores.sort();
  monkey_scores.reverse();
  return Some((monkey_scores[0] as u64) * (monkey_scores[1] as u64));
}

pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string())
                       .collect::<Vec<String>>()
                       .split(|x| *x == String::from(""))
                       .map(|x| x.into_iter().map(|x| x.clone()).collect::<Vec<String>>())
                       .collect::<Vec<Vec<String>>>();
  println!("Day 11 Part 1: {}",part1(&v).unwrap());
  println!("Day 11 Part 2: {}",part2(&v).unwrap());
}
