use std::io;
use std::cmp::max;
use std::collections::HashMap;
fn part1(graph: &Vec<Vec<usize>>, flows: &Vec<u32>, to_norm: &HashMap<String, usize>) -> Option<u32> {
  let mut relevant_to_bits: HashMap<usize, usize> = HashMap::new();
  for i in 0..flows.len() {
    if flows[i] > 0 {
      relevant_to_bits.insert(i, relevant_to_bits.len());
    }
  }
  let cnt_relevant = relevant_to_bits.len();
  let total_time = 30;

  let mut dp: Vec<Vec<Vec<u32> > > = vec![vec![vec![0; graph.len()];1 << cnt_relevant];total_time + 1];
  let mut visited: Vec<Vec<Vec<bool> > > = vec![vec![vec![false; graph.len()];1 << cnt_relevant];total_time + 1];
  visited[0][0][to_norm["aa"]] = true;

  for i in 0..total_time {
    for j in 0..(1 << cnt_relevant) {
      for k in 0..graph.len() {
        if !visited[i][j][k] {
          continue;
        }
        for neighbor in graph[k].iter() {
          dp[i + 1][j][*neighbor] = max(dp[i + 1][j][*neighbor], dp[i][j][k]);
          visited[i + 1][j][*neighbor] = true;
        }
        if relevant_to_bits.contains_key(&k) {
          let bit = relevant_to_bits.get(&k).unwrap();
          if ((j >> bit) & 1) == 0 {
            dp[i + 1][j | (1 << bit)][k] = max(dp[i + 1][j | (1 << bit)][k], dp[i][j][k] + flows[k] * ((total_time - i - 1) as u32));
            visited[i + 1][j | (1 << bit)][k] = true;
          }
        }
      }
    }
  }

  let mut answer = None;
  for i in 0..=total_time{
    for j in 0..(1 << cnt_relevant){
      for k in 0..graph.len(){
        if visited[i][j][k]{
          if answer.is_none() || dp[i][j][k] > answer.unwrap() {
            answer = Some(dp[i][j][k]);
          }
        }
      }
    }
  }

  
  return answer;
}

fn part2(graph: &Vec<Vec<usize>>, flows: &Vec<u32>, to_norm: &HashMap<String, usize>) -> Option<u32> {
  let mut relevant_to_bits: HashMap<usize, usize> = HashMap::new();
  for i in 0..flows.len() {
    if flows[i] > 0 {
      relevant_to_bits.insert(i, relevant_to_bits.len());
    }
  }
  let cnt_relevant = relevant_to_bits.len();
  let total_time = 26;

  let mut dp: Vec<Vec<Vec<u32> > > = vec![vec![vec![0; graph.len()];1 << cnt_relevant];total_time + 1];
  let mut visited: Vec<Vec<Vec<bool> > > = vec![vec![vec![false; graph.len()];1 << cnt_relevant];total_time + 1];
  visited[0][0][to_norm["aa"]] = true;

  for i in 0..total_time {
    for j in 0..(1 << cnt_relevant) {
      for k in 0..graph.len() {
        if !visited[i][j][k] {
          continue;
        }
        for neighbor in graph[k].iter() {
          dp[i + 1][j][*neighbor] = max(dp[i + 1][j][*neighbor], dp[i][j][k]);
          visited[i + 1][j][*neighbor] = true;
        }
        if relevant_to_bits.contains_key(&k) {
          let bit = relevant_to_bits.get(&k).unwrap();
          if ((j >> bit) & 1) == 0 {
            dp[i + 1][j | (1 << bit)][k] = max(dp[i + 1][j | (1 << bit)][k], dp[i][j][k] + flows[k] * ((total_time - i - 1) as u32));
            visited[i + 1][j | (1 << bit)][k] = true;
          }
        }
      }
    }
  }

  let mut best_for_conf = vec![0;1 << cnt_relevant];

  for i in 0..=total_time {
    for j in 0..(1 << cnt_relevant) {
      for k in 0..graph.len() {
        if !visited[i][j][k] {
          continue;
        }
        best_for_conf[j] = max(best_for_conf[j], dp[i][j][k]);
      }
    }
  }

  let mut answer = best_for_conf.clone();

  for conf in 0..(1 << cnt_relevant) {
    let opposite_conf = ((1 << cnt_relevant) - 1) ^ conf;
    let mut conf2 = opposite_conf;
    while conf2 > 0 {
      answer[conf | conf2] = max(answer[conf | conf2], best_for_conf[conf] + best_for_conf[conf2]);
      conf2 = (conf2 - 1) & opposite_conf;
    }
  }
  
  return answer.into_iter().max();
}
  
pub fn run() {
  let stdin = io::stdin();
  let v = stdin.lines().map(|x| x.unwrap().to_string().trim().to_string())
                       .map(|x| x.to_lowercase())
                       .map(|x| {
                          let y = x.strip_prefix("valve ").unwrap();
                          let first_split = y.split_once(" ").unwrap();
                          let z = first_split.1.strip_prefix("has flow rate=").unwrap();
                          let second_split = z.split_once(";").unwrap();
                          let last_data; 
                          if second_split.1.starts_with(" tunnel leads to valve") {
                            last_data = second_split.1.strip_prefix(" tunnel leads to valve").unwrap();
                          } else {
                            last_data = second_split.1.strip_prefix(" tunnels lead to valves").unwrap();
                          }
                          return vec![first_split.0.to_string(), second_split.0.to_string(),last_data.to_string()];
                       })
                       .map(|x| {
                          let first = x[0].trim().to_string();
                          let second = x[1].trim().parse::<u32>().unwrap();
                          let mut third: Vec<String> = Vec::new();

                          x[2].split(",")
                              .into_iter()
                              .for_each(|x| third.push(x.trim().to_string()));
                          return (first, second, third);
                       })
                       .collect::<Vec<(String, u32, Vec<String>)>>();
                 
  let mut norm_map: HashMap<String, usize> = HashMap::new();
  let mut flows: Vec<u32> = Vec::new();
  let mut graph: Vec<Vec<usize>> = Vec::new();

  v.iter().for_each(|x| {
    if !norm_map.contains_key(&x.0) {
      norm_map.insert(x.0.clone(), norm_map.len());
    }

    for i in 0..x.2.len() {
      if !norm_map.contains_key(&x.2[i]){
        norm_map.insert(x.2[i].clone(), norm_map.len());
      }
    } 
    
    while graph.len() < norm_map.len() {
      graph.push(Vec::new());
    }

    while flows.len() < norm_map.len() {
      flows.push(0);
    }

    flows[*norm_map.get(&x.0).unwrap()] = x.1;

    for i in 0..x.2.len(){
      graph[*norm_map.get(&x.0).unwrap()].push(*norm_map.get(&x.2[i]).unwrap());
    }
  });

  println!("Day 16 Part 1: {}",part1(&graph, &flows, &norm_map).unwrap());
  println!("Day 16 Part 2: {}",part2(&graph, &flows, &norm_map).unwrap());
}
