use std::fs;
use std::collections::HashMap;

type SpecCharMap = HashMap<char, i32>;

trait SpecChar{
    fn reset(&mut self);
    fn calculate(&mut self, line: &str) -> i32;
}

impl SpecChar for SpecCharMap{
    fn reset(&mut self){
        self.insert('(', 0);
        self.insert(')', 0);
        self.insert('[', 0);
        self.insert(']', 0);
        self.insert('{', 0);
        self.insert('}', 0);
        self.insert('<', 0);
        self.insert('>', 0);
    }

    fn calculate(&mut self, line: &str) -> i32{
        for char in line.chars(){
            self.insert(char, 1 + self[&char]);
        }
        println!("{:?}", self);


        let result = (self[&'('] - self[&')']).abs() * 3
            + (self[&'['] - self[&']']).abs() * 57
            + (self[&'{'] - self[&'}']).abs() * 1197
            + (self[&'<'] - self[&'>']).abs() * 25137;

        self.reset();
        result
    }
}

pub fn run_part_1(path: &str) -> i32{
    let contents = fs::read_to_string(path)
        .expect("Something went wrong with a file");

    let mut char_map: SpecCharMap = SpecCharMap::from([
            ('(', 0), (')', 0), ('[', 0), (']', 0), ('{', 0), ('}', 0), ('<', 0), ('>', 0)
        ]);
    for line in contents.lines(){
        let result = char_map.calculate(line);
        println!("{}, result = {}", line, result);
    }
    1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_part_1(){
        assert_eq!(26397, run_part_1("src/days/day10/input_files/test_file.txt"))
    }
}