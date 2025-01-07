use std::collections::HashMap;
use std::fs;
use std::io;

//use y and x

fn move_guard(input_matrix: &[String]) {
    let mut can_move = true;
    let mut facing_direction = 1;
    let mut guard_position: (u32, u32) = (0, 0); // y, x
    let mut visitedPositions: HashMap<(u32, u32), bool> = HashMap::new();
    let mut last_visitedPosition = (0, 0);

    // Procure pela posição inicial do guarda ('^').
    for (row, line) in input_matrix.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '^' {
                guard_position = (row as u32, col as u32);
                visitedPositions.insert((row as u32, col as u32), true);
                break;
            }
        }
    }


    while can_move {
        let (y, x) = guard_position;
    
        // Corrige facing_direction se ultrapassar o limite
        if facing_direction > 4 {
            facing_direction = 1;
        }
    
        match facing_direction {
            1 => {
                // Movendo para cima
                if y > 0 {
                    if let Some(ch) = input_matrix[y - 1].chars().nth(x) {
                        if ch == '#' {
                            facing_direction += 1;
                        } else {
                            visitedPositions.entry((y as u32, x as u32)).or_insert_with(|| {
                                println!("Adicionando posição: ({}, {})", y, x);
                                true
                            });
                            guard_position = (y - 1, x); // Atualiza posição
                        }
                    }
                } else {
                    facing_direction += 1; // Muda direção se não puder mover
                }
            }
            2 => {
                // Movendo para a direita
                if x + 1 < input_matrix[y].len() {
                    if let Some(ch) = input_matrix[y].chars().nth(x + 1) {
                        if ch == '#' {
                            facing_direction += 1;
                        } else {
                            visitedPositions.entry((y as u32, x as u32)).or_insert_with(|| {
                                println!("Adicionando posição: ({}, {})", y, x);
                                true
                            });
                            guard_position = (y, x + 1); // Atualiza posição
                        }
                    }
                } else {
                    facing_direction += 1; // Muda direção se não puder mover
                }
            }
            3 => {
                // Movendo para baixo
                if y + 1 < input_matrix.len() {
                    if let Some(ch) = input_matrix[y + 1].chars().nth(x) {
                        if ch == '#' {
                            facing_direction += 1;
                        } else {
                            visitedPositions.entry((y as u32, x as u32)).or_insert_with(|| {
                                println!("Adicionando posição: ({}, {})", y, x);
                                true
                            });
                            guard_position = (y + 1, x); // Atualiza posição
                        }
                    }
                } else {
                    facing_direction += 1; // Muda direção se não puder mover
                }
            }
            4 => {
                // Movendo para a esquerda
                if x > 0 {
                    if let Some(ch) = input_matrix[y].chars().nth(x - 1) {
                        if ch == '#' {
                            facing_direction += 1;
                        } else {
                            visitedPositions.entry((y as u32, x as u32)).or_insert_with(|| {
                                println!("Adicionando posição: ({}, {})", y, x);
                                true
                            });
                            guard_position = (y, x - 1); // Atualiza posição
                        }
                    }
                } else {
                    facing_direction += 1; // Muda direção se não puder mover
                }
            }
            _ => {}
        }
    
        // Se todas as direções já foram tentadas, não pode mais se mover
        if facing_direction > 4 {
            can_move = false;
        }
    }

    println!(
        "Posição inicial do guarda: {:?}, Facing direction: {}",
        guard_position, facing_direction
    );

    // O resto da lógica para movimentar o guarda pode ser implementada aqui.
}


fn read_input_and_transform_into_a_matrix() -> io::Result<Vec<String>> {
    // Inicialize o vetor vazio.
    let mut input_matrix: Vec<String> = Vec::new();

    // Leia o conteúdo do arquivo.
    let content = fs::read_to_string("./src/day6/input.txt")?;

    // Itere sobre as linhas e adicione ao vetor.
    for line in content.lines() {
        input_matrix.push(line.to_string());
    }

    // Retorne o vetor.
    Ok(input_matrix)
}


pub fn main(){

}