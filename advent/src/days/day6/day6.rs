use itertools::Itertools;
use std::fs;
type FishVec = Vec<Fish>;

trait FishT{
    fn to_string(&self) -> String;
    fn countdown(&mut self);
}

impl FishT for FishVec{
    fn to_string(&self) -> String {
        let result = self.iter()
            .map(|x| x.days.to_string())
            .join(",");
        result
    }

    fn countdown(&mut self){
        let mut new_fish_count: usize = 0;
        for fish in self.iter_mut(){
            if fish.days == 0{
                fish.days = 6;
                new_fish_count += 1
            }
            else{
                fish.days -= 1;
            }
        }
        for _ in 0..new_fish_count{
            self.push(Fish::new(8));
        }
    }
}

struct Fish{
    days: u8
}

impl Fish{
    pub fn new(days: u8) -> Fish{
        Fish{days}
    }
}


pub fn run_part_1(path: &str, days: usize) -> usize{
    let mut fiches: FishVec = fs::read_to_string(path)
        .expect("Something wrong with a file")
        .split(',')
        .map(|x| Fish::new(x.parse::<u8>().expect("Cant parse")))
        .collect();
    // println!("Initial state: {}", fiches.to_string());
    for _i in 1..days + 1{
        fiches.countdown();
        // println!("After {} day: {}", i, fiches.to_string());
    }
    fiches.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_part_1(){
        assert_eq!(26, run_part_1("src/days/day6/input_files/test_file.txt",18))
    }
}
