use std::collections::HashMap;

pub fn part1() -> String {
    let path = "input/day1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let (mut list_a, mut list_b): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let v: Vec<&str> = line.split("   ").collect();
            (v[0].parse::<i32>().unwrap(), v[1].parse::<i32>().unwrap())
        })
        .unzip();

    list_a.sort();
    list_b.sort();

    list_a
        .into_iter()
        .zip(list_b)
        .map(|(a, b)| ((a - b).abs()))
        .sum::<i32>()
        .to_string()
}

pub fn part2() -> String {
    let path = "input/day1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let (list_a, list_b): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let v: Vec<&str> = line.split("   ").collect();
            (v[0].parse::<i32>().unwrap(), v[1].parse::<i32>().unwrap())
        })
        .unzip();

    let mut hash: HashMap<i32, i32> = HashMap::new();

    for i in list_a {
        let occ = list_b.iter().filter(|v| **v == i).count();

        hash.insert(i, occ as i32);
    }

    hash.drain().map(|(a, b)| a * b).sum::<i32>().to_string()
}
