extern crate utilities;

#[derive(Debug, Clone)]
struct ScratchGame {    
    winning: Vec<u32>,
    owned: Vec<u32>,
    count: u32
}

impl ScratchGame {
    fn from_file(s: &str) -> Vec<Self> {
        utilities::lines_from_file(s).unwrap()
          .iter()
          .map(|l| ScratchGame::from_string(l))
          .collect::<Vec<_>>()
    }

    fn from_string(s: &str) -> Self {
        let parts = s.strip_prefix("Card ").unwrap().split(":").collect::<Vec<&str>>();
        let numbers = parts[1].split(" | ")
          .map(|x| utilities::parse_delimited_string(x, " ").unwrap())
          .collect::<Vec<_>>();                
        Self {             
            winning: numbers[0].clone(), 
            owned: numbers[1].clone(),
            count: 1}
    }

    fn count_matches(&self) -> u32 {
        self.owned.iter()
          .map(|x| self.winning.iter().fold(0, |sum, y| sum + ((*y == *x) as u32)))
          .sum()
    }

    fn get_score(&self) -> u32 {
        let matches = self.count_matches();
        
        if matches == 0 {
            0
        } else {
            u32::pow(2, matches - 1)
        }           
    }    
}

fn get_total_cards(cards: &Vec<ScratchGame>) -> u32 {
    let mut deck: Vec<ScratchGame> = cards.to_vec();        
    for i in 0..deck.len() {            
        let wins = deck[i].count_matches();        
        for j in 0..wins {
            let new_index = i + (j as usize) + 1;
            if new_index < deck.len() {
                deck[new_index].count += deck[i].count; 
            }                   
        }                    
    }
    
    deck.iter().fold(0, |sum, x| sum + x.count)
}



fn main() {
    let game = ScratchGame::from_file("input.txt");
    let p1: u32 = game.iter()
      .map(|g| g.get_score())
      .sum();
    println!("Part one {:?}", p1);

    let p2 = get_total_cards(&game);
    println!("Part two {:?}", p2);    
}
