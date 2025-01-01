use std::fs;
use std::io::{self, BufRead};

fn read_line_and_determin_if_it_is_ok_or_not(line: String) -> bool {
    let max_fall_value = 3;
    let mut isDeacreasing: Option<bool> = None;
    let mut isUnsafe = false;

    let numbers: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("lista de numeros: {:?}", numbers);
    for i in 0..numbers.len() - 1 {
        if numbers[i] < numbers[i + 1] {
            if isDeacreasing == None {
                isDeacreasing = Some(false);
            }

            if isDeacreasing == Some(true) {
                isUnsafe = true;

                return isUnsafe;
            }

            if numbers[i + 1] - numbers[i] > max_fall_value {
                isUnsafe = true;

                return isUnsafe;
            }
        } else if numbers[i] > numbers[i + 1] {
            if isDeacreasing == None {
                isDeacreasing = Some(true);
            }

            if isDeacreasing == Some(false) {
                isUnsafe = true;

                return isUnsafe;
            }

            if numbers[i] - numbers[i + 1] > max_fall_value {
                isUnsafe = true;

                return isUnsafe;
            }
        } else if numbers[i] == numbers[i + 1] {
            isUnsafe = true;

            return isUnsafe;
        }
    }

    return isUnsafe;
}

/**
 * i Knew how to do it but i dont have so much knoledge on Rust syntax and type
 * so i asked chatgpt to do it.
 */
fn is_safe_with_dampener(line: String) -> bool {
    // Primeiro, verifica a sequência original
    if !read_line_and_determin_if_it_is_ok_or_not(line.clone()) {
        return false; // Seguro sem remoção
    }

    // Se não for seguro, tente remover cada elemento
    let numbers: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 0..numbers.len() {
        // Remove o número no índice `i`
        let filtered: Vec<i32> = numbers
            .iter()
            .enumerate()
            .filter(|&(index, _)| index != i)
            .map(|(_, &num)| num)
            .collect();

        // Testa a sequência resultante
        let filtered_line = filtered
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        if !read_line_and_determin_if_it_is_ok_or_not(filtered_line) {
            return false; // Seguro com remoção
        }
    }

    true // Inseguro
}

fn read_input<F>(mut cb: F) -> io::Result<()>
where
    F: FnMut(String) -> bool,
{
    let path = "src/day2/input.txt";
    let mut sumOfSafe = 0;

    let file = fs::File::open(path)?;

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let result = cb(line?);
        if result == false {
            sumOfSafe += 1;
        }
    }

    print!("{}", sumOfSafe);

    Ok(())
}

pub fn main() {
    let _ = read_input(is_safe_with_dampener);
}
