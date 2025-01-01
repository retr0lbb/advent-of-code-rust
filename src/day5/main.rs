use std::{
    collections::{self, HashMap},
    fs,
};

fn create_rules_hash_map(lines: Vec<String>) -> collections::HashMap<i32, Vec<i32>> {
    let mut number_before_target: collections::HashMap<i32, Vec<i32>> = collections::HashMap::new();

    for line in lines {
        let rule_values_in_string: Vec<String> = line.split("|").map(|s| s.to_string()).collect();
        let rule_values: Result<[i32; 2], _> = rule_values_in_string
            .iter()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map(|v| [v[0], v[1]]);

        match rule_values {
            Ok(values) => {
                number_before_target
                    .entry(values[0])
                    .or_insert_with(Vec::new)
                    .push(values[1]);
            }
            Err(e) => {
                println!("Failed to parse values: {}", e);
            }
        }
    }

    return number_before_target;
}

fn extract_print_queue(queue: String) -> Vec<i32> {
    queue.split(",").map(|n| n.parse::<i32>().unwrap()).collect()
}

fn find_valid_queue(map: HashMap<i32, Vec<i32>>, queue: Vec<i32>) -> Option<i32> {
    for index1 in 0..queue.len() - 1 {
        if let Some(numbers_that_this_goes_before) = map.get(&queue[index1]) {
            // Verificando se a ordem de impressão está errada
            for number in numbers_that_this_goes_before {
                if queue[index1 + 1] == *number {
                    println!("invalid line for page: {}", queue[index1 + 1]);
                    return None; // Retorna None se a linha for inválida
                }
            }
        }
    }

    // Se a fila for válida, retornamos o valor médio
    let mid_index = (queue.len() as f64 / 2.0).floor() as usize;
    Some(queue[mid_index]) // Retorna o valor do meio
}

pub fn main() {
    let rules_input =
        fs::read_to_string("./src/day5/input.txt").expect("Failed to read the input file");
    let print_input =
        fs::read_to_string("./src/day5/input2.txt").expect("Failed to read the input2 file");

    let rules = rules_input.lines().map(|line| line.to_string()).collect::<Vec<_>>();
    let print_list = print_input.lines().map(|line| line.to_string()).collect::<Vec<_>>();

    let hashed_map = create_rules_hash_map(rules);
    let mut sum = 0;

    for line in print_list {
        let values_arr = extract_print_queue(line);
        if let Some(mid_page) = find_valid_queue(hashed_map.clone(), values_arr) {
            sum += mid_page; // Se for válido, soma o valor do meio
        }
    }

    println!("Sum of middle page numbers: {}", sum);
}
