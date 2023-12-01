extern crate utilities;

const STRINGS : [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

// Chunk string into chars, convert chars to digits, take first and last as number
fn number_from_string(s: &str) -> u32 {
    let digits = s.chars().flat_map(|x| x.to_digit(10)).collect::<Vec<u32>>();    
    match (digits.first(), digits.last()) {
        (Some(f), Some(l)) => f * 10 + l,
        _ => 0
    }
}

// Chunk string into chars, take digits as they come, otherwise compare to const string array
// and if value matching is found append that to the digit list. Return first and last as 
// a number
fn number_from_string_with_text(s: &str) -> u32 {
    let c: Vec<char> = s.chars().collect();
    let mut digits: Vec<u32> = Vec::new();
    let mut iter = 0;
    while iter < s.len() {        
        if c[iter] >= '0' && c[iter] <= '9' {
            digits.push(c[iter].to_digit(10).unwrap());
        } else {
            let next_string: String = s.chars().skip(iter).collect();
            for (i, s) in STRINGS.iter().enumerate() {
                if next_string.starts_with(s) {
                    digits.push(i as u32 + 1);
                }
            }
        }
        iter += 1;
    }
    match (digits.first(), digits.last()) {
        (Some(f), Some(l)) => f * 10 + l,
        _ => 0
    }
}

fn main() {    
    let lines = utilities::lines_from_file("input.txt").unwrap();    

    // Part one
    let p1: u32 = lines.iter()
      .map(|s| number_from_string(s))
      .collect::<Vec<u32>>()
      .iter()
      .sum();
    println!("Part one sum: {:?}", p1);

    // Part two
    let p2: u32 = lines.iter()
      .map(|s| number_from_string_with_text(s))
      .collect::<Vec<u32>>()
      .iter()
      .sum();
    println!("Part two sum: {:?}", p2);
}
