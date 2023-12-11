use std::{fs::File, io::Read, collections::HashMap};



fn main() {
    let mut file = File::open("input.txt").unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let lines = contents.lines();

    let mut sum = 0;

    for (i, line) in lines.enumerate(){
        sum += parse(i+1, line);
    }

    println!("{}", sum);
}




fn parse(index: usize, string: &str) -> usize{
    let parsed = string.replace(format!("Game {}:", &index).as_str(), "");
 
    let splitted: Vec<&str> = parsed.split(|c| c == ',' || c == ';').collect();

    let mut map = HashMap::<&str, usize>::new(); 
 
    for part in splitted{
        let mut parts = part.split_whitespace();
    
        let number = parts.next().unwrap().parse::<usize>().unwrap();
        let color = parts.next().unwrap();


        if let Some(current) = map.get(color){
            if &number > current{
                map.insert(color, number);
            }
        }else{
            map.insert(color, number);
        }
    }

    let red = map.get("red").unwrap();
    let green = map.get("green").unwrap();
    let blue = map.get("blue").unwrap();
 
    return red * green * blue;
 }
