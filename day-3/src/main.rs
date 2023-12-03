extern crate utilities;
use std::collections::{HashSet, HashMap};

#[derive(Debug)]
struct Schematic {
    data: Vec<Vec<char>>
}

#[derive(Debug)]
struct Part {
    number: u32,
    symbols: HashSet<(char, u32, u32)>
}


impl Schematic {
    fn new(file: &str) -> Self {
        let input = utilities::lines_from_file(file).unwrap();
        Self { data: input.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<_>>() }
    }

    fn get_digits_and_symbols(&self) -> Vec<Part> {
        let mut parts = Vec::new();
        for (y, line) in self.data.iter().enumerate() {
            let mut current_number = Vec::new();
            let mut symbols = HashSet::new();
            for (x, val) in line.iter().enumerate() {
                let y1 = y as i32;
                let x1 = x as i32;
                // Get valid neighbour indices
                let indices = vec![(y1 - 1, x1 - 1), (y1 - 1, x1), (y1 - 1, x1 + 1), 
                                   (y1, x1 - 1), (y1, x1 + 1),
                                   (y1 + 1, x1 - 1), (y1 + 1, x1), (y1 + 1, x1 + 1)];
                let valid = indices.iter()
                  .filter(|(y, x)| *y >= 0 && *y < self.data.len() as i32 && *x >= 0 && *x < line.len() as i32)
                  .collect::<Vec<_>>();

                // Check if neighours contain a symbol
                let new_symbols = valid.iter()
                  .map(|(y, x)| (self.data[*y as usize][*x as usize], *y as u32, *x as u32))
                  .filter(|(s, _, _)| *s != '.' && !(*s >= '0' && *s <= '9'))                  
                  .collect::<Vec<_>>();                                
                
                let mut push = false;

                if *val >= '0' && *val <= '9' {
                    current_number.push(val.to_digit(10).unwrap());                    
                    for symbol in new_symbols {
                        symbols.insert(symbol);
                    }                    
                    if x == line.len() - 1 { push = true; }
                } else if current_number.len() > 0 {
                    push = true;
                }
                
                               
              
                if push {
                    let mut n = 0;
                    for (i, v) in current_number.iter().rev().enumerate() {
                        let t = v * u32::pow(10, i as u32);                                
                        n += t;
                    }
                    if symbols.len() > 0 { parts.push(Part{ number: n, symbols: symbols}); }                        
                    current_number = Vec::new();
                    symbols = HashSet::new();
                }                
            }
        }
        parts
    }

    fn sum_part_numbers(&self) -> u32 {
        let parts = self.get_digits_and_symbols();
        parts.iter()
          .filter(|p| p.symbols.len() > 0)
          .fold(0, |sum, p| sum + p.number)
    }

    fn get_gear_ratio_sum(&self) -> u32 {
        let parts = self.get_digits_and_symbols();
        let mut gears: HashMap<&(char, u32, u32), Vec<u32>> = HashMap::new();
        for part in parts.iter() {
            for symbol in part.symbols.iter() {
                if symbol.0 == '*' {
                    gears.entry(symbol)
                      .and_modify(|p| p.push(part.number))
                      .or_insert_with(|| vec![part.number]);
                }
            }
        }
        gears.iter()
          .filter(|&(k, v)| v.len() == 2)
          .map(|(_, v)| v[0] * v[1])
          .sum()
    }
}






fn main() {    
    let s = Schematic::new("input.txt");     
    println!("Part one: {:?}", s.sum_part_numbers());
    println!("Part two: {:?}", s.get_gear_ratio_sum())
}
