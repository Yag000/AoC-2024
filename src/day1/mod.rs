use std::collections::HashMap;

pub fn part1() -> String {
    let path = "input/day1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let lists: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let v: Vec<&str> = line.split("   ").collect();
            (v[0].parse().unwrap(), v[1].parse().unwrap())
        })
        .collect();

    let (mut list_a, mut list_b): (Vec<i32>, Vec<i32>) = lists.into_iter().unzip();
    list_a.sort();
    list_b.sort();

    let couple = list_a.into_iter().zip(list_b);

    couple
        .into_iter()
        .map(|(a, b)| ((a - b).abs()))
        .sum::<i32>()
        .to_string()
}

pub fn part2() -> String {
    let path = "input/day1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let lists: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let v: Vec<&str> = line.split("   ").collect();
            (v[0].parse().unwrap(), v[1].parse().unwrap())
        })
        .collect();

    let (list_a, list_b): (Vec<i32>, Vec<i32>) = lists.into_iter().unzip();

    let mut hash: HashMap<i32, i32> = HashMap::new();

    for i in list_a {
        let occ = list_b
            .clone()
            .into_iter()
            .filter(|v| *v == i)
            .fold(0, |acc, _| acc + 1);

        let _ = hash.insert(i, occ);
    }

    let couple = hash.keys().zip(hash.values());

    couple
        .into_iter()
        .map(|(a, b)| a * b)
        .sum::<i32>()
        .to_string()
}
