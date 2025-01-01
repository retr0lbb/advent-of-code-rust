use regex::Regex;
use std::fs;
use std::io::{self, BufRead};


fn get_mult_in_string(search_string: String) -> Vec<String>{
    let regex_string: &str = "(mul[(][0-9]{1,3},[0-9]{1,3}[)])|(don't[(][)])|(do[(][)])";

    let remedy = Regex::new(&regex_string).unwrap();
 

    let memes: Vec<String>= remedy.find_iter(&search_string)
        .map(|m| m.as_str().to_string())
        .collect()
    ;

    return memes;
}

fn get_mul_string_turninto_number(memes_string: &str)-> Vec<i32> {
    let regex_string: &str = "[0-9]{1,3}";
    
    let regex = Regex::new(&regex_string).unwrap();

    let numbers: Vec<i32> = regex.find_iter(memes_string)
            .map(|n| n.as_str().to_string().parse().unwrap()).collect();


    return numbers;
}

fn read_txt() -> io::Result<()> {
    let path = "src/day3/input.txt";

    let file = fs::File::open(path)?;

    let reader = io::BufReader::new(file);
    let mut isOperable = true;
    let mut sum_of_results = 0;

    for line in reader.lines() {
       let foundedMult = get_mult_in_string(line?);

       for multString in foundedMult {
        
        if multString == "don't()" {
            isOperable = false
        }
        if multString == "do()" {
            isOperable = true
        }

        if isOperable == true {
            if (multString != "don't()") && (multString != "do()"){
                let numbers = get_mul_string_turninto_number(&multString);

                let mult_result = numbers[0] * numbers[1];

                sum_of_results += mult_result
            }
        }
       }
    }

    println!("Resposta {}", sum_of_results);
    Ok(())
}


pub fn main(){
    let _ = read_txt();
}