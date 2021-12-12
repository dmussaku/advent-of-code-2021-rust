use std::fs;

pub fn run_part_1(path: &str) -> i32{
    let positions: Vec<i32> = fs::read_to_string(path)
        .expect("Something wrong with a file")
        .split(',')
        .map(|x| x.parse::<i32>().expect("Can't parse"))
        .collect();
    let mut min: i32 = positions.iter()
        .map(|x| (x-positions[0]).abs()).sum();
    for num in positions[1..].iter(){
        let sum: i32 = positions.iter().map(|x| (x-num).abs()).sum();
        if sum < min{
            min = sum;
        }
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_part_1(){
        assert_eq!(37, run_part_1("src/days/day7/input_files/test_file.txt"))
    }
}