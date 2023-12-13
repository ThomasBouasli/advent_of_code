use std::{fs::File, io::Read, collections::{HashMap, HashSet}, vec};


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position{
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Value{
    start : Position,
    value : i32
}


impl Position {
    pub fn generate_adjacent(&self) -> Vec<Position> {
        let mut vector = Vec::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let adjacent_position = Position {
                    x: (self.x as isize + dx) as usize,
                    y: (self.y as isize + dy) as usize,
                };
                vector.push(adjacent_position);
            }
        }
        return vector;
    }
}



fn main() {
    let mut file = File::open("input.txt").unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let lines = contents.lines();

    let mut values_map = HashMap::<Position, Value>::new();
    let mut symbols : Vec<Position> = vec![];

    let mut sum = 0;
    let mut list : Vec<Position> = vec![];

    for (y, line) in lines.enumerate(){
        for (x, char) in line.chars().enumerate(){

            if !char.is_digit(10) && char != '.'{
                symbols.push(Position{x,y});
            }

            if sum > 0{
                if !char.is_digit(10){
                    //commit
                    let start = list.first().unwrap().clone();

                    values_map.insert(start.clone(), Value{
                        start,
                        value : sum
                    });

                    sum = 0;
                }else{
                    //append
                    if char.is_digit(10){
                        let position = Position{x,y};
                        list.push(position.clone());
                        sum = sum * 10 + char.to_digit(10).unwrap() as i32;

                        let start = list.first().unwrap().clone();
                        let value = -1;

                        values_map.insert(position, Value{start, value});
                    }   
                }
            }else{
                if char.is_digit(10){
                    //start
                    let position = Position{x,y};
                    list = vec![position];
                    sum = char.to_digit(10).unwrap() as i32;
                }
            }

            if char == '.'{
                continue;
            }
        }
    }


    let mut sum = 0;

    for symbol in symbols{

        let adjacent = symbol.generate_adjacent();

        let mut checked = HashSet::<Position>::new();

        for pos in adjacent{
            if let Some(val) = values_map.get(&pos){
                if let Some(_) = checked.get(&pos){
                    continue;
                }
                if val.value == -1{
                    let start = values_map.get(&val.start).unwrap();

                    if let Some(_) = checked.get(&val.start){
                        continue;
                    }

                    sum += start.value;
                    checked.insert(val.start.clone());
                }else{
                    sum += val.value;
                }

                checked.insert(pos);
            }
        }
    }
    println!("{}", sum);
}
