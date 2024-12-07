fn can_be_obtained(key: u64, t: &[&u64]) -> bool {
    match t.first() {
        None => key == 0,
        Some(v) => {
            can_be_obtained(key - *v, &t[1..])
                || (key % *v == 0 && can_be_obtained(key / *v, &t[1..]))
        }
    }
}

pub fn part1() -> String {
    let path = "input/day7.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let couples: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|line| {
            if let Some((key, values)) = line.split_once(':') {
                let key = key.parse::<u64>().unwrap();
                let values: Vec<u64> = values
                    .split_whitespace()
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();
                (key, values)
            } else {
                unreachable!();
            }
        })
        .collect();

    couples
        .iter()
        .filter_map(|(key, values)| {
            if can_be_obtained(*key, &values.iter().rev().collect::<Vec<&u64>>()) {
                Some(key)
            } else {
                None
            }
        })
        .sum::<u64>()
        .to_string()
}

fn can_be_obtained2(key: u64, t: &[&u64]) -> bool {
    match t.first() {
        None => key == 0,
        Some(v) => {
            let sk = key.to_string();
            let sv = v.to_string();

            can_be_obtained2(key - *v, &t[1..])
                || (key % *v == 0 && can_be_obtained2(key / *v, &t[1..]))
                || (sk.len() > sv.len()
                    && sk[sk.len() - sv.len()..] == sv
                    && can_be_obtained2(sk[0..sk.len() - sv.len()].parse().unwrap(), &t[1..]))
        }
    }
}

pub fn part2() -> String {
    let path = "input/day7.txt";
    let input = std::fs::read_to_string(path).unwrap();
    let couples: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|line| {
            if let Some((key, values)) = line.split_once(':') {
                let key = key.parse::<u64>().unwrap();
                let values: Vec<u64> = values
                    .split_whitespace()
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();
                (key, values)
            } else {
                unreachable!();
            }
        })
        .collect();

    couples
        .iter()
        .filter_map(|(key, values)| {
            if can_be_obtained2(*key, &values.iter().rev().collect::<Vec<&u64>>()) {
                Some(key)
            } else {
                None
            }
        })
        .sum::<u64>()
        .to_string()
}
