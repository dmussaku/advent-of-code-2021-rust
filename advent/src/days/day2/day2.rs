use std::fs;
use std::str::FromStr;


enum Command {
    Forward(usize),
    Up(usize),
    Down(usize)
}

impl FromStr for Command{
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut split = line.split_whitespace();
        let command_type = split.next().unwrap();
        let command_units = split.next().unwrap().parse().unwrap();
        match command_type {
            "forward" => Ok(Self::Forward(command_units)),
            "up" => Ok(Self::Up(command_units)),
            "down" => Ok(Self::Down(command_units)),
            _ => Err(())
        }
    }
}


struct Submarine{
    x: usize,
    y: usize
}

impl Submarine{
    pub fn new() -> Self {
        Submarine{
            x: 0,
            y: 0
        }
    }
    pub fn change_position(&mut self, command: Command){
        match command{
            Command::Forward(units) => {
                self.x += units;
            },
            Command::Up(units) => {
                self.y -= units;
            },
            Command::Down(units) => {
                self.y += units;
            }
        }
    }
    pub fn get_position_product(&self) -> usize{
        &self.x * &self.y
    }
}

pub fn run_part_1(path: &str) -> usize {
    let mut submarine = Submarine::new();
    let contents = fs::read_to_string(path)
        .expect("Something went wrong with a file");
    for line in contents.lines(){
        let command = Command::from_str(line).unwrap();
        submarine.change_position(command);
        // println!("Final position x={}, y={}", submarine.x, submarine.y);
    }
    submarine.get_position_product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_part_1(){
        assert_eq!(150, run_part_1("src/days/day2/input_files/test_file.txt"))
    }
}