use std::fs;


const RADIX: u32 = 10;


struct BitSum{
    bits: Vec<u32>,
    size: u32
}

impl BitSum {
    pub fn new(width: usize) -> Self{
        let bits: Vec<u32> = (0..width).map(|_x|0).collect();
        BitSum{ bits, size: 0}
    }

    pub fn add_bits(&mut self, bits: Vec<u32>){
        for i in 0..bits.len(){
            self.bits[i] += bits[i];
        }
        self.size += 1;
    }

    pub fn calculate_gamma_rate(&mut self) -> isize{
        let mut rate= String::new();
        for i in 0..self.bits.len(){
            if self.bits[i] > self.size / 2{
                rate.push('1');
            }
            else {rate.push('0')};
        }
        isize::from_str_radix(&rate, 2).unwrap()
    }

    pub fn calculate_epslion_rate(&mut self) -> isize{
        let mut rate= String::new();
        for i in 0..self.bits.len(){
            if self.bits[i] < self.size / 2{
                rate.push('1');
            }
            else {rate.push('0')};
        }
        isize::from_str_radix(&rate, 2).unwrap()
    }
}

pub fn run_part_1(path: &str) -> isize{
    let contents = fs::read_to_string(path)
        .expect("Something wrong with the file");
    let width = contents.lines().next().unwrap().len();
    let mut bit_sum = BitSum::new(width);
    for line in contents.lines(){
        let bits: Vec<u32> = line.chars()
            .map(|val| val.to_digit(RADIX).unwrap())
            .collect();
        bit_sum.add_bits(bits);
    }
    let gamma_rate = bit_sum.calculate_gamma_rate();
    let epsilon_rate = bit_sum.calculate_epslion_rate();
    gamma_rate * epsilon_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_part_1(){
        assert_eq!(198, run_part_1("src/days/day3/input_files/test_file.txt"))
    }
}