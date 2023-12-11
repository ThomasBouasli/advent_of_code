use std::{fs::File, io::Read};


fn main() {
    let mut file = File::open("input.txt").unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let lines = contents.lines();

    let mut sum = 0;

    for line in lines{
        sum += parse_digits(line);
    }

    println!("{}", sum);
}

fn parse_digits(string: &str) -> usize{
    let mut first_digits : String = String::new();
    let mut last_digits : String = String::new();

    for char in string.chars(){
        if char.is_digit(10){
            first_digits.push_str(char.to_string().as_str());
            break
        }
    }

    for char in string.chars().rev(){
        if char.is_digit(10){
            last_digits.push_str(char.to_string().as_str());
            break
        }
    }

    first_digits.push_str(&last_digits);


    return first_digits.parse::<usize>().unwrap();
}