mod days;

use days::{day1, day2, day3, day4, day6, day7};

fn main() {
    let day1_input = day1::day1::read_numbers_from_file("src/days/day1/input_files/file.txt");
    println!("Day1 part 1 result = {}", day1::day1::run_part_1(day1_input));
    println!("Day2 part 1 result = {}", day2::day2::run_part_1("src/days/day2/input_files/file.txt"));
    println!("Day3 part 1 result = {}", day3::day3::run_part_1("src/days/day3/input_files/file.txt"));
    println!("Day4 part 1 result = {}", day4::day4::run_part_1("src/days/day4/input_files/file.txt"));
    println!("Day6 part 1 result = {}", day6::day6::run_part_1("src/days/day6/input_files/file.txt", 80));
    println!("Day7 part 1 result = {}", day7::day7::run_part_1("src/days/day7/input_files/file.txt"));
}
