use std::fs;
use itertools::{Itertools};

pub struct BingoCard{
    card: [[(u8, bool); 5]; 5]
}

impl BingoCard{
    pub fn new() -> BingoCard{
        let card: [[(u8, bool); 5]; 5] = [[(0, false); 5];5];
        BingoCard{card}
    }

    pub fn add_row(&mut self, row: Vec<u8>, pos: usize){
        for i in 0..row.len(){
            self.card[pos][i] = (row[i], false)
        }
    }

    pub fn mark_number(&mut self, number: u8){
        self.card.iter_mut().flatten()
            .for_each(|cell| {if cell.0 == number { cell.1 = true }}
        );
    }

    pub fn is_bingo(&mut self) -> bool{
        let any_row = self.card.iter()
            .any(|row| row.iter().all(|cell| cell.1 == true));
        let any_col = (0..5)
            .into_iter()
            .any(|idx| self.card.iter()
                .map(|row| row[idx]).
                all(|cell| cell.1 == true));
        any_row || any_col
    }

    pub fn sum_unmarked_numbers(&self) -> usize{
        self.card.iter()
            .flatten()
            .filter(|cell| !cell.1)
            .map(|cell | cell.0 as usize)
            .sum()
    }
}


pub fn load_contents(path: &str) -> (Vec<u8>, Vec<BingoCard>) {
     let contents = fs::read_to_string(path)
         .expect("Something wrong with a file");
     let mut lines = contents.lines();
     let numbers: Vec<u8> = lines.next().unwrap().split(',')
         .map(|x| x.parse().unwrap()).collect();
    // println!("{:?}", numbers);
    let mut bingo_cards: Vec<BingoCard> = Vec::new();
    for chunk in lines.filter(|x| !x.is_empty()).chunks(5).into_iter(){
        let mut bingo_card = BingoCard::new();
        for (i, line) in chunk.into_iter().enumerate() {
            let row: Vec<u8> = line.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            bingo_card.add_row(row, i);
            // println!("{:?}", bingo_card.card[i]);
        }
        bingo_cards.push(bingo_card);
        // println!();
    }

    (numbers, bingo_cards)
}

pub fn play_game(numbers:Vec<u8>, mut bingo_cards: Vec<BingoCard>) -> Option<usize>{
    for number in numbers {
        // println!("Playing number {}", number);
        for bingo_card in bingo_cards.iter_mut() {
            bingo_card.mark_number(number);
            if bingo_card.is_bingo() == true {
                // println!("{:?}", bingo_card.card);
                let result = bingo_card.sum_unmarked_numbers() * number as usize;
                return Some(result);
            }
        }
    }
    None
}

pub fn run_part_1(path: &str) -> usize{
    let (numbers, bingo_cards) = load_contents(path);
    let result = play_game(numbers, bingo_cards);
    result.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_part_1(){

        assert_eq!(4512, run_part_1("src/days/day4/input_files/test_file.txt"))
    }
}