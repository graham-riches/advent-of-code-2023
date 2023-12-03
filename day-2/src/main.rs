extern crate utilities;

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<(u32, u32, u32)>
}

impl Game {
    fn from_string(s: &str) -> Self {
        let items = s.strip_prefix("Game ")
          .unwrap()
          .split(':')
          .collect::<Vec<&str>>();
        
        let rounds: Vec<_> = items[1].split(';')
          .map(|x| x.split(',')
            .map(|y| {
                let chunks = y.strip_prefix(" ").unwrap().split(' ').collect::<Vec<&str>>();
                let count = chunks[0].parse::<u32>().unwrap();
                match chunks[1] {
                    "red"   => (count, 0, 0),
                    "green" => (0, count, 0),
                    "blue"  => (0, 0, count),
                    _ => (0, 0, 0)
                }})
            .fold((0, 0, 0), |(r, g, b), (r1, g1, b1)| (r + r1, g + g1, b + b1)))
            .collect::<Vec<_>>();                  
        Game{ id: items[0].parse::<u32>().unwrap(), rounds: rounds }
    }

    fn is_valid(&self, counts: (u32, u32, u32)) -> bool {
        self.rounds.iter()
          .all(|(r, g, b)| r <= &counts.0 && g <= &counts.1 && b <= &counts.2)
    }

    fn get_set_power(&self) -> u32 {
        let mins: (u32, u32, u32) = self.rounds.iter()
          .fold((0, 0, 0), |(max_red, max_green, max_blue), (r, g, b)| 
            (if r > &max_red { *r } else { max_red }, 
             if g > &max_green { *g } else { max_green },
             if b > &max_blue { *b } else { max_blue }));
        mins.0 * mins.1 * mins.2
    }
}




fn main() {
    let lines = utilities::lines_from_file("input.txt").unwrap();
    
    // Part one:
    let p1: u32 = lines.iter()
      .map(|s| Game::from_string(s))
      .map(|g| if g.is_valid((12, 13, 14)) { g.id } else { 0 })
      .sum();
    println!("Part one: {:?}", p1);

    // Part two:
    let p2: u32 = lines.iter()
      .map(|s| Game::from_string(s))
      .map(|g| g.get_set_power())
      .sum();
    println!("Part two: {:?}", p2);
}