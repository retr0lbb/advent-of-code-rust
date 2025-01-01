use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

mod day2;
mod day3;
mod day4;
mod day5;

fn read_txt() -> io::Result<()> {
    let path = "src/day1.txt";

    let file = fs::File::open(path)?;

    let reader = io::BufReader::new(file);

    let mut right: Vec<i32> = Vec::new();
    let mut left: Vec<i32> = Vec::new();
    let mut sum = 0;

    for line in reader.lines() {
        let line = line?; // Trata o resultado de cada linha

        let numbers: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

        left.push(numbers[0].parse::<i32>().unwrap());
        right.push(numbers[1].parse::<i32>().unwrap());

        println!("left number: {}, right number: {}", numbers[0], numbers[1]);
    }

    right.sort();
    left.sort();

    if right.len() == left.len() {
        for i in 0..left.len() {
            let mut distance = left[i] - right[i];
            if distance < 0 {
                distance *= -1;
            }

            sum += distance;
        }
    }

    let mut number_hashmap: HashMap<i32, i32> = HashMap::new();
    let mut total_sum_of_numbers = 0;
    for i in 0..left.len() {
        number_hashmap.insert(left[i], 0);

        for j in 0..right.len() {
            if right[j] == left[i] {
                let new_value = number_hashmap.get(&left[i]).unwrap() + 1;
                number_hashmap.insert(left[i], new_value);
            }
        }

        let row_sum = left[i] * number_hashmap.get(&left[i]).unwrap();

        total_sum_of_numbers += row_sum;

        println!("adicionando {} a {}", row_sum, total_sum_of_numbers);
    }

    println!("{}", total_sum_of_numbers);

    Ok(())
}

fn main() {
    day5::main::main();
}
