use std::io;

fn to_snafu(mut number: u64) -> String {
  let mut digits: Vec<i64> = Vec::new();

  while number > 0 {
    digits.push((number % 5) as i64);
    number /= 5;
  }

  let mut i = 0;
  while i < digits.len() {
    if digits[i] >= 3 {
      digits[i] -= 5;
      if i + 1 >= digits.len() {
        digits.push(0);
      }
      digits[i + 1] += 1;
    }
    i += 1;
  }

  return digits.into_iter().map(|x| match x {2 => "2", 1 => "1", 0 => "0", -1 => "-", -2 => "=", _ => panic!("Illegal digit")}).fold(String::from(""), |x, y| {y.to_string() + x.as_str()});
}

fn from_snafu(number: String) -> u64 {
  let mut current_power = 1;
  let mut answer = 0;
  for _ in 0..number.len() {
    current_power *= 5; 
  }

  for digit in number.chars().into_iter() {
    current_power /= 5;
    answer = ((answer as i64) + current_power * match digit {
      '2' => 2,
      '1' => 1,
      '0' => 0,
      '-' => -1,
      '=' => -2,
      _ => panic!("Illegal character {}", digit)
    }) as u64;
  }

  return answer;
}

fn part1(v: &Vec<String>) -> Option<String> {
  Some(to_snafu(v.into_iter().map(|x| from_snafu(x.clone())).reduce(|x, y| x + y).unwrap()))
}

fn part2(_: &Vec<String>) -> Option<String> {
  Some(String::from("Happy Christmas!"))
}
  
pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string())
                       .filter(|x| x.len() > 0)
                       .collect::<Vec<String>>();
  println!("Day 24 Part 1: {}",part1(&v).unwrap());
  println!("Day 24 Part 2: {}",part2(&v).unwrap());
}
