use std::io;
use std::iter::Peekable;
use std::str::Chars;
use std::cmp::{Ordering, min};
use std::fmt;

trait PacketNode:fmt::Debug{
  fn get_value(&self) -> Option<u32>;
  fn get_children(&self) -> Option<&Vec<Box<dyn PacketNode>>>;
  fn append_child(&mut self, child: Box<dyn PacketNode>);
}

#[derive(Debug)]
struct PacketNodeInner{
  children: Box<Vec<Box<dyn PacketNode>>>
}

#[derive(Debug)]
struct PacketNodeLeaf{
  value: u32
}

impl PacketNode for PacketNodeLeaf {
  fn get_value(&self) -> Option<u32> {
    Some(self.value)
  }

  fn get_children(&self) -> Option<&Vec<Box<dyn PacketNode>>> {
    None
  }

  fn append_child(&mut self, _: Box<dyn PacketNode>) {
    panic!("Cannot append child to leaf");
  }
}

impl PacketNode for PacketNodeInner{
  fn get_value(&self) -> Option<u32> {
    None
  }

  fn get_children(&self) -> Option<&Vec<Box<dyn PacketNode>>> {
    Some(&self.children)
  }

  fn append_child(&mut self, child: Box<dyn PacketNode>) {
    self.children.as_mut().push(child);
  }
}

impl PacketNodeInner{
  pub fn new() -> Self{
    return Self{
      children: Box::new(Vec::new())
    }
  }
}

impl PacketNodeLeaf{
  pub fn new(value: u32) -> Self{
    return  Self{
      value
    }
  }
}

fn build_packet_node_tree(s: &mut Peekable<Chars> ) -> Option<Box<dyn PacketNode>>{
  if s.peek() == None {
    return None
  }
  let mut current_token_option = s.peek();
  let mut current_token = *current_token_option.unwrap();
  if ('0'..='9').contains(&current_token) {
    let mut value = 0;
    loop{
      value = value * 10 + (current_token as u32) - ('0' as u32);
      s.next();
      current_token_option = s.peek();
      if current_token_option == None{
        break;
      }
      current_token = *current_token_option.unwrap();
      if current_token == ',' || current_token == ']'{
        break;
      }
    }
    return Some(Box::<PacketNodeLeaf>::new(PacketNodeLeaf::new(value)));
  }
  if current_token != '['{
    panic!("Invalid expression");
  }
  s.next();

  let mut answer = PacketNodeInner::new();
  let mut first_element = true;
  loop {
    current_token_option = s.peek();
    if current_token_option == None{
      panic!("Unexpected end to expression");
    }
    current_token = *current_token_option.unwrap();
    if current_token == ']'{
      s.next();
      break; 
    }
    if current_token == ',' {
      if first_element {
        panic!("Unexpected comma in expression");
      }
      s.next();
      current_token_option = s.peek();
      if current_token_option == None {
        panic!("Unexpected end to expression");
      }
    }
    answer.append_child(build_packet_node_tree(s).unwrap());
    first_element = false;
  }
  return Some(Box::new(answer));
}

fn compare_packets(a: &dyn PacketNode, b: &dyn PacketNode) -> Ordering{
  match a.get_value() {
    None => {
      match b.get_value() {
        None => {
          let first = a.get_children().unwrap();
          let second = b.get_children().unwrap();
          for i in 0..min(first.len(), second.len()) {
            let order = compare_packets(first[i].as_ref(), second[i].as_ref());
            if order != Ordering::Equal{
              return order;
            }
          }
          first.len().cmp(&second.len())
        },
        Some(_) => {
          let first = a.get_children().unwrap();
          if first.len() == 0 {
            Ordering::Less
          } else {
            let order = compare_packets(first[0].as_ref(), b);
            if order != Ordering::Equal {
              order
            } else if first.len() > 1 {
              Ordering::Greater
            } else {
              Ordering::Equal
            }
          }
        }
      }
    },
    Some(value) => {
      match b.get_value() {
        None => {
          let second = b.get_children().unwrap();
          if second.len() == 0 {
            Ordering::Greater
          } else {
            let order = compare_packets(a, second[0].as_ref());
            if order != Ordering::Equal {
              order
            } else if second.len() > 1 {
              Ordering::Less
            } else {
              Ordering::Equal
            }
          }
        },
        Some(value2) => {
          value.cmp(&value2)
        }
      }
    }
  }
}

fn part1(v: &Vec<String>) -> Option<u32> {
  let packet_pairs = v.split(|x| x.len() == 0)
                      .into_iter()
                      .map(|x| x.into_iter().map(|x| x.clone()).collect::<Vec<String>>())
                      .map(|x| (build_packet_node_tree(&mut x[0].chars().into_iter().peekable()).unwrap(), build_packet_node_tree(&mut x[1].chars().into_iter().peekable()).unwrap()))
                      .collect::<Vec<(Box<dyn PacketNode>, Box<dyn PacketNode>)>>();
  let mut answer = 0;
  for i in 0..packet_pairs.len() {
    if compare_packets(packet_pairs[i].0.as_ref(), packet_pairs[i].1.as_ref()) == Ordering::Less{
      answer += (i + 1) as u32;
    }
  }
  return Some(answer);
}

fn part2(v: &Vec<String>) -> Option<u32> {
  let mut packets = v.into_iter()
                     .filter(|x| x.len() > 0)
                     .map(|x| build_packet_node_tree(&mut x.chars().into_iter().peekable()).unwrap())
                     .collect::<Vec<Box<dyn PacketNode>>>();
  let first_divider = PacketNodeInner{
    children: Box::new(vec![Box::new(PacketNodeLeaf::new(2))])
  };
  
  let second_divider = PacketNodeInner{
    children: Box::new(vec![Box::new(PacketNodeLeaf::new(6))])
  };
  
  let first_divider_copy = PacketNodeInner{
    children: Box::new(vec![Box::new(PacketNodeLeaf::new(2))])
  };
  
  let second_divider_copy = PacketNodeInner{
    children: Box::new(vec![Box::new(PacketNodeLeaf::new(6))])
  };
  packets.push(Box::new(first_divider));
  packets.push(Box::new(second_divider));
  packets.sort_by(|a, b| compare_packets(a.as_ref(), b.as_ref()));
  let mut answer = 1;
  for i in 0..packets.len() {
    if compare_packets(packets[i].as_ref(), &first_divider_copy) == Ordering::Equal{
      answer *= (i + 1) as u32;
    }
    if compare_packets(packets[i].as_ref(), &second_divider_copy) == Ordering::Equal{
      answer *= (i + 1) as u32;
    }
  }
  return Some(answer);
}
  
pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string())
                           .collect::<Vec<String>>();

              
  println!("Day 13 Part 1: {}",part1(&v).unwrap());
  println!("Day 13 Part 2: {}",part2(&v).unwrap());
}
