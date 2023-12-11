use std::{fs::File, io::Read};

struct Config{
    red: u8,
    green: u8,
    blue: u8
}

impl Config{
    pub fn new(red: u8, green:u8, blue: u8) -> Self{
        return Self{ red, green, blue };
    }

    pub fn get_max(&self, color: &str) -> Result<u8, String>{
        match color {
            "red" => Ok(self.red),
            "green" => Ok(self.green),
            "blue" => Ok(self.blue),
            _ => Err(String::from("Error"))
        }
    }
}


fn main() {
    let config = Config::new(12,13,14);

    let mut file = File::open("input.txt").unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let lines = contents.lines();

    let mut sum = 0;

    for (i, line) in lines.enumerate(){
        sum += parse(i+1, line, &config);
    }

    println!("{}", sum);
}

fn parse(index: usize, string: &str, config: &Config) -> usize{
   let parsed = string.replace(format!("Game {}:", &index).as_str(), "");

   let splitted: Vec<&str> = parsed.split(|c| c == ',' || c == ';').collect();

   for part in splitted{
    let mut parts = part.split_whitespace();

    let number = parts.next().unwrap().parse::<u8>().unwrap();
    let color = parts.next().unwrap();

    let max = config.get_max(color).unwrap();

    if number > max{
        return 0;
    }
   }

   return index;
}