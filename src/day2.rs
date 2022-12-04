use std::io;
use map_macro::map;
use std::collections::HashMap;


fn part1(v: &Vec<(String, String)>) -> u32 {
    let char_to_index:HashMap<String, u32> = map! {
        "A".to_string() => 0,
        "B".to_string() => 1,
        "C".to_string() => 2,
        "X".to_string() => 0,
        "Y".to_string() => 1,
        "Z".to_string() => 2
    };

    let index_outcome_table:HashMap<(u32, u32), u32> = map! {
        (0, 0) => 1,
        (0, 1) => 0,
        (0, 2) => 2,
        (1, 0) => 2,
        (1, 1) => 1,
        (1, 2) => 0,
        (2, 0) => 0,
        (2, 1) => 2,
        (2, 2) => 1
    };

    let index_to_score:HashMap<u32, u32> = map! {
        0 => 1,
        1 => 2,
        2 => 3
    };

    let outcome_to_score:HashMap<u32, u32> = map! {
        0 => 0,
        1 => 3,
        2 => 6,
    };
    
    let mut answer = 0;
    for elem in v.iter() {
        let outcome = index_outcome_table.get(&(*char_to_index.get(&elem.1).unwrap(), *char_to_index.get(&elem.0).unwrap())).unwrap();
        let score = outcome_to_score.get(&outcome).unwrap() + index_to_score.get(char_to_index.get(&elem.1).unwrap()).unwrap();
        answer += score;
    }
    return answer;
}

fn part2(v: &Vec<(String, String)>) -> u32 {
    let char_to_index:HashMap<String, u32> = map! {
        "A".to_string() => 0,
        "B".to_string() => 1,
        "C".to_string() => 2,
    };

    let index_outcome_table:HashMap<(u32, u32), u32> = map! {
        (0, 0) => 1,
        (0, 1) => 0,
        (0, 2) => 2,
        (1, 0) => 2,
        (1, 1) => 1,
        (1, 2) => 0,
        (2, 0) => 0,
        (2, 1) => 2,
        (2, 2) => 1
    };

    let index_to_score:HashMap<u32, u32> = map! {
        0 => 1,
        1 => 2,
        2 => 3
    };

    let outcome_to_score:HashMap<u32, u32> = map! {
        0 => 0,
        1 => 3,
        2 => 6,
    };

    let char_to_outcome:HashMap<String, u32> = map! {
        "X".to_string() => 0,
        "Y".to_string() => 1,
        "Z".to_string() => 2
    };
    
    let mut answer = 0;
    for elem in v.iter() {
        let outcome = *char_to_outcome.get(&elem.1).unwrap();
        let opponent_play = *char_to_index.get(&elem.0).unwrap();
        let mut play = 0;
        for (key, value) in &index_outcome_table{
            if key.1 == opponent_play && *value == outcome {
                play = key.0
            }
        }
        let score = outcome_to_score.get(&outcome).unwrap() + index_to_score.get(&play).unwrap();
        answer += score;
    }
    return answer;
}

pub fn run() {
    let stdin = io::stdin();
    let v = stdin.lines().map(|x| x.unwrap().trim().to_string()).map(|x| x.split(" ").map(|x| x.to_string()).collect::<Vec<String>>()).map(|x| (x[0].clone(), x[1].clone())).collect::<Vec<(String, String)>>();
    println!("Day 2 Part 1: {}",part1(&v));
    println!("Day 2 Part 2: {}",part2(&v));
}
