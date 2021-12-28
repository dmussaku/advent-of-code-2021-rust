use std::fs;

pub fn run_part_1(path: &str) -> i32{

    let numbers: Vec<usize> = fs::read_to_string(path)
        .unwrap()
        .chars()
        .map(|x| x as usize - 0x30)
        .collect();
    println!("{:?}", numbers);
    let mut low_points: Vec<usize> = Vec::new();
    let size = numbers.len() - 1;
    let mut i:usize = 1;
    while i <= size - 1{
        if numbers[i-1] > numbers[i] && numbers[i] < numbers[i + 1]{
            low_points.push(numbers[i]);
        }
        i += 1
    }
    println!("{:?}", low_points);
    1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_part_1(){
        assert_eq!(37, run_part_1("src/days/day9/input_files/test_file.txt"))
    }
}